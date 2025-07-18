use ::libc;
extern "C" {
    pub type __dirstream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn Log(logLevel: LogLevel, message: *const libc::c_char, _: ...);
    fn DiagTrace(message: *const libc::c_char, _: ...);
}
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type pid_t = __pid_t;
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
pub type LogLevel = libc::c_uint;
pub const error: LogLevel = 4;
pub const crit: LogLevel = 3;
pub const warn: LogLevel = 2;
pub const info: LogLevel = 1;
pub const debug: LogLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProcessStat {
    pub pid: pid_t,
    pub comm: *mut libc::c_char,
    pub state: libc::c_char,
    pub ppid: pid_t,
    pub pgrp: gid_t,
    pub session: libc::c_int,
    pub tty_nr: libc::c_int,
    pub tpgid: gid_t,
    pub flags: libc::c_uint,
    pub minflt: libc::c_ulong,
    pub cminflt: libc::c_ulong,
    pub majflt: libc::c_ulong,
    pub cmajflt: libc::c_ulong,
    pub utime: libc::c_ulong,
    pub stime: libc::c_ulong,
    pub cutime: libc::c_ulong,
    pub cstime: libc::c_ulong,
    pub priority: libc::c_long,
    pub nice: libc::c_long,
    pub num_threads: libc::c_long,
    pub itrealvalue: libc::c_long,
    pub starttime: libc::c_ulonglong,
    pub vsize: libc::c_ulong,
    pub rss: libc::c_long,
    pub rsslim: libc::c_ulong,
    pub startcode: libc::c_ulong,
    pub endcode: libc::c_ulong,
    pub startstack: libc::c_ulong,
    pub kstkesp: libc::c_ulong,
    pub kstkeip: libc::c_ulong,
    pub signal: libc::c_ulong,
    pub blocked: libc::c_ulong,
    pub sigignore: libc::c_ulong,
    pub sigcatch: libc::c_ulong,
    pub wchan: libc::c_ulong,
    pub nswap: libc::c_ulong,
    pub cnswap: libc::c_ulong,
    pub exit_signal: libc::c_int,
    pub processor: libc::c_int,
    pub rt_priority: libc::c_uint,
    pub policy: libc::c_uint,
    pub delayacct_blkio_ticks: libc::c_ulonglong,
    pub guest_time: libc::c_ulong,
    pub cguest_time: libc::c_long,
    pub start_data: libc::c_ulong,
    pub end_data: libc::c_ulong,
    pub start_brk: libc::c_ulong,
    pub arg_start: libc::c_ulong,
    pub arg_end: libc::c_ulong,
    pub env_start: libc::c_ulong,
    pub env_end: libc::c_ulong,
    pub exit_code: libc::c_int,
    pub num_filedescriptors: libc::c_int,
}
pub unsafe extern "C" fn GetProcessStat(
    mut pid: pid_t,
    mut proc_0: *mut ProcessStat,
) -> bool {
    let mut procFilePath: [libc::c_char; 32] = [0; 32];
    let mut fileBuffer: [libc::c_char; 1024] = [0; 1024];
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut savePtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut procFile: *mut FILE = 0 as *mut FILE;
    let mut fddir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    if sprintf(
        procFilePath.as_mut_ptr(),
        b"/proc/%d/fdinfo\0" as *const u8 as *const libc::c_char,
        pid,
    ) < 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    fddir = opendir(procFilePath.as_mut_ptr());
    if !fddir.is_null() {
        (*proc_0).num_filedescriptors = 0 as libc::c_int;
        loop {
            entry = readdir(fddir);
            if entry.is_null() {
                break;
            }
            (*proc_0).num_filedescriptors += 1;
            (*proc_0).num_filedescriptors;
        }
        closedir(fddir);
    } else {
        Log(
            error,
            b"Failed to open %s. Exiting...\n\0" as *const u8 as *const libc::c_char,
            procFilePath.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).num_filedescriptors -= 2 as libc::c_int;
    if sprintf(
        procFilePath.as_mut_ptr(),
        b"/proc/%d/stat\0" as *const u8 as *const libc::c_char,
        pid,
    ) < 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    procFile = fopen(
        procFilePath.as_mut_ptr(),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !procFile.is_null() {
        if (fgets(
            fileBuffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            procFile,
        ))
            .is_null()
        {
            Log(
                error,
                b"Failed to read from %s. Exiting...\n\0" as *const u8
                    as *const libc::c_char,
                procFilePath.as_mut_ptr(),
            );
            fclose(procFile);
            return 0 as libc::c_int != 0;
        }
        fclose(procFile);
    } else {
        Log(
            error,
            b"Failed to open %s.\n\0" as *const u8 as *const libc::c_char,
            procFilePath.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).pid = atoi(fileBuffer.as_mut_ptr());
    savePtr = strrchr(fileBuffer.as_mut_ptr(), ')' as i32);
    if !savePtr.is_null() {
        savePtr = savePtr.offset(2 as libc::c_int as isize);
        (*proc_0)
            .state = *(strtok_r(
            savePtr,
            b" \0" as *const u8 as *const libc::c_char,
            &mut savePtr,
        ))
            .offset(0 as libc::c_int as isize);
    }
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Parent PID. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 84\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .ppid = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int) as pid_t;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Process group ID. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 93\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .pgrp = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int) as gid_t;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Session ID. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 102\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .session = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_int;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Controlling terminal. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 111\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .tty_nr = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_int;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Foreground group ID. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 120\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .tpgid = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int) as gid_t;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Kernel flags. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 129\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .flags = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_uint;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Minflt. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 138\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).minflt = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cminflt. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 147\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).cminflt = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - majflt. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 156\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).majflt = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cmajflt. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 165\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).cmajflt = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - utime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 174\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).utime = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - stime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 183\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).stime = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cutime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 192\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).cutime = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cstime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 201\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).cstime = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - priority. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 210\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).priority = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - nice. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 219\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).nice = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - num_threads. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 228\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .num_threads = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - itrealvalue. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 237\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .itrealvalue = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - starttime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 246\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .starttime = strtoull(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - vsize. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 255\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).vsize = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - rss. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 264\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).rss = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - rsslim. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 273\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).rsslim = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - startcode. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 282\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).startcode = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - endcode. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 291\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).endcode = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - startstack. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 300\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .startstack = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - kstkesp. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 309\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).kstkesp = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - kstkeip. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 318\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).kstkeip = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - signal. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 327\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).signal = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - blocked. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 336\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).blocked = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - sigignore. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 345\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).sigignore = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - sigcatch. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 354\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).sigcatch = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - wchan. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 363\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).wchan = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - nswap. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 372\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).nswap = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cnswap. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 381\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).cnswap = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - exit_signal. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 390\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .exit_signal = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_int;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - processor. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 399\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .processor = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_int;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - rt_priority. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 408\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .rt_priority = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_uint;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - policy. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 417\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .policy = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_uint;
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - delayacct_blkio_ticks. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 426\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .delayacct_blkio_ticks = strtoull(
        token,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - guest_time. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 435\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .guest_time = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cguest_time. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 444\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .cguest_time = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - start_data. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 453\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .start_data = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - end_data. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 462\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).end_data = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - start_brk. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 471\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).end_data = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - arg_start. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 480\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).arg_start = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - arg_end. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 489\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).arg_end = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - env_start. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 498\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).env_start = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - env_end. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 507\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).env_end = strtoul(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    token = strtok_r(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr,
    );
    if token.is_null() {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - exit_code. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 516\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .exit_code = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as libc::c_int;
    return 1 as libc::c_int != 0;
}
