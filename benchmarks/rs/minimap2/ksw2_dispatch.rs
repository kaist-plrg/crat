use ::libc;
use ::c2rust_bitfields;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    fn abort() -> !;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ksw_extz_t {
    #[bitfield(name = "max", ty = "uint32_t", bits = "0..=30")]
    #[bitfield(name = "zdropped", ty = "uint32_t", bits = "31..=31")]
    pub max_zdropped: [u8; 4],
    pub max_q: libc::c_int,
    pub max_t: libc::c_int,
    pub mqe: libc::c_int,
    pub mqe_t: libc::c_int,
    pub mte: libc::c_int,
    pub mte_q: libc::c_int,
    pub score: libc::c_int,
    pub m_cigar: libc::c_int,
    pub n_cigar: libc::c_int,
    pub reach_end: libc::c_int,
    pub cigar: *mut uint32_t,
}
pub unsafe extern "C" fn __cpuidex(
    mut cpuid: *mut libc::c_int,
    mut func_id: libc::c_int,
    mut subfunc_id: libc::c_int,
) {
    let fresh0 = &mut *cpuid.offset(0 as libc::c_int as isize);
    let fresh1;
    let fresh2 = func_id;
    let fresh3 = &mut *cpuid.offset(2 as libc::c_int as isize);
    let fresh4;
    let fresh5 = subfunc_id;
    asm!(
        "cpuid\nmov {restmp0:x}, %bx", restmp0 = lateout(reg) * cpuid.offset(1 as
        libc::c_int as isize), inlateout("ax") c2rust_asm_casts::AsmCast::cast_in(fresh0,
        fresh2) => fresh1, inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, lateout("dx") * cpuid.offset(3 as libc::c_int as isize),
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
}
static mut ksw_simd: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn x86_simd() -> libc::c_int {
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut cpuid: [libc::c_int; 4] = [0; 4];
    let mut max_id: libc::c_int = 0;
    __cpuidex(cpuid.as_mut_ptr(), 0 as libc::c_int, 0 as libc::c_int);
    max_id = cpuid[0 as libc::c_int as usize];
    if max_id == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    __cpuidex(cpuid.as_mut_ptr(), 1 as libc::c_int, 0 as libc::c_int);
    if cpuid[3 as libc::c_int as usize] >> 25 as libc::c_int & 1 as libc::c_int != 0 {
        flag |= 0x1 as libc::c_int;
    }
    if cpuid[3 as libc::c_int as usize] >> 26 as libc::c_int & 1 as libc::c_int != 0 {
        flag |= 0x2 as libc::c_int;
    }
    if cpuid[2 as libc::c_int as usize] >> 0 as libc::c_int & 1 as libc::c_int != 0 {
        flag |= 0x4 as libc::c_int;
    }
    if cpuid[2 as libc::c_int as usize] >> 9 as libc::c_int & 1 as libc::c_int != 0 {
        flag |= 0x8 as libc::c_int;
    }
    if cpuid[2 as libc::c_int as usize] >> 19 as libc::c_int & 1 as libc::c_int != 0 {
        flag |= 0x10 as libc::c_int;
    }
    if cpuid[2 as libc::c_int as usize] >> 20 as libc::c_int & 1 as libc::c_int != 0 {
        flag |= 0x20 as libc::c_int;
    }
    if cpuid[2 as libc::c_int as usize] >> 28 as libc::c_int & 1 as libc::c_int != 0 {
        flag |= 0x40 as libc::c_int;
    }
    if max_id >= 7 as libc::c_int {
        __cpuidex(cpuid.as_mut_ptr(), 7 as libc::c_int, 0 as libc::c_int);
        if cpuid[1 as libc::c_int as usize] >> 5 as libc::c_int & 1 as libc::c_int != 0 {
            flag |= 0x80 as libc::c_int;
        }
        if cpuid[1 as libc::c_int as usize] >> 16 as libc::c_int & 1 as libc::c_int != 0
        {
            flag |= 0x100 as libc::c_int;
        }
    }
    return flag;
}
pub unsafe extern "C" fn ksw_extz2_sse(
    mut km: *mut libc::c_void,
    mut qlen: libc::c_int,
    mut query: *const uint8_t,
    mut tlen: libc::c_int,
    mut target: *const uint8_t,
    mut m: int8_t,
    mut mat: *const int8_t,
    mut q: int8_t,
    mut e: int8_t,
    mut w: libc::c_int,
    mut zdrop: libc::c_int,
    mut end_bonus: libc::c_int,
    mut flag: libc::c_int,
    mut ez: *mut ksw_extz_t,
) {
    extern "C" {
        #[link_name = "ksw_extz2_sse2"]
        fn ksw_extz2_sse2_0(
            km_0: *mut libc::c_void,
            qlen_0: libc::c_int,
            query_0: *const uint8_t,
            tlen_0: libc::c_int,
            target_0: *const uint8_t,
            m_0: int8_t,
            mat_0: *const int8_t,
            q_0: int8_t,
            e_0: int8_t,
            w_0: libc::c_int,
            zdrop_0: libc::c_int,
            end_bonus_0: libc::c_int,
            flag_0: libc::c_int,
            ez_0: *mut ksw_extz_t,
        );
    }
    extern "C" {
        #[link_name = "ksw_extz2_sse41"]
        fn ksw_extz2_sse41_0(
            km_0: *mut libc::c_void,
            qlen_0: libc::c_int,
            query_0: *const uint8_t,
            tlen_0: libc::c_int,
            target_0: *const uint8_t,
            m_0: int8_t,
            mat_0: *const int8_t,
            q_0: int8_t,
            e_0: int8_t,
            w_0: libc::c_int,
            zdrop_0: libc::c_int,
            end_bonus_0: libc::c_int,
            flag_0: libc::c_int,
            ez_0: *mut ksw_extz_t,
        );
    }
    if ksw_simd < 0 as libc::c_int {
        ksw_simd = x86_simd();
    }
    if ksw_simd & 0x10 as libc::c_int != 0 {
        ksw_extz2_sse41_0(
            km,
            qlen,
            query,
            tlen,
            target,
            m,
            mat,
            q,
            e,
            w,
            zdrop,
            end_bonus,
            flag,
            ez,
        );
    } else if ksw_simd & 0x2 as libc::c_int != 0 {
        ksw_extz2_sse2_0(
            km,
            qlen,
            query,
            tlen,
            target,
            m,
            mat,
            q,
            e,
            w,
            zdrop,
            end_bonus,
            flag,
            ez,
        );
    } else {
        abort();
    };
}
pub unsafe extern "C" fn ksw_extd2_sse(
    mut km: *mut libc::c_void,
    mut qlen: libc::c_int,
    mut query: *const uint8_t,
    mut tlen: libc::c_int,
    mut target: *const uint8_t,
    mut m: int8_t,
    mut mat: *const int8_t,
    mut q: int8_t,
    mut e: int8_t,
    mut q2: int8_t,
    mut e2: int8_t,
    mut w: libc::c_int,
    mut zdrop: libc::c_int,
    mut end_bonus: libc::c_int,
    mut flag: libc::c_int,
    mut ez: *mut ksw_extz_t,
) {
    extern "C" {
        #[link_name = "ksw_extd2_sse2"]
        fn ksw_extd2_sse2_0(
            km_0: *mut libc::c_void,
            qlen_0: libc::c_int,
            query_0: *const uint8_t,
            tlen_0: libc::c_int,
            target_0: *const uint8_t,
            m_0: int8_t,
            mat_0: *const int8_t,
            q_0: int8_t,
            e_0: int8_t,
            q2_0: int8_t,
            e2_0: int8_t,
            w_0: libc::c_int,
            zdrop_0: libc::c_int,
            end_bonus_0: libc::c_int,
            flag_0: libc::c_int,
            ez_0: *mut ksw_extz_t,
        );
    }
    extern "C" {
        #[link_name = "ksw_extd2_sse41"]
        fn ksw_extd2_sse41_0(
            km_0: *mut libc::c_void,
            qlen_0: libc::c_int,
            query_0: *const uint8_t,
            tlen_0: libc::c_int,
            target_0: *const uint8_t,
            m_0: int8_t,
            mat_0: *const int8_t,
            q_0: int8_t,
            e_0: int8_t,
            q2_0: int8_t,
            e2_0: int8_t,
            w_0: libc::c_int,
            zdrop_0: libc::c_int,
            end_bonus_0: libc::c_int,
            flag_0: libc::c_int,
            ez_0: *mut ksw_extz_t,
        );
    }
    if ksw_simd < 0 as libc::c_int {
        ksw_simd = x86_simd();
    }
    if ksw_simd & 0x10 as libc::c_int != 0 {
        ksw_extd2_sse41_0(
            km,
            qlen,
            query,
            tlen,
            target,
            m,
            mat,
            q,
            e,
            q2,
            e2,
            w,
            zdrop,
            end_bonus,
            flag,
            ez,
        );
    } else if ksw_simd & 0x2 as libc::c_int != 0 {
        ksw_extd2_sse2_0(
            km,
            qlen,
            query,
            tlen,
            target,
            m,
            mat,
            q,
            e,
            q2,
            e2,
            w,
            zdrop,
            end_bonus,
            flag,
            ez,
        );
    } else {
        abort();
    };
}
pub unsafe extern "C" fn ksw_exts2_sse(
    mut km: *mut libc::c_void,
    mut qlen: libc::c_int,
    mut query: *const uint8_t,
    mut tlen: libc::c_int,
    mut target: *const uint8_t,
    mut m: int8_t,
    mut mat: *const int8_t,
    mut q: int8_t,
    mut e: int8_t,
    mut q2: int8_t,
    mut noncan: int8_t,
    mut zdrop: libc::c_int,
    mut junc_bonus: int8_t,
    mut flag: libc::c_int,
    mut junc: *const uint8_t,
    mut ez: *mut ksw_extz_t,
) {
    extern "C" {
        #[link_name = "ksw_exts2_sse2"]
        fn ksw_exts2_sse2_0(
            km_0: *mut libc::c_void,
            qlen_0: libc::c_int,
            query_0: *const uint8_t,
            tlen_0: libc::c_int,
            target_0: *const uint8_t,
            m_0: int8_t,
            mat_0: *const int8_t,
            q_0: int8_t,
            e_0: int8_t,
            q2_0: int8_t,
            noncan_0: int8_t,
            zdrop_0: libc::c_int,
            junc_bonus_0: int8_t,
            flag_0: libc::c_int,
            junc_0: *const uint8_t,
            ez_0: *mut ksw_extz_t,
        );
    }
    extern "C" {
        #[link_name = "ksw_exts2_sse41"]
        fn ksw_exts2_sse41_0(
            km_0: *mut libc::c_void,
            qlen_0: libc::c_int,
            query_0: *const uint8_t,
            tlen_0: libc::c_int,
            target_0: *const uint8_t,
            m_0: int8_t,
            mat_0: *const int8_t,
            q_0: int8_t,
            e_0: int8_t,
            q2_0: int8_t,
            noncan_0: int8_t,
            zdrop_0: libc::c_int,
            junc_bonus_0: int8_t,
            flag_0: libc::c_int,
            junc_0: *const uint8_t,
            ez_0: *mut ksw_extz_t,
        );
    }
    if ksw_simd < 0 as libc::c_int {
        ksw_simd = x86_simd();
    }
    if ksw_simd & 0x10 as libc::c_int != 0 {
        ksw_exts2_sse41_0(
            km,
            qlen,
            query,
            tlen,
            target,
            m,
            mat,
            q,
            e,
            q2,
            noncan,
            zdrop,
            junc_bonus,
            flag,
            junc,
            ez,
        );
    } else if ksw_simd & 0x2 as libc::c_int != 0 {
        ksw_exts2_sse2_0(
            km,
            qlen,
            query,
            tlen,
            target,
            m,
            mat,
            q,
            e,
            q2,
            noncan,
            zdrop,
            junc_bonus,
            flag,
            junc,
            ez,
        );
    } else {
        abort();
    };
}
