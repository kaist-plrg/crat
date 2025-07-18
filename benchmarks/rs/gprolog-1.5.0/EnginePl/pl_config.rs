use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __int32_t = libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UsedMachRegInf {
    pub mach_reg_name: *mut libc::c_char,
    pub pl_reg_name: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RegInf {
    pub type_0: [libc::c_char; 32],
    pub name: [libc::c_char; 32],
}
pub type TypTag = libc::c_uint;
pub const ADDRESS: TypTag = 2;
pub const LONG_INT: TypTag = 1;
pub const SHORT_UNS: TypTag = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TagInf {
    pub name: [libc::c_char; 32],
    pub type_0: TypTag,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackInf {
    pub name: [libc::c_char; 32],
    pub desc: [libc::c_char; 64],
    pub def_size: libc::c_int,
    pub top_macro: [libc::c_char; 128],
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut used_mach_reg: [UsedMachRegInf; 256] = [UsedMachRegInf {
    mach_reg_name: 0 as *const libc::c_char as *mut libc::c_char,
    pl_reg_name: 0 as *const libc::c_char as *mut libc::c_char,
    type_0: 0 as *const libc::c_char as *mut libc::c_char,
}; 256];
pub static mut nb_of_used_mach_regs: libc::c_int = 0;
pub static mut save_str: [libc::c_char; 512] = [0; 512];
pub static mut fw_r: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut fw_s: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut fg_c: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    if *(b"x86_64\0" as *const u8 as *const libc::c_char) as libc::c_int == '?' as i32 {
        printf(
            b"*** This architecture is not supported ***\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if (8 as libc::c_int * 8 as libc::c_int) as libc::c_ulong
        != (::std::mem::size_of::<PlLong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        printf(
            b"Bad Value of WORD_SIZE - should be %d\n\0" as *const u8
                as *const libc::c_char,
            (::std::mem::size_of::<PlLong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    fw_r = fopen(
        b"wam_regs.h\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if fw_r.is_null() {
        Pl_Fatal_Error(
            b"cannot open %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"wam_regs.h\0" as *const u8 as *const libc::c_char,
        );
    }
    fprintf(
        fw_r,
        b"/* this file is automatically generated by pl_config.c */\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        fw_r,
        b"\n#include \"gp_config.h\"\n\n\0" as *const u8 as *const libc::c_char,
    );
    fw_s = fopen(
        b"wam_stacks.h\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if fw_s.is_null() {
        Pl_Fatal_Error(
            b"cannot open %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"wam_stacks.h\0" as *const u8 as *const libc::c_char,
        );
    }
    fprintf(
        fw_s,
        b"/* this file is automatically generated by pl_config.c */\n\0" as *const u8
            as *const libc::c_char,
    );
    fg_c = fopen(
        b"gprolog_cst.h\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if fg_c.is_null() {
        Pl_Fatal_Error(
            b"cannot open %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gprolog_cst.h\0" as *const u8 as *const libc::c_char,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"\t-------------------------------\n\0" as *const u8 as *const libc::c_char);
    printf(b"\t--- GNU PROLOG INSTALLATION ---\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"\t-------------------------------\n\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"GNU Prolog version: %s (%s)\n\0" as *const u8 as *const libc::c_char,
        b"1.5.0\0" as *const u8 as *const libc::c_char,
        b"Jul 11 2025\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Operating system  : %s\n\0" as *const u8 as *const libc::c_char,
        b"linux-gnu\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Processor         : %s\n\0" as *const u8 as *const libc::c_char,
        b"x86_64\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Size of a WAM word: %d bits\n\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int * 8 as libc::c_int,
    );
    printf(
        b"C compiler        : %s\n\0" as *const u8 as *const libc::c_char,
        b"gcc\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"C flags           : %s\n\0" as *const u8 as *const libc::c_char,
        b"-O3 -fomit-frame-pointer -Wno-char-subscripts\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"C flags machine   : %s\n\0" as *const u8 as *const libc::c_char,
        b" -fno-strict-aliasing -fcommon\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Assembler         : %s\n\0" as *const u8 as *const libc::c_char,
        b"as\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Assembler flags   : %s\n\0" as *const u8 as *const libc::c_char,
        b"--64\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Loader flags      : %s\n\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Loader libraries  : %s\n\0" as *const u8 as *const libc::c_char,
        b"-lm\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Use line editor   : %s\n\0" as *const u8 as *const libc::c_char,
        b"Yes\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Use piped consult : %s\n\0" as *const u8 as *const libc::c_char,
        b"Yes\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Use sockets       : %s\n\0" as *const u8 as *const libc::c_char,
        b"Yes\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Use FD solver     : %s\n\0" as *const u8 as *const libc::c_char,
        b"Yes\0" as *const u8 as *const libc::c_char,
    );
    Write_GProlog_Cst();
    Generate_Archi();
    Write_C_Compiler_Info();
    printf(
        b"Use machine regs. : Yes (%d used)\n\0" as *const u8 as *const libc::c_char,
        nb_of_used_mach_regs,
    );
    fclose(fw_r);
    fclose(fw_s);
    fclose(fg_c);
    printf(b"Used register(s)  : \0" as *const u8 as *const libc::c_char);
    if nb_of_used_mach_regs != 0 {
        i = 0 as libc::c_int;
        while i < nb_of_used_mach_regs {
            printf(
                b"%s (%s)  \0" as *const u8 as *const libc::c_char,
                used_mach_reg[i as usize].mach_reg_name,
                used_mach_reg[i as usize].pl_reg_name,
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"none\n\0" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"\t------------------------------\n\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Write_GProlog_Cst() {
    let mut major: libc::c_int = 0;
    let mut minor: libc::c_int = 0;
    let mut patch_level: libc::c_int = 0;
    sscanf(
        b"1.5.0\0" as *const u8 as *const libc::c_char,
        b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
        &mut major as *mut libc::c_int,
        &mut minor as *mut libc::c_int,
        &mut patch_level as *mut libc::c_int,
    );
    fputc('\n' as i32, fg_c);
    fprintf(
        fg_c,
        b"#define __GNU_PROLOG__        \t%d\n\0" as *const u8 as *const libc::c_char,
        major,
    );
    fprintf(
        fg_c,
        b"#define __GPROLOG__           \t%d\n\0" as *const u8 as *const libc::c_char,
        major,
    );
    fprintf(
        fg_c,
        b"#define __GPROLOG_MINOR__     \t%d\n\0" as *const u8 as *const libc::c_char,
        minor,
    );
    fprintf(
        fg_c,
        b"#define __GPROLOG_PATCHLEVEL__\t%d\n\0" as *const u8 as *const libc::c_char,
        patch_level,
    );
    fprintf(
        fg_c,
        b"#define __GPROLOG_VERSION__   \t%d\n\0" as *const u8 as *const libc::c_char,
        major * 10000 as libc::c_int + minor * 100 as libc::c_int + patch_level,
    );
    fputc('\n' as i32, fg_c);
    fprintf(
        fg_c,
        b"#define PROLOG_DIALECT  \t\"gprolog\"\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fg_c,
        b"#define PROLOG_NAME     \t\"GNU Prolog\"\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        fg_c,
        b"#define PROLOG_VERSION  \t\"1.5.0\"\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fg_c,
        b"#define PROLOG_DATE     \t\"Jul 11 2025\"\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        fg_c,
        b"#define PROLOG_COPYRIGHT\t\"Copyright (C) 1999-2025 Daniel Diaz\"\n\0"
            as *const u8 as *const libc::c_char,
    );
    fputc('\n' as i32, fg_c);
}
pub unsafe extern "C" fn Write_C_Compiler_Info() {
    let mut i: libc::c_int = 0;
    fprintf(
        fw_r,
        b"\n#define NB_USED_MACHINE_REGS\t%d\n\n\0" as *const u8 as *const libc::c_char,
        nb_of_used_mach_regs,
    );
    fprintf(fw_r, b"#define CFLAGS_REGS\t\t\"\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_of_used_mach_regs {
        fprintf(
            fw_r,
            b"-ffixed-%s\0" as *const u8 as *const libc::c_char,
            used_mach_reg[i as usize].mach_reg_name,
        );
        fputc(' ' as i32, fw_r);
        i += 1;
        i;
    }
    fputs(b"\"\n\0" as *const u8 as *const libc::c_char, fw_r);
}
pub unsafe extern "C" fn Generate_Archi() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut g: *mut FILE = 0 as *mut FILE;
    static mut str: [libc::c_char; 512] = [0; 512];
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    f = fopen(
        b"wam_archi.def\0" as *const u8 as *const libc::c_char,
        b"rt\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        Pl_Fatal_Error(
            b"cannot open %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"wam_archi.def\0" as *const u8 as *const libc::c_char,
        );
    }
    g = fopen(
        b"wam_archi.h\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if g.is_null() {
        Pl_Fatal_Error(
            b"cannot open %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"wam_archi.h\0" as *const u8 as *const libc::c_char,
        );
    }
    while feof(f) == 0
        && !(fgets(
            str.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
            f,
        ))
            .is_null()
    {
        if *str.as_mut_ptr() as libc::c_int != '@' as i32 {
            fputs(str.as_mut_ptr(), g);
        } else {
            strcpy(save_str.as_mut_ptr(), str.as_mut_ptr());
            p1 = Read_Identifier(
                str.as_mut_ptr().offset(1 as libc::c_int as isize),
                1 as libc::c_int,
                &mut p2,
            );
            if strcmp(p1, b"begin\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            {
                Pl_Fatal_Error(
                    b"Syntax error: incorrect @ declaration in: %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    save_str.as_mut_ptr(),
                );
            }
            p1 = Read_Identifier(
                p2.offset(1 as libc::c_int as isize),
                1 as libc::c_int,
                &mut p2,
            );
            if strcmp(p1, b"regs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                Generate_Regs(f, g);
            } else if strcmp(p1, b"tags\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                Generate_Tags(f, g);
            } else if strcmp(p1, b"stacks\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                Generate_Stacks(f, g);
            } else {
                Pl_Fatal_Error(
                    b"Syntax error: unknown section in: %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    save_str.as_mut_ptr(),
                );
            }
        }
    }
    fclose(f);
    fclose(g);
}
pub unsafe extern "C" fn Read_Identifier(
    mut s: *mut libc::c_char,
    mut fail_if_error: libc::c_int,
    mut end: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    *end = s;
    if *(*__ctype_b_loc()).offset(**end as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        if fail_if_error != 0 {
            Pl_Fatal_Error(
                b"Syntax error: identifier expected in: %s\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                save_str.as_mut_ptr(),
            );
        } else {
            return 0 as *mut libc::c_char
        }
    }
    loop {
        *end = (*end).offset(1);
        *end;
        if !(*(*__ctype_b_loc()).offset(**end as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            || **end as libc::c_int == '_' as i32)
        {
            break;
        }
    }
    if *(*__ctype_b_loc()).offset(**end as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        Pl_Fatal_Error(
            b"Syntax error: space expected after identifier in: %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            save_str.as_mut_ptr(),
        );
    }
    **end = '\0' as i32 as libc::c_char;
    return s;
}
pub unsafe extern "C" fn Read_String(
    mut s: *mut libc::c_char,
    mut end: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int != '"' as i32 {
        Pl_Fatal_Error(
            b"Syntax error: string expected in: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            save_str.as_mut_ptr(),
        );
    }
    *end = s.offset(1 as libc::c_int as isize);
    while **end as libc::c_int != '"' as i32 {
        if **end as libc::c_int == '\0' as i32 {
            Pl_Fatal_Error(
                b"Syntax error: string expected in: %s\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                save_str.as_mut_ptr(),
            );
        }
        *end = (*end).offset(1);
        *end;
    }
    *end = (*end).offset(1);
    *end;
    **end = '\0' as i32 as libc::c_char;
    return s;
}
pub unsafe extern "C" fn Pl_Read_Integer(
    mut s: *mut libc::c_char,
    mut end: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    *end = s;
    if *(*__ctype_b_loc()).offset(**end as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        Pl_Fatal_Error(
            b"Syntax error: integer expected in: %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            save_str.as_mut_ptr(),
        );
    }
    loop {
        x = x * 10 as libc::c_int + **end as libc::c_int - '0' as i32;
        *end = (*end).offset(1);
        *end;
        if !(*(*__ctype_b_loc()).offset(**end as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            || **end as libc::c_int == '_' as i32)
        {
            break;
        }
    }
    if *(*__ctype_b_loc()).offset(**end as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        Pl_Fatal_Error(
            b"Syntax error: space expected after identifier in: %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            save_str.as_mut_ptr(),
        );
    }
    **end = '\0' as i32 as libc::c_char;
    return x;
}
pub unsafe extern "C" fn Generate_Regs(mut f: *mut FILE, mut g: *mut FILE) {
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut str: [libc::c_char; 512] = [0; 512];
    let mut str_base: [libc::c_char; 32] = *::std::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut used_regs: [*mut libc::c_char; 1] = [0 as *mut libc::c_char];
    let mut p: *mut *mut libc::c_char = used_regs.as_mut_ptr();
    let mut reg: [[RegInf; 50]; 15] = [[RegInf {
        type_0: [0; 32],
        name: [0; 32],
    }; 50]; 15];
    let mut nb_reg: [libc::c_int; 15] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut dp: *mut RegInf = 0 as *mut RegInf;
    let mut total_nb_reg: libc::c_int = 0 as libc::c_int;
    let mut nb_not_alloc: libc::c_int = 0 as libc::c_int;
    let mut regs_to_save_for_signal: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    loop {
        if feof(f) != 0
            || (fgets(
                str.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                    as libc::c_int,
                f,
            ))
                .is_null()
        {
            Pl_Fatal_Error(
                b"Syntax error: end expected for @begin reg\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if *str.as_mut_ptr() as libc::c_int != '@' as i32 {
            if *str.as_mut_ptr() as libc::c_int != '\n' as i32 {
                fputs(str.as_mut_ptr(), g);
            }
        } else {
            strcpy(save_str.as_mut_ptr(), str.as_mut_ptr());
            p1 = Read_Identifier(
                str.as_mut_ptr().offset(1 as libc::c_int as isize),
                1 as libc::c_int,
                &mut p2,
            );
            if strcmp(p1, b"end\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                break;
            }
            if strcmp(p1, b"filler\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                p1 = Read_Identifier(
                    p2.offset(1 as libc::c_int as isize),
                    0 as libc::c_int,
                    &mut p2,
                );
                if p1.is_null() {
                    i = Pl_Read_Integer(p2, &mut p2);
                    p1 = str.as_mut_ptr();
                    sprintf(p1, b"%d\0" as *const u8 as *const libc::c_char, i);
                }
                sprintf(
                    str_base.as_mut_ptr().offset(strlen(str_base.as_mut_ptr()) as isize),
                    b"%s+\0" as *const u8 as *const libc::c_char,
                    p1,
                );
            } else if strcmp(p1, b"reg\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                i = Pl_Read_Integer(p2.offset(1 as libc::c_int as isize), &mut p2);
                strcpy(
                    (reg[i as usize][nb_reg[i as usize] as usize].type_0).as_mut_ptr(),
                    Read_Identifier(
                        p2.offset(1 as libc::c_int as isize),
                        1 as libc::c_int,
                        &mut p2,
                    ),
                );
                strcpy(
                    (reg[i as usize][nb_reg[i as usize] as usize].name).as_mut_ptr(),
                    Read_Identifier(
                        p2.offset(1 as libc::c_int as isize),
                        1 as libc::c_int,
                        &mut p2,
                    ),
                );
                nb_reg[i as usize] += 1;
                nb_reg[i as usize];
            } else {
                Pl_Fatal_Error(
                    b"Syntax error: incorrect @ declaration in: %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    save_str.as_mut_ptr(),
                );
            }
        }
    }
    fprintf(
        g,
        b"\n\n   /*--- Begin Register Generation ---*/\n\n\0" as *const u8
            as *const libc::c_char,
    );
    if !(*p).is_null() {
        used_mach_reg[nb_of_used_mach_regs as usize].mach_reg_name = *p;
        used_mach_reg[nb_of_used_mach_regs as usize]
            .pl_reg_name = b"pl_reg_bank\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        used_mach_reg[nb_of_used_mach_regs as usize]
            .type_0 = b"WamWordP\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        nb_of_used_mach_regs += 1;
        nb_of_used_mach_regs;
        fprintf(
            g,
            b"register WamWord \t\t*pl_reg_bank asm (\"%s\");\n\n\0" as *const u8
                as *const libc::c_char,
            *p,
        );
        fprintf(
            fw_r,
            b"#define MAP_REG_BANK\t\t\"%s\"\n\n\0" as *const u8 as *const libc::c_char,
            *p,
        );
        p = p.offset(1);
        p;
    } else {
        fprintf(g, b"#ifdef ENGINE_FILE\n\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            g,
            b"       WamWord \t\t\t*pl_reg_bank;\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(g, b"\n#else\n\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            g,
            b"extern WamWord \t\t\t*pl_reg_bank;\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(g, b"\n#endif\n\n\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        total_nb_reg += nb_reg[i as usize];
        while j < nb_reg[i as usize] {
            dp = &mut *(*reg.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize) as *mut RegInf;
            if !(*p).is_null() {
                used_mach_reg[nb_of_used_mach_regs as usize].mach_reg_name = *p;
                used_mach_reg[nb_of_used_mach_regs as usize]
                    .pl_reg_name = ((*dp).name).as_mut_ptr();
                used_mach_reg[nb_of_used_mach_regs as usize]
                    .type_0 = ((*dp).type_0).as_mut_ptr();
                nb_of_used_mach_regs += 1;
                nb_of_used_mach_regs;
                fprintf(
                    g,
                    b"register %s\t\t%-3s asm (\"%s\");\n\0" as *const u8
                        as *const libc::c_char,
                    ((*dp).type_0).as_mut_ptr(),
                    ((*dp).name).as_mut_ptr(),
                    *p,
                );
                let fresh0 = p;
                p = p.offset(1);
                fprintf(
                    fw_r,
                    b"#define MAP_REG_%-10s\t\"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    ((*dp).name).as_mut_ptr(),
                    *fresh0,
                );
                if (*p).is_null() {
                    fprintf(g, b"\n\n\0" as *const u8 as *const libc::c_char);
                }
            } else {
                fprintf(
                    g,
                    b"#define %s\t\t\t(((%-8s *) pl_reg_bank)[%s%d])\n\0" as *const u8
                        as *const libc::c_char,
                    ((*dp).name).as_mut_ptr(),
                    ((*dp).type_0).as_mut_ptr(),
                    str_base.as_mut_ptr(),
                    nb_not_alloc,
                );
                let fresh1 = nb_not_alloc;
                nb_not_alloc = nb_not_alloc + 1;
                fprintf(
                    fw_r,
                    b"#define MAP_OFFSET_%-6s\t((%s%d)*%d)\n\0" as *const u8
                        as *const libc::c_char,
                    ((*dp).name).as_mut_ptr(),
                    str_base.as_mut_ptr(),
                    fresh1,
                    ::std::mem::size_of::<PlLong>() as libc::c_ulong as libc::c_int,
                );
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fprintf(g, b"\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"#define NB_OF_REGS          \t%d\n\0" as *const u8 as *const libc::c_char,
        total_nb_reg,
    );
    fprintf(
        g,
        b"#define NB_OF_ALLOC_REGS    \t%d\n\0" as *const u8 as *const libc::c_char,
        total_nb_reg - nb_not_alloc,
    );
    fprintf(
        g,
        b"#define NB_OF_NOT_ALLOC_REGS\t%d\n\0" as *const u8 as *const libc::c_char,
        nb_not_alloc,
    );
    fprintf(
        g,
        b"#define REG_BANK_SIZE       \t(%sNB_OF_NOT_ALLOC_REGS)\n\0" as *const u8
            as *const libc::c_char,
        str_base.as_mut_ptr(),
    );
    fprintf(
        g,
        b"\n\n\n\n#define NB_OF_USED_MACHINE_REGS %d\n\0" as *const u8
            as *const libc::c_char,
        nb_of_used_mach_regs,
    );
    regs_to_save_for_signal = (nb_of_used_mach_regs > 1 as libc::c_int) as libc::c_int;
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"#ifdef ENGINE_FILE\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"WamWord *save_reg_bank;\n\n\0" as *const u8 as *const libc::c_char);
    if regs_to_save_for_signal != 0 {
        fprintf(
            g,
            b"WamWord pl_buff_signal_reg[NB_OF_USED_MACHINE_REGS + 1];\n\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    fprintf(g, b"char *pl_reg_tbl[] = { \0" as *const u8 as *const libc::c_char);
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        while j < nb_reg[i as usize] {
            dp = &mut *(*reg.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize) as *mut RegInf;
            fprintf(
                g,
                b"\"%s\"%s\0" as *const u8 as *const libc::c_char,
                ((*dp).name).as_mut_ptr(),
                if k < total_nb_reg - 1 as libc::c_int {
                    b", \0" as *const u8 as *const libc::c_char
                } else {
                    b"};\n\0" as *const u8 as *const libc::c_char
                },
            );
            k += 1;
            k;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fprintf(g, b"\n#else\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"extern WamWord *save_reg_bank;\n\n\0" as *const u8 as *const libc::c_char,
    );
    if regs_to_save_for_signal != 0 {
        fprintf(
            g,
            b"extern WamWord pl_buff_signal_reg[];\n\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    fprintf(g, b"extern char *pl_reg_tbl[];\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"\n#endif\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"#define Init_Reg_Bank(x)  save_reg_bank = pl_reg_bank = x\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(g, b"\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"#define Reg(i)\t\t\t(\0" as *const u8 as *const libc::c_char);
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        while j < nb_reg[i as usize] {
            dp = &mut *(*reg.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize) as *mut RegInf;
            if k < total_nb_reg - 1 as libc::c_int {
                let fresh2 = k;
                k = k + 1;
                fprintf(
                    g,
                    b"((i)==%d) ? (WamWord) %-3s\t: \\\n\t\t\t\t \0" as *const u8
                        as *const libc::c_char,
                    fresh2,
                    ((*dp).name).as_mut_ptr(),
                );
            } else {
                fprintf(
                    g,
                    b"           (WamWord) %s)\n\0" as *const u8 as *const libc::c_char,
                    ((*dp).name).as_mut_ptr(),
                );
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fprintf(
        g,
        b"\n\n\n\n#define Save_All_Regs(buff_save) \\\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(g, b"  do { \\\n\0" as *const u8 as *const libc::c_char);
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        while j < nb_reg[i as usize] {
            dp = &mut *(*reg.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize) as *mut RegInf;
            fprintf(
                g,
                b"    buff_save[%d] = (WamWord) %s; \\\n\0" as *const u8
                    as *const libc::c_char,
                k,
                ((*dp).name).as_mut_ptr(),
            );
            k += 1;
            k;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fprintf(g, b"  } while(0)\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"\n\n\n\n#define Restore_All_Regs(buff_save) \\\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(g, b"  do { \\\n\0" as *const u8 as *const libc::c_char);
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        while j < nb_reg[i as usize] {
            dp = &mut *(*reg.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize) as *mut RegInf;
            fprintf(
                g,
                b"    %-6s = (%-8s) buff_save[%d]; \\\n\0" as *const u8
                    as *const libc::c_char,
                ((*dp).name).as_mut_ptr(),
                ((*dp).type_0).as_mut_ptr(),
                k,
            );
            k += 1;
            k;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fprintf(g, b"  } while(0)\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"\n\n\n\n#define Save_Machine_Regs(buff_save) \\\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(g, b"  do { \\\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_of_used_mach_regs {
        fprintf(
            g,
            b"    buff_save[%d] = (WamWord) %s; \\\n\0" as *const u8
                as *const libc::c_char,
            i,
            used_mach_reg[i as usize].pl_reg_name,
        );
        i += 1;
        i;
    }
    fprintf(g, b"  } while(0)\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"\n\n#define Restore_Machine_Regs(buff_save) \\\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(g, b"  do { \\\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_of_used_mach_regs {
        fprintf(
            g,
            b"    %s = (%-8s) buff_save[%d]; \\\n\0" as *const u8 as *const libc::c_char,
            used_mach_reg[i as usize].pl_reg_name,
            used_mach_reg[i as usize].type_0,
            i,
        );
        i += 1;
        i;
    }
    fprintf(g, b"  } while(0)\n\0" as *const u8 as *const libc::c_char);
    if regs_to_save_for_signal != 0 {
        fprintf(
            g,
            b"\n\n\n\n#define Start_Protect_Regs_For_Signal \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(g, b"  do { \\\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            g,
            b"    Save_Machine_Regs(pl_buff_signal_reg); \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            g,
            b"    pl_buff_signal_reg[NB_OF_USED_MACHINE_REGS] = 1; \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(g, b"  } while(0)\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            g,
            b"\n\n#define Stop_Protect_Regs_For_Signal \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            g,
            b"  pl_buff_signal_reg[NB_OF_USED_MACHINE_REGS] = 0; \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            g,
            b"\n\n#define Restore_Protect_Regs_For_Signal \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(g, b"  do { \\\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            g,
            b"    if (pl_buff_signal_reg[NB_OF_USED_MACHINE_REGS]) { \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            g,
            b"      Restore_Machine_Regs(pl_buff_signal_reg); \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            g,
            b"      Stop_Protect_Regs_For_Signal; \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(g, b"    } \\\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            g,
            b"    pl_reg_bank = save_reg_bank; \\\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(g, b"  } while(0)\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            g,
            b"\n\n\n\n#define Start_Protect_Regs_For_Signal\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            g,
            b"\n\n#define Stop_Protect_Regs_For_Signal\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            g,
            b"\n\n#define Restore_Protect_Regs_For_Signal\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    fprintf(
        g,
        b"\n\n   /*--- End Register Generation ---*/\n\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn Generate_Tags(mut f: *mut FILE, mut g: *mut FILE) {
    static mut str: [libc::c_char; 512] = [0; 512];
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag: [TagInf; 128] = [TagInf {
        name: [0; 32],
        type_0: SHORT_UNS,
        value: 0,
    }; 128];
    let mut nb_tag: libc::c_int = 0 as libc::c_int;
    let mut tag_size: libc::c_int = 0;
    let mut tag_size_low: libc::c_int = 0;
    let mut tag_size_high: libc::c_int = 0;
    let mut value_size: libc::c_int = 0;
    let mut max_value: libc::c_int = 0 as libc::c_int;
    let mut tag_mask: PlULong = 0;
    let mut min_integer: PlLong = 0;
    let mut max_integer: PlLong = 0;
    let mut i: libc::c_int = 0;
    loop {
        if feof(f) != 0
            || (fgets(
                str.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                    as libc::c_int,
                f,
            ))
                .is_null()
        {
            Pl_Fatal_Error(
                b"Syntax error: end expected for @begin tag\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if *str.as_mut_ptr() as libc::c_int != '@' as i32 {
            if *str.as_mut_ptr() as libc::c_int != '\n' as i32 {
                fputs(str.as_mut_ptr(), g);
            }
        } else {
            strcpy(save_str.as_mut_ptr(), str.as_mut_ptr());
            p1 = Read_Identifier(
                str.as_mut_ptr().offset(1 as libc::c_int as isize),
                1 as libc::c_int,
                &mut p2,
            );
            if strcmp(p1, b"end\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                break;
            }
            if strcmp(p1, b"tag\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                strcpy(
                    (tag[nb_tag as usize].name).as_mut_ptr(),
                    Read_Identifier(
                        p2.offset(1 as libc::c_int as isize),
                        1 as libc::c_int,
                        &mut p2,
                    ),
                );
                p1 = Read_Identifier(
                    p2.offset(1 as libc::c_int as isize),
                    1 as libc::c_int,
                    &mut p2,
                );
                if strcmp(p1, b"long_int\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    tag[nb_tag as usize].type_0 = LONG_INT;
                } else if strcmp(p1, b"short_uns\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    tag[nb_tag as usize].type_0 = SHORT_UNS;
                } else if strcmp(p1, b"address\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    tag[nb_tag as usize].type_0 = ADDRESS;
                } else {
                    Pl_Fatal_Error(
                        b"Syntax error: wrong tag type in: %s\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        save_str.as_mut_ptr(),
                    );
                }
                tag[nb_tag as usize]
                    .value = Pl_Read_Integer(
                    p2.offset(1 as libc::c_int as isize),
                    &mut p2,
                );
                if tag[nb_tag as usize].value > max_value {
                    max_value = tag[nb_tag as usize].value;
                }
                nb_tag += 1;
                nb_tag;
            } else {
                Pl_Fatal_Error(
                    b"Syntax error: incorrect @ declaration in: %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    save_str.as_mut_ptr(),
                );
            }
        }
    }
    fprintf(
        g,
        b"\n\n   /*--- Begin Tag Generation ---*/\n\n\0" as *const u8
            as *const libc::c_char,
    );
    max_value += 1;
    max_value;
    if max_value < nb_tag {
        Pl_Fatal_Error(
            b"There is an invalid tag value (repetition ?)\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    tag_size = 0 as libc::c_int;
    while (1 as libc::c_int) << tag_size < max_value {
        tag_size += 1;
        tag_size;
    }
    tag_size_low = 3 as libc::c_int;
    tag_size_high = tag_size - tag_size_low;
    value_size = 8 as libc::c_int * 8 as libc::c_int - tag_size;
    tag_mask = (((1 as libc::c_int) << tag_size) - 1 as libc::c_int) as PlULong
        >> tag_size_low << value_size + tag_size_low
        | (((1 as libc::c_int) << tag_size) - 1 as libc::c_int
            & ((1 as libc::c_int) << tag_size_low) - 1 as libc::c_int) as libc::c_ulong;
    max_integer = ((1 as libc::c_int as PlLong)
        << 8 as libc::c_int * 8 as libc::c_int - tag_size - 1 as libc::c_int)
        - 1 as libc::c_int as libc::c_long;
    min_integer = -max_integer - 1 as libc::c_int as libc::c_long;
    fprintf(
        fg_c,
        b"#define PL_MIN_INTEGER\t\t%ld\n\0" as *const u8 as *const libc::c_char,
        min_integer,
    );
    fprintf(
        fg_c,
        b"#define PL_MAX_INTEGER\t\t%ld\n\0" as *const u8 as *const libc::c_char,
        max_integer,
    );
    fputc('\n' as i32, fg_c);
    fprintf(
        g,
        b"#define TAG_SIZE     \t\t%d\n\0" as *const u8 as *const libc::c_char,
        tag_size,
    );
    fprintf(
        g,
        b"#define TAG_SIZE_LOW \t\t%d\n\0" as *const u8 as *const libc::c_char,
        tag_size_low,
    );
    fprintf(
        g,
        b"#define TAG_SIZE_HIGH\t\t%d\n\0" as *const u8 as *const libc::c_char,
        tag_size_high,
    );
    fprintf(
        g,
        b"#define VALUE_SIZE   \t\t%d\n\0" as *const u8 as *const libc::c_char,
        value_size,
    );
    fprintf(
        g,
        b"#define TAG_MASK     \t\t(PlULong)%#lx\n\0" as *const u8
            as *const libc::c_char,
        tag_mask,
    );
    fprintf(
        g,
        b"#define VALUE_MASK   \t\t(PlULong)%#lx\n\0" as *const u8
            as *const libc::c_char,
        !tag_mask,
    );
    fprintf(
        g,
        b"#define Tag_Mask_Of(w)\t\t((PlLong) (w) & (TAG_MASK))\n\0" as *const u8
            as *const libc::c_char,
    );
    if tag_size_high > 0 as libc::c_int {
        fprintf(
            g,
            b"#define Tag_From_Tag_Mask(w) \t(((PlULong) (w) >> %d) | ((w) & %d))\n\0"
                as *const u8 as *const libc::c_char,
            value_size,
            ((1 as libc::c_int) << tag_size_low) - 1 as libc::c_int,
        );
    } else {
        fprintf(
            g,
            b"#define Tag_From_Tag_Mask(w) \t(w)\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if tag_size_high > 0 as libc::c_int {
        fprintf(
            g,
            b"#define Tag_Of(w)     \t\t((((PlULong) (w) >> %d) << %d) | ((w) & %d))\n\0"
                as *const u8 as *const libc::c_char,
            8 as libc::c_int * 8 as libc::c_int - tag_size_high,
            tag_size_low,
            ((1 as libc::c_int) << tag_size_low) - 1 as libc::c_int,
        );
    } else {
        fprintf(
            g,
            b"#define Tag_Of(w)     \t\tTag_Mask_Of(w)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < nb_tag {
        fprintf(
            g,
            b"#define TAG_%s_MASK\t\t(PlULong)%#lx\n\0" as *const u8
                as *const libc::c_char,
            (tag[i as usize].name).as_mut_ptr(),
            tag[i as usize].value as PlULong >> tag_size_low << value_size + tag_size_low
                | (tag[i as usize].value
                    & ((1 as libc::c_int) << tag_size_low) - 1 as libc::c_int)
                    as libc::c_ulong,
        );
        i += 1;
        i;
    }
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"#define NB_OF_TAGS       \t%d\n\0" as *const u8 as *const libc::c_char,
        nb_tag,
    );
    i = 0 as libc::c_int;
    while i < nb_tag {
        fprintf(
            g,
            b"#define %-10s \t\t%-2d\n\0" as *const u8 as *const libc::c_char,
            (tag[i as usize].name).as_mut_ptr(),
            tag[i as usize].value,
        );
        fprintf(
            fg_c,
            b"#define PL_%-10s \t\t%-2d\n\0" as *const u8 as *const libc::c_char,
            (tag[i as usize].name).as_mut_ptr(),
            tag[i as usize].value,
        );
        i += 1;
        i;
    }
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"\t/* General Tag/UnTag macros */\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        g,
        b"#define Tag_Long_Int(tm, v)  \t((((PlLong) ((v) << %d)) >> %d) | (tm))\n\0"
            as *const u8 as *const libc::c_char,
        tag_size,
        tag_size_high,
    );
    fprintf(
        g,
        b"#define Tag_Short_Uns(tm, v)\t(((PlLong) (v) << %d) + (tm))\n\0" as *const u8
            as *const libc::c_char,
        tag_size_low,
    );
    fprintf(
        g,
        b"#define Tag_Address(tm, v)  \t((PlLong) (v) + (tm))\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"#define UnTag_Long_Int(w)    \t((PlLong) ((w) << %d) >> %d)\n\0" as *const u8
            as *const libc::c_char,
        tag_size_high,
        tag_size,
    );
    fprintf(
        g,
        b"#define UnTag_Short_Uns(w)\tUnTag_Long_Int(w)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        g,
        b"#define UnTag_Address(w)  \t((WamWord *) ((w) & VALUE_MASK))\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"\t/* Specialized Tag/UnTag macros */\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_tag {
        fprintf(
            g,
            b"#define Tag_%s(v)  \t\t\0" as *const u8 as *const libc::c_char,
            (tag[i as usize].name).as_mut_ptr(),
        );
        match tag[i as usize].type_0 as libc::c_uint {
            1 => {
                if tag[i as usize].value == 0 as libc::c_int {
                    fprintf(
                        g,
                        b"(((PlULong) (v) << %d) & VALUE_MASK)\n\0" as *const u8
                            as *const libc::c_char,
                        tag_size_low,
                    );
                } else if tag[i as usize].value
                    == ((1 as libc::c_int) << tag_size) - 1 as libc::c_int
                {
                    fprintf(
                        g,
                        b"(((PlULong) (v) << %d) | TAG_MASK)\n\0" as *const u8
                            as *const libc::c_char,
                        tag_size_low,
                    );
                } else {
                    fprintf(
                        g,
                        b"Tag_Long_Int(TAG_%s_MASK, v)\n\0" as *const u8
                            as *const libc::c_char,
                        (tag[i as usize].name).as_mut_ptr(),
                    );
                }
            }
            0 => {
                fprintf(
                    g,
                    b"Tag_Short_Uns(TAG_%s_MASK, v)\n\0" as *const u8
                        as *const libc::c_char,
                    (tag[i as usize].name).as_mut_ptr(),
                );
            }
            2 => {
                fprintf(
                    g,
                    b"Tag_Address(TAG_%s_MASK, v)\n\0" as *const u8
                        as *const libc::c_char,
                    (tag[i as usize].name).as_mut_ptr(),
                );
            }
            _ => {}
        }
        i += 1;
        i;
    }
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_tag {
        fprintf(
            g,
            b"#define UnTag_%s(w)  \t\t\0" as *const u8 as *const libc::c_char,
            (tag[i as usize].name).as_mut_ptr(),
        );
        match tag[i as usize].type_0 as libc::c_uint {
            1 => {
                fprintf(g, b"UnTag_Long_Int(w)\n\0" as *const u8 as *const libc::c_char);
            }
            0 => {
                if tag[i as usize].value <= 3 as libc::c_int {
                    fprintf(
                        g,
                        b"((PlULong) (w) >> %d)\n\0" as *const u8 as *const libc::c_char,
                        tag_size_low,
                    );
                } else {
                    fprintf(
                        g,
                        b"UnTag_Short_Uns(w)\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            2 => {
                if tag[i as usize].value == 0 as libc::c_int {
                    fprintf(
                        g,
                        b"((WamWord *) (w))\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    fprintf(
                        g,
                        b"UnTag_Address(w)\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    fprintf(g, b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_tag {
        fprintf(
            g,
            b"#define Tag_Is_%s(w)  \t\t(Tag_Mask_Of(w) == TAG_%s_MASK)\n\0" as *const u8
                as *const libc::c_char,
            (tag[i as usize].name).as_mut_ptr(),
            (tag[i as usize].name).as_mut_ptr(),
        );
        i += 1;
        i;
    }
    fprintf(g, b"\ntypedef enum\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"{\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"  LONG_INT,\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"  SHORT_UNS,\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"  ADDRESS\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"}TypTag;\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"\ntypedef struct\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"{\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"  char *name;\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"  TypTag type;\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"  int value;\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"  PlLong tag_mask;\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"}InfTag;\n\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"#ifdef ENGINE_FILE\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"InfTag pl_tag_tbl[] =\n{\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_tag {
        fprintf(
            g,
            b"  { \"%s\", %s, %d, %lx}%s\0" as *const u8 as *const libc::c_char,
            (tag[i as usize].name).as_mut_ptr(),
            if tag[i as usize].type_0 as libc::c_uint
                == LONG_INT as libc::c_int as libc::c_uint
            {
                b"LONG_INT\0" as *const u8 as *const libc::c_char
            } else if tag[i as usize].type_0 as libc::c_uint
                == SHORT_UNS as libc::c_int as libc::c_uint
            {
                b"SHORT_UNS\0" as *const u8 as *const libc::c_char
            } else {
                b"ADDRESS\0" as *const u8 as *const libc::c_char
            },
            tag[i as usize].value,
            tag[i as usize].value as PlULong >> tag_size_low << value_size + tag_size_low
                | (tag[i as usize].value
                    & ((1 as libc::c_int) << tag_size_low) - 1 as libc::c_int)
                    as libc::c_ulong,
            if i < nb_tag - 1 as libc::c_int {
                b",\n\0" as *const u8 as *const libc::c_char
            } else {
                b"\n};\n\0" as *const u8 as *const libc::c_char
            },
        );
        i += 1;
        i;
    }
    fprintf(g, b"\n#else\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"extern InfTag pl_tag_tbl[];\n\0" as *const u8 as *const libc::c_char);
    fprintf(g, b"\n#endif\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"\n\n   /*--- End Tag Generation ---*/\n\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn Generate_Stacks(mut f: *mut FILE, mut g: *mut FILE) {
    static mut str: [libc::c_char; 512] = [0; 512];
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut stack: [StackInf; 12] = [StackInf {
        name: [0; 32],
        desc: [0; 64],
        def_size: 0,
        top_macro: [0; 128],
    }; 12];
    let mut nb_stack: libc::c_int = 0 as libc::c_int;
    loop {
        if feof(f) != 0
            || (fgets(
                str.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                    as libc::c_int,
                f,
            ))
                .is_null()
        {
            Pl_Fatal_Error(
                b"Syntax error: end expected for @begin stack\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if *str.as_mut_ptr() as libc::c_int != '@' as i32 {
            if *str.as_mut_ptr() as libc::c_int != '\n' as i32 {
                fputs(str.as_mut_ptr(), g);
            }
        } else {
            strcpy(save_str.as_mut_ptr(), str.as_mut_ptr());
            p1 = Read_Identifier(
                str.as_mut_ptr().offset(1 as libc::c_int as isize),
                1 as libc::c_int,
                &mut p2,
            );
            if strcmp(p1, b"end\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                break;
            }
            if strcmp(p1, b"stack\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                strcpy(
                    (stack[nb_stack as usize].name).as_mut_ptr(),
                    Read_Identifier(
                        p2.offset(1 as libc::c_int as isize),
                        1 as libc::c_int,
                        &mut p2,
                    ),
                );
                strcpy(
                    (stack[nb_stack as usize].desc).as_mut_ptr(),
                    Read_String(p2.offset(1 as libc::c_int as isize), &mut p2),
                );
                i = Pl_Read_Integer(p2.offset(1 as libc::c_int as isize), &mut p2);
                stack[nb_stack as usize]
                    .def_size = ((i * 1024 as libc::c_int) as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<PlLong>() as libc::c_ulong)
                    as libc::c_int;
                strcpy(
                    (stack[nb_stack as usize].top_macro).as_mut_ptr(),
                    Read_Identifier(
                        p2.offset(1 as libc::c_int as isize),
                        1 as libc::c_int,
                        &mut p2,
                    ),
                );
                nb_stack += 1;
                nb_stack;
            } else {
                Pl_Fatal_Error(
                    b"Syntax error: incorrect @ declaration in: %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    save_str.as_mut_ptr(),
                );
            }
        }
    }
    fprintf(
        g,
        b"\n\n   /*--- Begin Stack Generation ---*/\n\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(g, b"#include \"wam_stacks.h\"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fw_s,
        b"#define NB_OF_STACKS \t\t%d\n\n\0" as *const u8 as *const libc::c_char,
        nb_stack,
    );
    i = 0 as libc::c_int;
    while i < nb_stack {
        strcpy(str.as_mut_ptr(), (stack[i as usize].name).as_mut_ptr());
        *str
            .as_mut_ptr() = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *str.as_mut_ptr() as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*str.as_mut_ptr() as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*str.as_mut_ptr() as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        fprintf(
            fw_s,
            b"#define %s_Stack       \t(pl_stk_tbl[%d].stack)\n\0" as *const u8
                as *const libc::c_char,
            str.as_mut_ptr(),
            i,
        );
        fprintf(
            fw_s,
            b"#define %s_Size        \t(pl_stk_tbl[%d].size)\n\0" as *const u8
                as *const libc::c_char,
            str.as_mut_ptr(),
            i,
        );
        fprintf(
            fw_s,
            b"#define %s_Offset(adr) \t((WamWord *)(adr) - %s_Stack)\n\0" as *const u8
                as *const libc::c_char,
            str.as_mut_ptr(),
            str.as_mut_ptr(),
        );
        fprintf(
            fw_s,
            b"#define %s_Used_Size   \t%s_Offset(%s)\n\n\0" as *const u8
                as *const libc::c_char,
            str.as_mut_ptr(),
            str.as_mut_ptr(),
            (stack[i as usize].top_macro).as_mut_ptr(),
        );
        i += 1;
        i;
    }
    fprintf(
        fw_s,
        b"\n#define Stack_Top(s)       \t(\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nb_stack - 1 as libc::c_int {
        fprintf(
            fw_s,
            b"((s) == %d) ? %s : \0" as *const u8 as *const libc::c_char,
            i,
            (stack[i as usize].top_macro).as_mut_ptr(),
        );
        i += 1;
        i;
    }
    fprintf(
        fw_s,
        b"%s)\n\0" as *const u8 as *const libc::c_char,
        (stack[(nb_stack - 1 as libc::c_int) as usize].top_macro).as_mut_ptr(),
    );
    fprintf(fw_s, b"\ntypedef struct\n\0" as *const u8 as *const libc::c_char);
    fprintf(fw_s, b"{\n\0" as *const u8 as *const libc::c_char);
    fprintf(fw_s, b"  char *name;\n\0" as *const u8 as *const libc::c_char);
    fprintf(fw_s, b"  char *desc;\n\0" as *const u8 as *const libc::c_char);
    fprintf(fw_s, b"  char *env_var_name;\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fw_s,
        b"  PlLong *p_def_size;\t/* used for fixed_sizes */\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        fw_s,
        b"  int default_size; \t/* in WamWords */\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        fw_s,
        b"  int size;         \t/* in WamWords */\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fw_s, b"  WamWord *stack;\n\0" as *const u8 as *const libc::c_char);
    fprintf(fw_s, b"}InfStack;\n\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(fw_s, b"#ifdef ENGINE_FILE\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fw_s,
        b"    /* these variables can be overwritten by top_comp.c (see stack size file) */\n\0"
            as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nb_stack {
        fprintf(
            fw_s,
            b"PlLong pl_def_%s_size;\n\0" as *const u8 as *const libc::c_char,
            (stack[i as usize].name).as_mut_ptr(),
        );
        i += 1;
        i;
    }
    fprintf(fw_s, b"PlLong pl_fixed_sizes;\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(fw_s, b"InfStack pl_stk_tbl[] =\n{\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_stack {
        strcpy(str.as_mut_ptr(), (stack[i as usize].name).as_mut_ptr());
        p1 = str.as_mut_ptr();
        while *p1 != 0 {
            *p1 = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *p1 as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(*p1 as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*p1 as libc::c_int as isize);
                }
                __res
            }) as libc::c_char;
            p1 = p1.offset(1);
            p1;
        }
        fprintf(
            fw_s,
            b" { \"%s\", %s, \"%sSZ\", &pl_def_%s_size, %d, 0, NULL }%s\0" as *const u8
                as *const libc::c_char,
            (stack[i as usize].name).as_mut_ptr(),
            (stack[i as usize].desc).as_mut_ptr(),
            str.as_mut_ptr(),
            (stack[i as usize].name).as_mut_ptr(),
            stack[i as usize].def_size,
            if i < nb_stack - 1 as libc::c_int {
                b",\n\0" as *const u8 as *const libc::c_char
            } else {
                b"\n};\n\0" as *const u8 as *const libc::c_char
            },
        );
        i += 1;
        i;
    }
    fprintf(fw_s, b"\n#else\n\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nb_stack {
        fprintf(
            fw_s,
            b"extern PlLong pl_def_%s_size;\n\0" as *const u8 as *const libc::c_char,
            (stack[i as usize].name).as_mut_ptr(),
        );
        i += 1;
        i;
    }
    fprintf(
        fw_s,
        b"extern PlLong pl_fixed_sizes;\n\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw_s,
        b"extern InfStack pl_stk_tbl[];\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fw_s, b"\n#endif\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        g,
        b"\n\n   /*--- End Stack Generation ---*/\n\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn Pl_Fatal_Error(mut format: *mut libc::c_char, mut args: ...) {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    vfprintf(stderr, format, arg_ptr.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
