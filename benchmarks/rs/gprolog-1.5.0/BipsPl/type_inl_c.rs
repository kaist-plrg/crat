use ::libc;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Blt_Var(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Non_Var(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return !(tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Atom(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Integer(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Float(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0x4 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Number(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x4 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Atomic(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        || (tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
            || tag_mask as libc::c_ulong == 0x4 as libc::c_int as PlULong))
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Compound(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Callable(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        || (tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong
            || tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong))
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Fd_Var(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Non_Fd_Var(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return !(tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Generic_Var(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return (tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Non_Generic_Var(mut x: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = x;
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
    return !(tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Ground(mut start_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
            || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
        {
            return 0 as libc::c_int;
        }
        if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            if Pl_Blt_Ground(*adr.offset(0 as libc::c_int as isize)) == 0 {
                return 0 as libc::c_int;
            }
            start_word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
        } else {
            if !(tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong) {
                break;
            }
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            arity = (*adr.offset(0 as libc::c_int as isize) as PlULong
                >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
            adr = &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut WamWord;
            loop {
                arity -= 1;
                if !(arity > 0 as libc::c_int) {
                    break;
                }
                let fresh0 = adr;
                adr = adr.offset(1);
                if Pl_Blt_Ground(*fresh0) == 0 {
                    return 0 as libc::c_int;
                }
            }
            start_word = *adr;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_List(mut start_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
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
            return 1 as libc::c_int;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            return 0 as libc::c_int;
        }
        start_word = *((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    };
}
pub unsafe extern "C" fn Pl_Blt_Partial_List(mut start_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            return 1 as libc::c_int;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            return 0 as libc::c_int;
        }
        start_word = *((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    };
}
pub unsafe extern "C" fn Pl_Blt_List_Or_Partial_List(mut start_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
            || word as libc::c_ulong
                == (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            return 1 as libc::c_int;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            return 0 as libc::c_int;
        }
        start_word = *((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    };
}
