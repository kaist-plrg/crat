use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Hash_Incr_Init(hi: *mut HashIncrInfo);
    fn Pl_Hash_Incr_Int32(hi: *mut HashIncrInfo, x: uint32_t);
    fn Pl_Hash_Incr_Int64(hi: *mut HashIncrInfo, x: uint64_t);
    fn Pl_Hash_Incr_Double(hi: *mut HashIncrInfo, x: libc::c_double);
    fn Pl_Hash_Incr_Term(hi: *mut HashIncrInfo) -> uint32_t;
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Obtain_Float(adr: *mut WamWord) -> libc::c_double;
    static mut pl_fd_variable_size: Option::<unsafe extern "C" fn() -> libc::c_int>;
    static mut pl_fd_copy_variable: Option::<unsafe extern "C" fn() -> libc::c_int>;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Positive_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Compound(
        start_word: WamWord,
        func: *mut libc::c_int,
        arity: *mut libc::c_int,
    ) -> *mut WamWord;
    fn Pl_Check_For_Un_Integer(start_word: WamWord);
    fn Pl_Un_Integer(value: PlLong, start_word: WamWord) -> Bool;
    static mut pl_representation_too_many_variables: libc::c_int;
    fn Pl_Err_Representation(atom_flag: libc::c_int);
    static mut pl_representation_max_arity: libc::c_int;
    static mut pl_type_predicate_indicator: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Instantiation();
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashIncrInfo {
    pub len: libc::c_int,
    pub hash: uint32_t,
}
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AtomProp {
    #[bitfield(name = "length", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "op_mask", ty = "libc::c_uint", bits = "16..=19")]
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "20..=21")]
    #[bitfield(name = "needs_quote", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "needs_scan", ty = "libc::c_uint", bits = "23..=23")]
    pub length_op_mask_type_0_needs_quote_needs_scan: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AtomInf {
    pub name: *mut libc::c_char,
    pub hash: libc::c_uint,
    pub prop: AtomProp,
    pub info: *mut libc::c_void,
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_pi_name_word: WamWord = 0;
pub static mut pl_pi_arity_word: WamWord = 0;
pub static mut pl_glob_dico_var: [PlLong; 32768] = [0; 32768];
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
static mut base_copy: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut vars: [WamWord; 65536] = [0; 65536];
static mut end_vars: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut top_vars: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub unsafe extern "C" fn Pl_Term_Compare(
    mut start_u_word: WamWord,
    mut start_v_word: WamWord,
) -> PlLong {
    let mut u_word: WamWord = 0;
    let mut u_tag_mask: WamWord = 0;
    let mut v_word: WamWord = 0;
    let mut v_tag_mask: WamWord = 0;
    let mut u_tag: WamWord = 0;
    let mut v_tag: WamWord = 0;
    let mut u_func: libc::c_int = 0;
    let mut u_arity: libc::c_int = 0;
    let mut u_arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut v_func: libc::c_int = 0;
    let mut v_arity: libc::c_int = 0;
    let mut v_arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    let mut deref_last_word: WamWord = 0;
    u_word = start_u_word;
    loop {
        deref_last_word = u_word;
        u_tag_mask = (u_word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
            as WamWord;
        if u_tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        u_word = *(u_word as *mut WamWord);
        if !(u_word != deref_last_word) {
            break;
        }
    }
    let mut deref_last_word_0: WamWord = 0;
    v_word = start_v_word;
    loop {
        deref_last_word_0 = v_word;
        v_tag_mask = (v_word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
            as WamWord;
        if v_tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        v_word = *(v_word as *mut WamWord);
        if !(v_word != deref_last_word_0) {
            break;
        }
    }
    u_tag = u_tag_mask;
    v_tag = v_tag_mask;
    match u_tag {
        0 => {
            return if v_tag != 0 as libc::c_int as libc::c_long {
                -(1 as libc::c_int) as libc::c_long
            } else {
                (u_word as *mut WamWord).offset_from(v_word as *mut WamWord)
                    as libc::c_long
            };
        }
        5 => {
            if v_tag == 0 as libc::c_int as libc::c_long {
                return 1 as libc::c_int as PlLong;
            }
            return if v_tag != 5 as libc::c_int as libc::c_long {
                -(1 as libc::c_int) as libc::c_long
            } else {
                ((u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord)
                    .offset_from(
                        (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                            as *mut WamWord,
                    ) as libc::c_long
            };
        }
        4 => {
            if v_tag == 0 as libc::c_int as libc::c_long
                || v_tag == 5 as libc::c_int as libc::c_long
            {
                return 1 as libc::c_int as PlLong;
            }
            if v_tag != 4 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int) as PlLong;
            }
            d1 = Pl_Obtain_Float(
                (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord,
            );
            d2 = Pl_Obtain_Float(
                (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord,
            );
            return (if d1 < d2 {
                -(1 as libc::c_int)
            } else if d1 == d2 {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }) as PlLong;
        }
        7 => {
            if v_tag == 0 as libc::c_int as libc::c_long
                || v_tag == 5 as libc::c_int as libc::c_long
                || v_tag == 4 as libc::c_int as libc::c_long
            {
                return 1 as libc::c_int as PlLong;
            }
            return if v_tag != 7 as libc::c_int as libc::c_long {
                -(1 as libc::c_int) as libc::c_long
            } else {
                (u_word << 0 as libc::c_int >> 3 as libc::c_int)
                    - (v_word << 0 as libc::c_int >> 3 as libc::c_int)
            };
        }
        3 => {
            if v_tag == 0 as libc::c_int as libc::c_long
                || v_tag == 5 as libc::c_int as libc::c_long
                || v_tag == 4 as libc::c_int as libc::c_long
                || v_tag == 7 as libc::c_int as libc::c_long
            {
                return 1 as libc::c_int as PlLong;
            }
            return (if v_tag != 3 as libc::c_int as libc::c_long {
                -(1 as libc::c_int)
            } else {
                strcmp(
                    (*pl_atom_tbl
                        .offset((u_word as PlULong >> 3 as libc::c_int) as isize))
                        .name,
                    (*pl_atom_tbl
                        .offset((v_word as PlULong >> 3 as libc::c_int) as isize))
                        .name,
                )
            }) as PlLong;
        }
        _ => {}
    }
    v_arg_adr = Pl_Rd_Compound(v_word, &mut v_func, &mut v_arity);
    if v_arg_adr.is_null() {
        return 1 as libc::c_int as PlLong;
    }
    u_arg_adr = Pl_Rd_Compound(u_word, &mut u_func, &mut u_arity);
    if u_arity != v_arity {
        return (u_arity - v_arity) as PlLong;
    }
    if u_func != v_func {
        return strcmp(
            (*pl_atom_tbl.offset(u_func as isize)).name,
            (*pl_atom_tbl.offset(v_func as isize)).name,
        ) as PlLong;
    }
    i = 0 as libc::c_int;
    while i < u_arity {
        let fresh0 = u_arg_adr;
        u_arg_adr = u_arg_adr.offset(1);
        let fresh1 = v_arg_adr;
        v_arg_adr = v_arg_adr.offset(1);
        x = Pl_Term_Compare(*fresh0, *fresh1) as libc::c_int;
        if x != 0 as libc::c_int {
            return x as PlLong;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int as PlLong;
}
pub unsafe extern "C" fn Pl_Treat_Vars_Of_Term(
    mut start_word: WamWord,
    mut generic_var: Bool,
    mut fct: Option::<unsafe extern "C" fn() -> Bool>,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = start_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        match word as libc::c_ulong & 0x7 as libc::c_int as PlULong {
            0 => {
                if ::std::mem::transmute::<
                    _,
                    fn(_, _) -> Bool,
                >((Some(fct.unwrap())).unwrap())(word as *mut WamWord, word) == 0
                {
                    return 0 as libc::c_int;
                }
                break;
            }
            5 => {
                if generic_var != 0 {
                    if ::std::mem::transmute::<
                        _,
                        fn(_, _) -> Bool,
                    >(
                        (Some(fct.unwrap())).unwrap(),
                    )(
                        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                            as *mut WamWord,
                        word,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                }
                break;
            }
            1 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                adr = &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
                let fresh2 = adr;
                adr = adr.offset(1);
                if Pl_Treat_Vars_Of_Term(*fresh2, generic_var, fct) == 0 {
                    return 0 as libc::c_int;
                }
                start_word = *adr;
            }
            2 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                i = (*adr.offset(0 as libc::c_int as isize) as PlULong
                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    as libc::c_int;
                adr = &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                    as *mut WamWord;
                loop {
                    i -= 1;
                    if !(i != 0) {
                        break;
                    }
                    let fresh3 = adr;
                    adr = adr.offset(1);
                    if Pl_Treat_Vars_Of_Term(*fresh3, generic_var, fct) == 0 {
                        return 0 as libc::c_int;
                    }
                }
                start_word = *adr;
            }
            _ => {
                break;
            }
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_List_Length(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = start_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            return n;
        }
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            return -(1 as libc::c_int);
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            return -(2 as libc::c_int);
        }
        n += 1;
        n;
        start_word = *((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    };
}
pub unsafe extern "C" fn Pl_Term_Size(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = start_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        match tag_mask {
            5 => {
                return n + 1 as libc::c_int
                    + ::std::mem::transmute::<
                        _,
                        fn(_) -> libc::c_int,
                    >(
                        (Some(pl_fd_variable_size.unwrap())).unwrap(),
                    )(
                        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                            as *mut WamWord,
                    );
            }
            4 => return n + 1 as libc::c_int + 1 as libc::c_int,
            1 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                adr = &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
                let fresh4 = adr;
                adr = adr.offset(1);
                n += 1 as libc::c_int + Pl_Term_Size(*fresh4);
                start_word = *adr;
            }
            2 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                n += 2 as libc::c_int;
                i = (*adr.offset(0 as libc::c_int as isize) as PlULong
                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    as libc::c_int;
                adr = &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                    as *mut WamWord;
                loop {
                    i -= 1;
                    if !(i != 0) {
                        break;
                    }
                    let fresh5 = adr;
                    adr = adr.offset(1);
                    n += Pl_Term_Size(*fresh5);
                }
                start_word = *adr;
            }
            _ => return n + 1 as libc::c_int,
        }
    };
}
pub unsafe extern "C" fn Pl_Copy_Term(
    mut dst_adr: *mut WamWord,
    mut src_adr: *mut WamWord,
) {
    let mut qtop: *mut WamWord = 0 as *mut WamWord;
    let mut base: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    static mut fix_bug: *mut WamWord = 0 as *const WamWord as *mut WamWord;
    let fresh6 = dst_adr;
    dst_adr = dst_adr.offset(1);
    base_copy = fresh6;
    top_vars = vars.as_mut_ptr();
    base = top_vars;
    fix_bug = dst_adr;
    Copy_Term_Rec(base_copy, src_adr, &mut fix_bug);
    qtop = top_vars;
    while qtop != base {
        qtop = qtop.offset(-1);
        p = *qtop as *mut WamWord;
        qtop = qtop.offset(-1);
        *p = *qtop;
    }
}
unsafe extern "C" fn Copy_Term_Rec(
    mut dst_adr: *mut WamWord,
    mut src_adr: *mut WamWord,
    mut p: *mut *mut WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut q: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = *src_adr;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        match tag_mask {
            0 => {
                adr = word as *mut WamWord;
                q = *p;
                if adr < q && adr >= base_copy {
                    *dst_adr = word;
                    return;
                }
                if top_vars >= end_vars {
                    Pl_Err_Representation(pl_representation_too_many_variables);
                }
                let fresh7 = top_vars;
                top_vars = top_vars.offset(1);
                *fresh7 = word;
                let fresh8 = top_vars;
                top_vars = top_vars.offset(1);
                *fresh8 = adr as WamWord;
                *dst_adr = (dst_adr as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                *adr = *dst_adr;
                return;
            }
            5 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                q = *p;
                if adr < q && adr >= base_copy {
                    *dst_adr = (adr as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                    return;
                }
                if top_vars >= end_vars {
                    Pl_Err_Representation(pl_representation_too_many_variables);
                }
                let fresh9 = top_vars;
                top_vars = top_vars.offset(1);
                *fresh9 = word;
                let fresh10 = top_vars;
                top_vars = top_vars.offset(1);
                *fresh10 = adr as WamWord;
                q = *p;
                *p = q
                    .offset(
                        ::std::mem::transmute::<
                            _,
                            fn(_, _) -> libc::c_int,
                        >((Some(pl_fd_copy_variable.unwrap())).unwrap())(q, adr) as isize,
                    );
                *dst_adr = (q as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                *adr = *dst_adr;
                return;
            }
            4 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                q = *p;
                *q
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *adr.offset(0 as libc::c_int as isize);
                *p = q.offset(1 as libc::c_int as isize);
                *dst_adr = (q as PlLong as libc::c_ulong)
                    .wrapping_add(0x4 as libc::c_int as PlULong) as WamWord;
                return;
            }
            1 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                q = *p;
                *dst_adr = (q as PlLong as libc::c_ulong)
                    .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
                *p = (&mut *q.offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                    as *mut WamWord)
                    .offset(1 as libc::c_int as isize);
                q = &mut *q.offset(0 as libc::c_int as isize) as *mut WamWord;
                adr = &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
                let fresh11 = q;
                q = q.offset(1);
                let fresh12 = adr;
                adr = adr.offset(1);
                Copy_Term_Rec(fresh11, fresh12, p);
                dst_adr = q;
                src_adr = adr;
            }
            2 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                q = *p;
                *dst_adr = (q as PlLong as libc::c_ulong)
                    .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
                *q
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *adr.offset(0 as libc::c_int as isize);
                i = (*adr.offset(0 as libc::c_int as isize) as PlULong
                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    as libc::c_int;
                *p = (&mut *q.offset((1 as libc::c_int + i - 1 as libc::c_int) as isize)
                    as *mut WamWord)
                    .offset(1 as libc::c_int as isize);
                q = &mut *q.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                    as *mut WamWord;
                adr = &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                    as *mut WamWord;
                loop {
                    i -= 1;
                    if !(i != 0) {
                        break;
                    }
                    let fresh13 = q;
                    q = q.offset(1);
                    let fresh14 = adr;
                    adr = adr.offset(1);
                    Copy_Term_Rec(fresh13, fresh14, p);
                }
                dst_adr = q;
                src_adr = adr;
            }
            _ => {
                *dst_adr = word;
                return;
            }
        }
    };
}
pub unsafe extern "C" fn Pl_Copy_Contiguous_Term(
    mut dst_adr: *mut WamWord,
    mut src_adr: *mut WamWord,
) {
    let mut word: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut q: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    loop {
        word = *src_adr;
        match word as libc::c_ulong & 0x7 as libc::c_int as PlULong {
            0 => {
                adr = word as *mut WamWord;
                q = dst_adr.offset(adr.offset_from(src_adr) as libc::c_long as isize);
                *dst_adr = (q as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                if adr > src_adr {
                    Pl_Copy_Contiguous_Term(q, adr);
                }
                return;
            }
            5 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                ::std::mem::transmute::<
                    _,
                    fn(_, _) -> libc::c_int,
                >((Some(pl_fd_copy_variable.unwrap())).unwrap())(dst_adr, adr);
                return;
            }
            4 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                q = dst_adr.offset(adr.offset_from(src_adr) as libc::c_long as isize);
                *q
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *adr.offset(0 as libc::c_int as isize);
                *dst_adr = (q as PlLong as libc::c_ulong)
                    .wrapping_add(0x4 as libc::c_int as PlULong) as WamWord;
                return;
            }
            1 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                q = dst_adr.offset(adr.offset_from(src_adr) as libc::c_long as isize);
                *dst_adr = (q as PlLong as libc::c_ulong)
                    .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
                q = &mut *q.offset(0 as libc::c_int as isize) as *mut WamWord;
                adr = &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
                let fresh15 = q;
                q = q.offset(1);
                let fresh16 = adr;
                adr = adr.offset(1);
                Pl_Copy_Contiguous_Term(fresh15, fresh16);
                dst_adr = q;
                src_adr = adr;
            }
            2 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                q = dst_adr.offset(adr.offset_from(src_adr) as libc::c_long as isize);
                *dst_adr = (q as PlLong as libc::c_ulong)
                    .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
                *q
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *adr.offset(0 as libc::c_int as isize);
                i = (*adr.offset(0 as libc::c_int as isize) as PlULong
                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    as libc::c_int;
                q = &mut *q.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                    as *mut WamWord;
                adr = &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                    as *mut WamWord;
                loop {
                    i -= 1;
                    if !(i != 0) {
                        break;
                    }
                    let fresh17 = q;
                    q = q.offset(1);
                    let fresh18 = adr;
                    adr = adr.offset(1);
                    Pl_Copy_Contiguous_Term(fresh17, fresh18);
                }
                dst_adr = q;
                src_adr = adr;
            }
            _ => {
                *dst_adr = word;
                return;
            }
        }
    };
}
pub unsafe extern "C" fn Pl_Acyclic_Term_1(mut start_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut adr1: *mut WamWord = 0 as *mut WamWord;
    let mut arity: libc::c_int = 0;
    let mut ok: Bool = 1 as libc::c_int;
    let mut mark_base: *mut WamWord = H;
    let mut mark_sp: *mut WamWord = mark_base;
    let fresh19 = mark_sp;
    mark_sp = mark_sp.offset(1);
    *fresh19 = 1 as libc::c_int as WamWord;
    let fresh20 = mark_sp;
    mark_sp = mark_sp.offset(1);
    *fresh20 = &mut start_word as *mut WamWord as WamWord;
    while mark_sp > mark_base {
        word = *mark_sp.offset(-(1 as libc::c_int) as isize);
        if word & 1 as libc::c_int as libc::c_long != 0 {
            word = (word >> 1 as libc::c_int) << 1 as libc::c_int;
            adr = word as *mut WamWord;
            word = *mark_sp.offset(-(2 as libc::c_int) as isize);
            *adr = word;
        } else if !(ok == 0) {
            adr = word as *mut WamWord;
            start_word = *adr;
            let ref mut fresh21 = *mark_sp.offset(-(2 as libc::c_int) as isize);
            *fresh21 -= 1;
            if *fresh21 == 0 as libc::c_int as libc::c_long {
                mark_sp = mark_sp.offset(-(2 as libc::c_int as isize));
            } else {
                *mark_sp
                    .offset(
                        -(1 as libc::c_int) as isize,
                    ) = adr.offset(1 as libc::c_int as isize) as WamWord;
            }
            let mut deref_last_word: WamWord = 0;
            word = start_word;
            loop {
                deref_last_word = word;
                tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                    as WamWord;
                if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                    break;
                }
                word = *(word as *mut WamWord);
                if !(word != deref_last_word) {
                    break;
                }
            }
            if word as libc::c_ulong
                == (0 as libc::c_int as PlLong as libc::c_ulong)
                    .wrapping_add(0x1 as libc::c_int as PlULong)
            {
                ok = 0 as libc::c_int;
                continue;
            } else {
                if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
                    adr1 = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord;
                    arity = 2 as libc::c_int;
                    adr1 = &mut *adr1.offset(0 as libc::c_int as isize) as *mut WamWord;
                } else {
                    if !(tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong) {
                        continue;
                    }
                    adr1 = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord;
                    arity = (*adr1.offset(0 as libc::c_int as isize) as PlULong
                        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                        as libc::c_int;
                    adr1 = &mut *adr1
                        .offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                        as *mut WamWord;
                }
                let fresh22 = mark_sp;
                mark_sp = mark_sp.offset(1);
                *fresh22 = start_word;
                let fresh23 = mark_sp;
                mark_sp = mark_sp.offset(1);
                *fresh23 = adr as WamWord | 1 as libc::c_int as libc::c_long;
                *adr = (0 as libc::c_int as PlLong as libc::c_ulong)
                    .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
                let fresh24 = mark_sp;
                mark_sp = mark_sp.offset(1);
                *fresh24 = arity as WamWord;
                let fresh25 = mark_sp;
                mark_sp = mark_sp.offset(1);
                *fresh25 = adr1 as WamWord;
                continue;
            }
        }
        mark_sp = mark_sp.offset(-(2 as libc::c_int as isize));
    }
    return ok;
}
unsafe extern "C" fn Term_Hash(
    mut start_word: WamWord,
    mut depth: PlLong,
    mut hash: *mut libc::c_uint,
) -> Bool {
    let mut hi: HashIncrInfo = HashIncrInfo { len: 0, hash: 0 };
    Pl_Hash_Incr_Init(&mut hi);
    if depth != 0 as libc::c_int as libc::c_long
        && Term_Hash_Rec(start_word, depth, &mut hi) == 0
    {
        return 0 as libc::c_int;
    }
    *hash = Pl_Hash_Incr_Term(&mut hi);
    return 1 as libc::c_int;
}
unsafe extern "C" fn Term_Hash_Rec(
    mut start_word: WamWord,
    mut depth: PlLong,
    mut hi: *mut HashIncrInfo,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = start_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        match tag_mask {
            0 | 5 => return 0 as libc::c_int,
            3 => {
                Pl_Hash_Incr_Int32(
                    hi,
                    (*pl_atom_tbl.offset((word as PlULong >> 3 as libc::c_int) as isize))
                        .hash,
                );
                break;
            }
            7 => {
                Pl_Hash_Incr_Int64(
                    hi,
                    (word << 0 as libc::c_int >> 3 as libc::c_int) as uint64_t,
                );
                break;
            }
            4 => {
                Pl_Hash_Incr_Double(
                    hi,
                    Pl_Obtain_Float(
                        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                            as *mut WamWord,
                    ),
                );
                break;
            }
            1 => {
                depth -= 1;
                if depth == 0 as libc::c_int as libc::c_long {
                    Pl_Hash_Incr_Int32(
                        hi,
                        (*pl_atom_tbl
                            .offset('.' as i32 as libc::c_uchar as libc::c_int as isize))
                            .hash,
                    );
                    Pl_Hash_Incr_Int32(hi, 2 as libc::c_int as uint32_t);
                    break;
                } else {
                    adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord;
                    if Term_Hash_Rec(*adr.offset(0 as libc::c_int as isize), depth, hi)
                        == 0
                    {
                        return 0 as libc::c_int;
                    }
                    start_word = *adr
                        .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
                }
            }
            2 => {
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                func = (*adr.offset(0 as libc::c_int as isize) as PlULong
                    & ((1 as libc::c_int as PlULong)
                        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
                arity = (*adr.offset(0 as libc::c_int as isize) as PlULong
                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    as libc::c_int;
                Pl_Hash_Incr_Int32(hi, (*pl_atom_tbl.offset(func as isize)).hash);
                Pl_Hash_Incr_Int32(hi, arity as uint32_t);
                depth -= 1;
                if depth == 0 as libc::c_int as libc::c_long {
                    break;
                }
                adr = &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                    as *mut WamWord;
                loop {
                    arity -= 1;
                    if !(arity != 0) {
                        break;
                    }
                    let fresh26 = adr;
                    adr = adr.offset(1);
                    if Term_Hash_Rec(*fresh26, depth, hi) == 0 {
                        return 0 as libc::c_int;
                    }
                }
                start_word = *adr;
            }
            _ => {
                break;
            }
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Term_Hash_4(
    mut start_word: WamWord,
    mut depth_word: WamWord,
    mut range_word: WamWord,
    mut hash_word: WamWord,
) -> Bool {
    let mut depth: PlLong = Pl_Rd_Integer_Check(depth_word);
    let mut range: PlLong = Pl_Rd_Positive_Check(range_word);
    let mut hash: libc::c_uint = 0;
    if range <= 0 as libc::c_int as libc::c_long
        || range
            > ((1 as libc::c_int)
                << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
                as libc::c_long
    {
        range = ((1 as libc::c_int)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int) as PlLong;
    }
    Pl_Check_For_Un_Integer(hash_word);
    if Term_Hash(start_word, depth, &mut hash) == 0 {
        return 1 as libc::c_int;
    }
    return Pl_Un_Integer(hash as libc::c_long % range, hash_word);
}
pub unsafe extern "C" fn Pl_Term_Hash_2(
    mut start_word: WamWord,
    mut hash_word: WamWord,
) -> Bool {
    return Pl_Term_Hash_4(
        start_word,
        ((-(1 as libc::c_int) as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord,
        ((0 as libc::c_int as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord,
        hash_word,
    );
}
pub unsafe extern "C" fn Pl_Get_Pred_Indicator(
    mut pred_indic_word: WamWord,
    mut must_be_ground: Bool,
    mut arity: *mut libc::c_int,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut func: libc::c_int = 0;
    let mut arity1: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = pred_indic_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong && must_be_ground != 0 {
        Pl_Err_Instantiation();
    }
    if Pl_Get_Structure(
        '/' as i32 as libc::c_uchar as libc::c_int,
        2 as libc::c_int,
        pred_indic_word,
    ) == 0
    {
        Pl_Err_Type(pl_type_predicate_indicator, pred_indic_word);
    }
    pl_pi_name_word = Pl_Unify_Variable();
    pl_pi_arity_word = Pl_Unify_Variable();
    let mut deref_last_word_0: WamWord = 0;
    word = pl_pi_name_word;
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
    if must_be_ground == 0 && tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        func = -(1 as libc::c_int);
    } else {
        func = Pl_Rd_Atom_Check(word);
    }
    let mut deref_last_word_1: WamWord = 0;
    word = pl_pi_arity_word;
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
    if must_be_ground == 0 && tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        *arity = -(1 as libc::c_int);
    } else {
        arity1 = Pl_Rd_Positive_Check(pl_pi_arity_word);
        if arity1 > (256 as libc::c_int - 1 as libc::c_int) as libc::c_long {
            Pl_Err_Representation(pl_representation_max_arity);
        }
        *arity = arity1 as libc::c_int;
    }
    return func;
}
pub unsafe extern "C" fn Pl_Get_Pred_Indic_3(
    mut pred_indic_word: WamWord,
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    func = Pl_Get_Pred_Indicator(pred_indic_word, 1 as libc::c_int, &mut arity);
    return (Pl_Get_Atom(func, func_word) != 0
        && Pl_Get_Integer(arity as PlLong, arity_word) != 0) as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    end_vars = vars
        .as_mut_ptr()
        .offset((32768 as libc::c_int * 2 as libc::c_int) as isize);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
