use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memchr2(
        s: *const libc::c_void,
        c1: libc::c_int,
        c2: libc::c_int,
        n: size_t,
    ) -> *mut libc::c_void;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kwsmatch {
    pub index: idx_t,
    pub offset: idx_t,
    pub size: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kwset {
    pub obstack: obstack,
    pub words: idx_t,
    pub trie: *mut trie,
    pub mind: idx_t,
    pub delta: [libc::c_uchar; 256],
    pub next: [*mut trie; 256],
    pub target: *mut libc::c_char,
    pub shift: *mut idx_t,
    pub trans: *const libc::c_char,
    pub gc1: libc::c_int,
    pub gc1help: libc::c_int,
    pub gc2: libc::c_char,
    pub kwsexec: Option::<
        unsafe extern "C" fn(
            kwset_t,
            *const libc::c_char,
            idx_t,
            *mut kwsmatch,
            bool,
        ) -> ptrdiff_t,
    >,
}
pub type kwset_t = *mut kwset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trie {
    pub accepting: ptrdiff_t,
    pub links: *mut tree,
    pub parent: *mut trie,
    pub next: *mut trie,
    pub fail: *mut trie,
    pub depth: idx_t,
    pub shift: idx_t,
    pub maxshift: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree {
    pub llink: *mut tree,
    pub rlink: *mut tree,
    pub trie: *mut trie,
    pub label: libc::c_uchar,
    pub balance: libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub const L: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const R: C2RustUnnamed_2 = 1;
pub const NCHAR: C2RustUnnamed_3 = 256;
pub type C2RustUnnamed_3 = libc::c_uint;
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
unsafe extern "C" fn U(mut ch: libc::c_char) -> libc::c_uchar {
    return to_uchar(ch);
}
#[inline]
unsafe extern "C" fn tr(
    mut trans: *const libc::c_char,
    mut c: libc::c_char,
) -> libc::c_char {
    return (if !trans.is_null() {
        *trans.offset(U(c) as isize) as libc::c_int
    } else {
        c as libc::c_int
    }) as libc::c_char;
}
pub unsafe extern "C" fn kwsalloc(mut trans: *const libc::c_char) -> kwset_t {
    let mut kwset: *mut kwset = xmalloc(::std::mem::size_of::<kwset>() as libc::c_ulong)
        as *mut kwset;
    _obstack_begin(
        &mut (*kwset).obstack,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    (*kwset).words = 0 as libc::c_int as idx_t;
    (*kwset)
        .trie = ({
        let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = ::std::mem::size_of::<trie>() as libc::c_ulong;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        {
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }
    }) as *mut trie;
    (*(*kwset).trie).accepting = 0 as libc::c_int as ptrdiff_t;
    (*(*kwset).trie).links = 0 as *mut tree;
    (*(*kwset).trie).parent = 0 as *mut trie;
    (*(*kwset).trie).next = 0 as *mut trie;
    (*(*kwset).trie).fail = 0 as *mut trie;
    (*(*kwset).trie).depth = 0 as libc::c_int as idx_t;
    (*(*kwset).trie).shift = 0 as libc::c_int as idx_t;
    (*kwset).mind = 9223372036854775807 as libc::c_long;
    (*kwset).target = 0 as *mut libc::c_char;
    (*kwset).trans = trans;
    (*kwset)
        .kwsexec = Some(
        acexec
            as unsafe extern "C" fn(
                kwset_t,
                *const libc::c_char,
                idx_t,
                *mut kwsmatch,
                bool,
            ) -> ptrdiff_t,
    );
    return kwset;
}
pub unsafe extern "C" fn kwsincr(
    mut kwset: kwset_t,
    mut text: *const libc::c_char,
    mut len: idx_t,
) {
    if 0 as libc::c_int as libc::c_long <= len {} else {
        unreachable!();
    };
    let mut trie: *mut trie = (*kwset).trie;
    let mut trans: *const libc::c_char = (*kwset).trans;
    let mut reverse: bool = (*kwset).kwsexec
        == Some(
            bmexec
                as unsafe extern "C" fn(
                    kwset_t,
                    *const libc::c_char,
                    idx_t,
                    *mut kwsmatch,
                    bool,
                ) -> ptrdiff_t,
        );
    if reverse {
        text = text.offset(len as isize);
    }
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut uc: libc::c_uchar = (if reverse as libc::c_int != 0 {
            text = text.offset(-1);
            *text as libc::c_int
        } else {
            let fresh1 = text;
            text = text.offset(1);
            *fresh1 as libc::c_int
        }) as libc::c_uchar;
        let mut label: libc::c_uchar = (if !trans.is_null() {
            *trans.offset(uc as isize) as libc::c_int
        } else {
            uc as libc::c_int
        }) as libc::c_uchar;
        let mut cur: *mut tree = (*trie).links;
        let mut links: [*mut tree; 12] = [0 as *mut tree; 12];
        let mut dirs: [C2RustUnnamed_2; 12] = [L; 12];
        links[0 as libc::c_int
            as usize] = &mut (*trie).links as *mut *mut tree as *mut tree;
        dirs[0 as libc::c_int as usize] = L;
        let mut depth: idx_t = 1 as libc::c_int as idx_t;
        while !cur.is_null() && label as libc::c_int != (*cur).label as libc::c_int {
            links[depth as usize] = cur;
            if (label as libc::c_int) < (*cur).label as libc::c_int {
                let fresh2 = depth;
                depth = depth + 1;
                dirs[fresh2 as usize] = L;
                cur = (*cur).llink;
            } else {
                let fresh3 = depth;
                depth = depth + 1;
                dirs[fresh3 as usize] = R;
                cur = (*cur).rlink;
            }
        }
        if cur.is_null() {
            cur = ({
                let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
                let mut __o: *mut obstack = __h;
                let mut __len: size_t = ::std::mem::size_of::<tree>() as libc::c_ulong;
                if ({
                    let mut __o1: *const obstack = __o;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < __len
                {
                    _obstack_newchunk(__o, __len);
                }
                (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                {
                    let mut __o1: *mut obstack = __h;
                    let mut __value: *mut libc::c_void = (*__o1).object_base
                        as *mut libc::c_void;
                    if (*__o1).next_free == __value as *mut libc::c_char {
                        (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                    }
                    (*__o1)
                        .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                        as libc::c_ulong)
                        < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        (*__o1).object_base
                    } else {
                        0 as *mut libc::c_char
                    })
                        .offset(
                            ((((*__o1).next_free)
                                .offset_from(
                                    if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        < ::std::mem::size_of::<*mut libc::c_void>()
                                            as libc::c_ulong
                                    {
                                        (*__o1).object_base
                                    } else {
                                        0 as *mut libc::c_char
                                    },
                                ) as libc::c_long as libc::c_ulong)
                                .wrapping_add((*__o1).alignment_mask)
                                & !(*__o1).alignment_mask) as isize,
                        );
                    if ((*__o1).next_free)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                        > ((*__o1).chunk_limit)
                            .offset_from((*__o1).chunk as *mut libc::c_char)
                            as libc::c_long as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                }
            }) as *mut tree;
            (*cur).llink = 0 as *mut tree;
            (*cur).rlink = 0 as *mut tree;
            (*cur)
                .trie = ({
                let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
                let mut __o: *mut obstack = __h;
                let mut __len: size_t = ::std::mem::size_of::<trie>() as libc::c_ulong;
                if ({
                    let mut __o1: *const obstack = __o;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < __len
                {
                    _obstack_newchunk(__o, __len);
                }
                (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                {
                    let mut __o1: *mut obstack = __h;
                    let mut __value: *mut libc::c_void = (*__o1).object_base
                        as *mut libc::c_void;
                    if (*__o1).next_free == __value as *mut libc::c_char {
                        (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                    }
                    (*__o1)
                        .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                        as libc::c_ulong)
                        < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        (*__o1).object_base
                    } else {
                        0 as *mut libc::c_char
                    })
                        .offset(
                            ((((*__o1).next_free)
                                .offset_from(
                                    if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        < ::std::mem::size_of::<*mut libc::c_void>()
                                            as libc::c_ulong
                                    {
                                        (*__o1).object_base
                                    } else {
                                        0 as *mut libc::c_char
                                    },
                                ) as libc::c_long as libc::c_ulong)
                                .wrapping_add((*__o1).alignment_mask)
                                & !(*__o1).alignment_mask) as isize,
                        );
                    if ((*__o1).next_free)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                        > ((*__o1).chunk_limit)
                            .offset_from((*__o1).chunk as *mut libc::c_char)
                            as libc::c_long as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                }
            }) as *mut trie;
            (*(*cur).trie).accepting = 0 as libc::c_int as ptrdiff_t;
            (*(*cur).trie).links = 0 as *mut tree;
            (*(*cur).trie).parent = trie;
            (*(*cur).trie).next = 0 as *mut trie;
            (*(*cur).trie).fail = 0 as *mut trie;
            (*(*cur).trie).depth = (*trie).depth + 1 as libc::c_int as libc::c_long;
            (*(*cur).trie).shift = 0 as libc::c_int as idx_t;
            (*cur).label = label;
            (*cur).balance = 0 as libc::c_int as libc::c_char;
            depth -= 1;
            if dirs[depth as usize] as libc::c_uint == L as libc::c_int as libc::c_uint {
                (*links[depth as usize]).llink = cur;
            } else {
                (*links[depth as usize]).rlink = cur;
            }
            while depth != 0 && (*links[depth as usize]).balance == 0 {
                if dirs[depth as usize] as libc::c_uint
                    == L as libc::c_int as libc::c_uint
                {
                    (*links[depth as usize]).balance -= 1;
                    (*links[depth as usize]).balance;
                } else {
                    (*links[depth as usize]).balance += 1;
                    (*links[depth as usize]).balance;
                }
                depth -= 1;
                depth;
            }
            if depth != 0
                && (dirs[depth as usize] as libc::c_uint
                    == L as libc::c_int as libc::c_uint
                    && {
                        (*links[depth as usize]).balance -= 1;
                        (*links[depth as usize]).balance as libc::c_int != 0
                    }
                    || dirs[depth as usize] as libc::c_uint
                        == R as libc::c_int as libc::c_uint
                        && {
                            (*links[depth as usize]).balance += 1;
                            (*links[depth as usize]).balance as libc::c_int != 0
                        })
            {
                let mut t: *mut tree = 0 as *mut tree;
                let mut r: *mut tree = 0 as *mut tree;
                let mut l: *mut tree = 0 as *mut tree;
                let mut rl: *mut tree = 0 as *mut tree;
                let mut lr: *mut tree = 0 as *mut tree;
                match (*links[depth as usize]).balance as libc::c_int {
                    -2 => {
                        match dirs[(depth + 1 as libc::c_int as libc::c_long) as usize]
                            as libc::c_uint
                        {
                            0 => {
                                r = links[depth as usize];
                                t = (*r).llink;
                                rl = (*t).rlink;
                                (*t).rlink = r;
                                (*r).llink = rl;
                                (*r).balance = 0 as libc::c_int as libc::c_char;
                                (*t).balance = (*r).balance;
                            }
                            1 => {
                                r = links[depth as usize];
                                l = (*r).llink;
                                t = (*l).rlink;
                                rl = (*t).rlink;
                                lr = (*t).llink;
                                (*t).llink = l;
                                (*l).rlink = lr;
                                (*t).rlink = r;
                                (*r).llink = rl;
                                (*l)
                                    .balance = (if (*t).balance as libc::c_int
                                    != 1 as libc::c_int
                                {
                                    0 as libc::c_int
                                } else {
                                    -(1 as libc::c_int)
                                }) as libc::c_char;
                                (*r)
                                    .balance = (if (*t).balance as libc::c_int
                                    != -(1 as libc::c_int) as libc::c_char as libc::c_int
                                {
                                    0 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) as libc::c_char;
                                (*t).balance = 0 as libc::c_int as libc::c_char;
                            }
                            _ => {
                                abort();
                            }
                        }
                    }
                    2 => {
                        match dirs[(depth + 1 as libc::c_int as libc::c_long) as usize]
                            as libc::c_uint
                        {
                            1 => {
                                l = links[depth as usize];
                                t = (*l).rlink;
                                lr = (*t).llink;
                                (*t).llink = l;
                                (*l).rlink = lr;
                                (*l).balance = 0 as libc::c_int as libc::c_char;
                                (*t).balance = (*l).balance;
                            }
                            0 => {
                                l = links[depth as usize];
                                r = (*l).rlink;
                                t = (*r).llink;
                                lr = (*t).llink;
                                rl = (*t).rlink;
                                (*t).llink = l;
                                (*l).rlink = lr;
                                (*t).rlink = r;
                                (*r).llink = rl;
                                (*l)
                                    .balance = (if (*t).balance as libc::c_int
                                    != 1 as libc::c_int
                                {
                                    0 as libc::c_int
                                } else {
                                    -(1 as libc::c_int)
                                }) as libc::c_char;
                                (*r)
                                    .balance = (if (*t).balance as libc::c_int
                                    != -(1 as libc::c_int) as libc::c_char as libc::c_int
                                {
                                    0 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                }) as libc::c_char;
                                (*t).balance = 0 as libc::c_int as libc::c_char;
                            }
                            _ => {
                                abort();
                            }
                        }
                    }
                    _ => {
                        abort();
                    }
                }
                if dirs[(depth - 1 as libc::c_int as libc::c_long) as usize]
                    as libc::c_uint == L as libc::c_int as libc::c_uint
                {
                    (*links[(depth - 1 as libc::c_int as libc::c_long) as usize])
                        .llink = t;
                } else {
                    (*links[(depth - 1 as libc::c_int as libc::c_long) as usize])
                        .rlink = t;
                }
            }
        }
        trie = (*cur).trie;
    }
    if (*trie).accepting == 0 {
        (*trie)
            .accepting = 2 as libc::c_int as libc::c_long * (*kwset).words
            + 1 as libc::c_int as libc::c_long;
    }
    (*kwset).words += 1;
    (*kwset).words;
    if (*trie).depth < (*kwset).mind {
        (*kwset).mind = (*trie).depth;
    }
}
pub unsafe extern "C" fn kwswords(mut kwset: kwset_t) -> idx_t {
    return (*kwset).words;
}
unsafe extern "C" fn enqueue(mut tree: *mut tree, mut last: *mut *mut trie) {
    if tree.is_null() {
        return;
    }
    enqueue((*tree).llink, last);
    enqueue((*tree).rlink, last);
    (**last).next = (*tree).trie;
    *last = (**last).next;
}
unsafe extern "C" fn treefails(
    mut tree: *const tree,
    mut fail: *const trie,
    mut recourse: *mut trie,
    mut reverse: bool,
) {
    let mut cur: *mut tree = 0 as *mut tree;
    if tree.is_null() {
        return;
    }
    treefails((*tree).llink, fail, recourse, reverse);
    treefails((*tree).rlink, fail, recourse, reverse);
    while !fail.is_null() {
        cur = (*fail).links;
        while !cur.is_null()
            && (*tree).label as libc::c_int != (*cur).label as libc::c_int
        {
            if ((*tree).label as libc::c_int) < (*cur).label as libc::c_int {
                cur = (*cur).llink;
            } else {
                cur = (*cur).rlink;
            }
        }
        if !cur.is_null() {
            (*(*tree).trie).fail = (*cur).trie;
            if !reverse && (*(*cur).trie).accepting != 0
                && (*(*tree).trie).accepting == 0
            {
                (*(*tree).trie).accepting = -(1 as libc::c_int) as ptrdiff_t;
            }
            return;
        }
        fail = (*fail).fail;
    }
    (*(*tree).trie).fail = recourse;
}
unsafe extern "C" fn treedelta(
    mut tree: *const tree,
    mut depth: idx_t,
    mut delta: *mut libc::c_uchar,
) {
    if tree.is_null() {
        return;
    }
    treedelta((*tree).llink, depth, delta);
    treedelta((*tree).rlink, depth, delta);
    if depth < *delta.offset((*tree).label as isize) as libc::c_long {
        *delta.offset((*tree).label as isize) = depth as libc::c_uchar;
    }
}
unsafe extern "C" fn hasevery(mut a: *const tree, mut b: *const tree) -> bool {
    if b.is_null() {
        return 1 as libc::c_int != 0;
    }
    if !hasevery(a, (*b).llink) {
        return 0 as libc::c_int != 0;
    }
    if !hasevery(a, (*b).rlink) {
        return 0 as libc::c_int != 0;
    }
    while !a.is_null() && (*b).label as libc::c_int != (*a).label as libc::c_int {
        if ((*b).label as libc::c_int) < (*a).label as libc::c_int {
            a = (*a).llink;
        } else {
            a = (*a).rlink;
        }
    }
    return !a.is_null();
}
unsafe extern "C" fn treenext(mut tree: *const tree, mut next: *mut *mut trie) {
    if tree.is_null() {
        return;
    }
    treenext((*tree).llink, next);
    treenext((*tree).rlink, next);
    let ref mut fresh4 = *next.offset((*tree).label as isize);
    *fresh4 = (*tree).trie;
}
pub unsafe extern "C" fn kwsprep(mut kwset: kwset_t) {
    let mut trans: *const libc::c_char = (*kwset).trans;
    let mut deltabuf: [libc::c_uchar; 256] = [0; 256];
    let mut delta: *mut libc::c_uchar = if !trans.is_null() {
        deltabuf.as_mut_ptr()
    } else {
        ((*kwset).delta).as_mut_ptr()
    };
    let mut curr: *mut trie = 0 as *mut trie;
    let mut last: *mut trie = 0 as *mut trie;
    let mut reverse: bool = (*kwset).words == 1 as libc::c_int as libc::c_long;
    if reverse {
        let mut new_kwset: kwset_t = 0 as *mut kwset;
        last = (*kwset).trie;
        curr = last;
        while !curr.is_null() {
            enqueue((*curr).links, &mut last);
            curr = (*curr).next;
        }
        (*kwset)
            .target = ({
            let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
            let mut __o: *mut obstack = __h;
            let mut __len: size_t = (*kwset).mind as size_t;
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            {
                let mut __o1: *mut obstack = __h;
                let mut __value: *mut libc::c_void = (*__o1).object_base
                    as *mut libc::c_void;
                if (*__o1).next_free == __value as *mut libc::c_char {
                    (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                }
                (*__o1)
                    .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                    as libc::c_ulong)
                    < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    (*__o1).object_base
                } else {
                    0 as *mut libc::c_char
                })
                    .offset(
                        ((((*__o1).next_free)
                            .offset_from(
                                if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                },
                            ) as libc::c_long as libc::c_ulong)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
                    > ((*__o1).chunk_limit)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                {
                    (*__o1).next_free = (*__o1).chunk_limit;
                }
                (*__o1).object_base = (*__o1).next_free;
                __value
            }
        }) as *mut libc::c_char;
        curr = (*kwset).trie;
        let mut i: idx_t = 0 as libc::c_int as idx_t;
        while i < (*kwset).mind {
            *((*kwset).target)
                .offset(i as isize) = (*(*curr).links).label as libc::c_char;
            curr = (*curr).next;
            i += 1;
            i;
        }
        new_kwset = kwsalloc((*kwset).trans);
        (*new_kwset)
            .kwsexec = Some(
            bmexec
                as unsafe extern "C" fn(
                    kwset_t,
                    *const libc::c_char,
                    idx_t,
                    *mut kwsmatch,
                    bool,
                ) -> ptrdiff_t,
        );
        kwsincr(new_kwset, (*kwset).target, (*kwset).mind);
        let mut __o: *mut obstack = &mut (*kwset).obstack;
        let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
        if __obj > (*__o).chunk as *mut libc::c_void
            && __obj < (*__o).chunk_limit as *mut libc::c_void
        {
            (*__o).object_base = __obj as *mut libc::c_char;
            (*__o).next_free = (*__o).object_base;
        } else {
            _obstack_free(__o, __obj);
        }
        *kwset = *new_kwset;
        rpl_free(new_kwset as *mut libc::c_void);
    }
    memset(
        delta as *mut libc::c_void,
        (if (*kwset).mind
            < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_long
        {
            (*kwset).mind
        } else {
            (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_long
        }) as libc::c_int,
        ::std::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong,
    );
    last = (*kwset).trie;
    curr = last;
    while !curr.is_null() {
        enqueue((*curr).links, &mut last);
        treedelta((*curr).links, (*curr).depth, delta);
        treefails((*curr).links, (*curr).fail, (*kwset).trie, reverse);
        if reverse {
            (*curr).shift = (*kwset).mind;
            (*curr).maxshift = (*kwset).mind;
            let mut fail: *mut trie = 0 as *mut trie;
            fail = (*curr).fail;
            while !fail.is_null() {
                if !hasevery((*fail).links, (*curr).links) {
                    if (*curr).depth - (*fail).depth < (*fail).shift {
                        (*fail).shift = (*curr).depth - (*fail).depth;
                    }
                }
                if (*curr).accepting != 0
                    && (*fail).maxshift > (*curr).depth - (*fail).depth
                {
                    (*fail).maxshift = (*curr).depth - (*fail).depth;
                }
                fail = (*fail).fail;
            }
        }
        curr = (*curr).next;
    }
    if reverse {
        curr = (*(*kwset).trie).next;
        while !curr.is_null() {
            if (*curr).maxshift > (*(*curr).parent).maxshift {
                (*curr).maxshift = (*(*curr).parent).maxshift;
            }
            if (*curr).shift > (*curr).maxshift {
                (*curr).shift = (*curr).maxshift;
            }
            curr = (*curr).next;
        }
    }
    let mut nextbuf: [*mut trie; 256] = [0 as *mut trie; 256];
    let mut next: *mut *mut trie = if !trans.is_null() {
        nextbuf.as_mut_ptr()
    } else {
        ((*kwset).next).as_mut_ptr()
    };
    memset(
        next as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut trie; 256]>() as libc::c_ulong,
    );
    treenext((*(*kwset).trie).links, next);
    let mut gc1: libc::c_int = -(2 as libc::c_int);
    let mut gc1help: libc::c_int = -(1 as libc::c_int);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < NCHAR as libc::c_int {
        let mut ti: libc::c_int = i_0;
        if !trans.is_null() {
            ti = U(*trans.offset(i_0 as isize)) as libc::c_int;
            (*kwset).next[i_0 as usize] = *next.offset(ti as isize);
        }
        if !((*kwset).next[i_0 as usize]).is_null() {
            if gc1 < -(1 as libc::c_int) {
                gc1 = ti;
                gc1help = i_0;
            } else if gc1 == ti {
                gc1help = if gc1help == ti { i_0 } else { -(1 as libc::c_int) };
            } else if i_0 == ti && gc1 == gc1help {
                gc1help = i_0;
            } else {
                gc1 = -(1 as libc::c_int);
            }
        }
        i_0 += 1;
        i_0;
    }
    (*kwset).gc1 = gc1;
    (*kwset).gc1help = gc1help;
    if reverse {
        (*kwset)
            .target = ({
            let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
            let mut __o_0: *mut obstack = __h;
            let mut __len: size_t = (*kwset).mind as size_t;
            if ({
                let mut __o1: *const obstack = __o_0;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len
            {
                _obstack_newchunk(__o_0, __len);
            }
            (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
            {
                let mut __o1: *mut obstack = __h;
                let mut __value: *mut libc::c_void = (*__o1).object_base
                    as *mut libc::c_void;
                if (*__o1).next_free == __value as *mut libc::c_char {
                    (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                }
                (*__o1)
                    .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                    as libc::c_ulong)
                    < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    (*__o1).object_base
                } else {
                    0 as *mut libc::c_char
                })
                    .offset(
                        ((((*__o1).next_free)
                            .offset_from(
                                if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                },
                            ) as libc::c_long as libc::c_ulong)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
                    > ((*__o1).chunk_limit)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                {
                    (*__o1).next_free = (*__o1).chunk_limit;
                }
                (*__o1).object_base = (*__o1).next_free;
                __value
            }
        }) as *mut libc::c_char;
        curr = (*kwset).trie;
        let mut i_1: idx_t = (*kwset).mind;
        while (0 as libc::c_int as libc::c_long) < i_1 {
            *((*kwset).target)
                .offset(
                    (i_1 - 1 as libc::c_int as libc::c_long) as isize,
                ) = (*(*curr).links).label as libc::c_char;
            curr = (*curr).next;
            i_1 -= 1;
            i_1;
        }
        if (*kwset).mind > 1 as libc::c_int as libc::c_long {
            (*kwset)
                .shift = ({
                let mut __h: *mut obstack = &mut (*kwset).obstack as *mut obstack;
                let mut __o_0: *mut obstack = __h;
                let mut __len: size_t = (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*kwset).mind - 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong,
                    );
                if ({
                    let mut __o1: *const obstack = __o_0;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < __len
                {
                    _obstack_newchunk(__o_0, __len);
                }
                (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
                {
                    let mut __o1: *mut obstack = __h;
                    let mut __value: *mut libc::c_void = (*__o1).object_base
                        as *mut libc::c_void;
                    if (*__o1).next_free == __value as *mut libc::c_char {
                        (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                    }
                    (*__o1)
                        .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                        as libc::c_ulong)
                        < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        (*__o1).object_base
                    } else {
                        0 as *mut libc::c_char
                    })
                        .offset(
                            ((((*__o1).next_free)
                                .offset_from(
                                    if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        < ::std::mem::size_of::<*mut libc::c_void>()
                                            as libc::c_ulong
                                    {
                                        (*__o1).object_base
                                    } else {
                                        0 as *mut libc::c_char
                                    },
                                ) as libc::c_long as libc::c_ulong)
                                .wrapping_add((*__o1).alignment_mask)
                                & !(*__o1).alignment_mask) as isize,
                        );
                    if ((*__o1).next_free)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                        > ((*__o1).chunk_limit)
                            .offset_from((*__o1).chunk as *mut libc::c_char)
                            as libc::c_long as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                }
            }) as *mut idx_t;
            curr = (*(*kwset).trie).next;
            let mut i_2: idx_t = 0 as libc::c_int as idx_t;
            while i_2 < (*kwset).mind - 1 as libc::c_int as libc::c_long {
                *((*kwset).shift).offset(i_2 as isize) = (*curr).shift;
                curr = (*curr).next;
                i_2 += 1;
                i_2;
            }
            (*kwset)
                .gc2 = tr(
                trans,
                *((*kwset).target)
                    .offset(((*kwset).mind - 2 as libc::c_int as libc::c_long) as isize),
            );
        }
    }
    if !trans.is_null() {
        let mut i_3: libc::c_int = 0 as libc::c_int;
        while i_3 < NCHAR as libc::c_int {
            (*kwset)
                .delta[i_3
                as usize] = *delta.offset(U(*trans.offset(i_3 as isize)) as isize);
            i_3 += 1;
            i_3;
        }
    }
}
#[inline]
unsafe extern "C" fn bm_delta2_search(
    mut tpp: *mut *const libc::c_char,
    mut ep: *const libc::c_char,
    mut sp: *const libc::c_char,
    mut len: idx_t,
    mut trans: *const libc::c_char,
    mut gc1: libc::c_char,
    mut gc2: libc::c_char,
    mut d1: *const libc::c_uchar,
    mut kwset: kwset_t,
) -> bool {
    let mut tp: *const libc::c_char = *tpp;
    let mut d: idx_t = len;
    let mut skip: idx_t = 0 as libc::c_int as idx_t;
    loop {
        let mut i: idx_t = 2 as libc::c_int as idx_t;
        if tr(trans, *tp.offset(-(2 as libc::c_int) as isize)) as libc::c_int
            == gc2 as libc::c_int
        {
            loop {
                i += 1;
                if !(i <= d) {
                    break;
                }
                if tr(trans, *tp.offset(-i as isize)) as libc::c_int
                    != tr(trans, *sp.offset(-i as isize)) as libc::c_int
                {
                    break;
                }
            }
            if i > d {
                i = d + skip + 1 as libc::c_int as libc::c_long;
                while i <= len {
                    if tr(trans, *tp.offset(-i as isize)) as libc::c_int
                        != tr(trans, *sp.offset(-i as isize)) as libc::c_int
                    {
                        break;
                    }
                    i += 1;
                    i;
                }
                if i > len {
                    *tpp = tp.offset(-(len as isize));
                    return 1 as libc::c_int != 0;
                }
            }
        }
        d = *((*kwset).shift).offset((i - 2 as libc::c_int as libc::c_long) as isize);
        tp = tp.offset(d as isize);
        if tp > ep {
            break;
        }
        if tr(trans, *tp.offset(-(1 as libc::c_int) as isize)) as libc::c_int
            != gc1 as libc::c_int
        {
            if !d1.is_null() {
                tp = tp
                    .offset(
                        *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                            as libc::c_int as isize,
                    );
            }
            break;
        } else {
            skip = i - 1 as libc::c_int as libc::c_long;
        }
    }
    *tpp = tp;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn memchr_kwset(
    mut s: *const libc::c_char,
    mut n: idx_t,
    mut kwset: kwset_t,
) -> *const libc::c_char {
    let mut slim: *const libc::c_char = s.offset(n as isize);
    if (*kwset).gc1help < 0 as libc::c_int {
        while s < slim {
            if !((*kwset).next[U(*s) as usize]).is_null() {
                return s;
            }
            s = s.offset(1);
            s;
        }
    } else {
        let mut small_heuristic: libc::c_int = 2 as libc::c_int;
        let mut small_bytes: idx_t = (small_heuristic as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            as idx_t;
        while s < slim {
            if !((*kwset).next[U(*s) as usize]).is_null() {
                return s;
            }
            s = s.offset(1);
            s;
            if (s as uintptr_t).wrapping_rem(small_bytes as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            {
                return memchr2(
                    s as *const libc::c_void,
                    (*kwset).gc1,
                    (*kwset).gc1help,
                    slim.offset_from(s) as libc::c_long as size_t,
                ) as *const libc::c_char;
            }
        }
    }
    return 0 as *const libc::c_char;
}
#[inline]
unsafe extern "C" fn bmexec_trans(
    mut kwset: kwset_t,
    mut text: *const libc::c_char,
    mut size: idx_t,
) -> ptrdiff_t {
    if 0 as libc::c_int as libc::c_long <= size {} else {
        unreachable!();
    };
    let mut d1: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ep: *const libc::c_char = 0 as *const libc::c_char;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tp: *const libc::c_char = 0 as *const libc::c_char;
    let mut d: libc::c_int = 0;
    let mut len: idx_t = (*kwset).mind;
    let mut trans: *const libc::c_char = (*kwset).trans;
    if len == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as ptrdiff_t;
    }
    if len > size {
        return -(1 as libc::c_int) as ptrdiff_t;
    }
    if len == 1 as libc::c_int as libc::c_long {
        tp = memchr_kwset(text, size, kwset);
        return if !tp.is_null() {
            tp.offset_from(text) as libc::c_long
        } else {
            -(1 as libc::c_int) as libc::c_long
        };
    }
    d1 = ((*kwset).delta).as_mut_ptr();
    sp = ((*kwset).target).offset(len as isize);
    tp = text.offset(len as isize);
    let mut gc1: libc::c_char = (*kwset).gc1 as libc::c_char;
    let mut gc2: libc::c_char = (*kwset).gc2;
    let mut len12: idx_t = 0;
    if !((if ::std::mem::size_of::<idx_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if !((0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t) {
            if (if (12 as libc::c_int) < 0 as libc::c_int {
                if len < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int
                        }) + 12 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (len < (127 as libc::c_int / 12 as libc::c_int) as libc::c_long)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            12 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((12 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 127 as libc::c_int
                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            127 as libc::c_int / -(12 as libc::c_int)
                        }) as libc::c_long <= -(1 as libc::c_int) as libc::c_long - len)
                            as libc::c_int
                    }
                } else {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            12 as libc::c_int
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            12 as libc::c_int
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_int
                    }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            len
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < len
                                    + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < len
                                && ((-(1 as libc::c_int)
                                    - (-(127 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_long) < len - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        ((((-(127 as libc::c_int) - 1 as libc::c_int)
                            / 12 as libc::c_int) as libc::c_long) < len) as libc::c_int
                    }
                }
            } else {
                if 12 as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    if len < 0 as libc::c_int as libc::c_long {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                len
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                len
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    })
                                        + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        })
                                            + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        })
                                            + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((0 as libc::c_int)
                                    < 12 as libc::c_int
                                        + (-(127 as libc::c_int) - 1 as libc::c_int)) as libc::c_int
                            } else {
                                (-(1 as libc::c_int)
                                    - (-(127 as libc::c_int) - 1 as libc::c_int)
                                    < 12 as libc::c_int - 1 as libc::c_int) as libc::c_int
                            }
                        } else {
                            ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                / len < 12 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        (((127 as libc::c_int / 12 as libc::c_int) as libc::c_long)
                            < len) as libc::c_int
                    }
                }
            }) != 0
            {
                len12 = (len as libc::c_uint)
                    .wrapping_mul(12 as libc::c_int as libc::c_uint) as libc::c_schar
                    as idx_t;
                1 as libc::c_int
            } else {
                len12 = (len as libc::c_uint)
                    .wrapping_mul(12 as libc::c_int as libc::c_uint) as libc::c_schar
                    as idx_t;
                0 as libc::c_int
            }
        } else {
            if (if (12 as libc::c_int) < 0 as libc::c_int {
                if len < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        }) + 12 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (len
                            < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                / 12 as libc::c_int) as libc::c_long) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            12 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((12 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            })
                                + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                / -(12 as libc::c_int)
                        }) as libc::c_long <= -(1 as libc::c_int) as libc::c_long - len)
                            as libc::c_int
                    }
                } else {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            12 as libc::c_int
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 0 as libc::c_int
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            12 as libc::c_int
                        }) + 0 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 0 as libc::c_int) as libc::c_int
                    }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            len
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < len + 0 as libc::c_int as libc::c_long) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < len
                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    as libc::c_long) < len - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        (((0 as libc::c_int / 12 as libc::c_int) as libc::c_long) < len)
                            as libc::c_int
                    }
                }
            } else {
                if 12 as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    if len < 0 as libc::c_int as libc::c_long {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                len
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                len
                            }) + 0 as libc::c_int as libc::c_long)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) + 0 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                        }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((0 as libc::c_int) < 12 as libc::c_int + 0 as libc::c_int)
                                    as libc::c_int
                            } else {
                                ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    < 12 as libc::c_int - 1 as libc::c_int) as libc::c_int
                            }
                        } else {
                            (0 as libc::c_int as libc::c_long / len
                                < 12 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        ((((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            / 12 as libc::c_int) as libc::c_long) < len) as libc::c_int
                    }
                }
            }) != 0
            {
                len12 = (len as libc::c_uint)
                    .wrapping_mul(12 as libc::c_int as libc::c_uint) as libc::c_uchar
                    as idx_t;
                1 as libc::c_int
            } else {
                len12 = (len as libc::c_uint)
                    .wrapping_mul(12 as libc::c_int as libc::c_uint) as libc::c_uchar
                    as idx_t;
                0 as libc::c_int
            }
        }
    } else {
        if ::std::mem::size_of::<idx_t>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if !((0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t) {
                if (if (12 as libc::c_int) < 0 as libc::c_int {
                    if len < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                32767 as libc::c_int
                            }) + 12 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            (len
                                < (32767 as libc::c_int / 12 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((12 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 32767 as libc::c_int
                                    >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                32767 as libc::c_int / -(12 as libc::c_int)
                            }) as libc::c_long
                                <= -(1 as libc::c_int) as libc::c_long - len) as libc::c_int
                        }
                    } else {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_int
                        }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                len
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((0 as libc::c_int as libc::c_long)
                                    < len
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < len
                                    && ((-(1 as libc::c_int)
                                        - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                        as libc::c_long) < len - 1 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            }
                        } else {
                            ((((-(32767 as libc::c_int) - 1 as libc::c_int)
                                / 12 as libc::c_int) as libc::c_long) < len) as libc::c_int
                        }
                    }
                } else {
                    if 12 as libc::c_int == 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        if len < 0 as libc::c_int as libc::c_long {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        })
                                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            })
                                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            })
                                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long) as libc::c_int
                            }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((0 as libc::c_int)
                                        < 12 as libc::c_int
                                            + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                        as libc::c_int
                                } else {
                                    (-(1 as libc::c_int)
                                        - (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        < 12 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                }
                            } else {
                                ((-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long / len < 12 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            }
                        } else {
                            (((32767 as libc::c_int / 12 as libc::c_int) as libc::c_long)
                                < len) as libc::c_int
                        }
                    }
                }) != 0
                {
                    len12 = (len as libc::c_uint)
                        .wrapping_mul(12 as libc::c_int as libc::c_uint) as libc::c_short
                        as idx_t;
                    1 as libc::c_int
                } else {
                    len12 = (len as libc::c_uint)
                        .wrapping_mul(12 as libc::c_int as libc::c_uint) as libc::c_short
                        as idx_t;
                    0 as libc::c_int
                }
            } else {
                if (if (12 as libc::c_int) < 0 as libc::c_int {
                    if len < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                            }) + 12 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            (len
                                < ((32767 as libc::c_int * 2 as libc::c_int
                                    + 1 as libc::c_int) / 12 as libc::c_int) as libc::c_long)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((12 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                })
                                    + (32767 as libc::c_int * 2 as libc::c_int
                                        + 1 as libc::c_int)
                                    >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                    / -(12 as libc::c_int)
                            }) as libc::c_long
                                <= -(1 as libc::c_int) as libc::c_long - len) as libc::c_int
                        }
                    } else {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 0 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 0 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                12 as libc::c_int
                            }) + 0 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 0 as libc::c_int) as libc::c_int
                        }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                len
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((0 as libc::c_int as libc::c_long)
                                    < len + 0 as libc::c_int as libc::c_long) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < len
                                    && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                        as libc::c_long) < len - 1 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            }
                        } else {
                            (((0 as libc::c_int / 12 as libc::c_int) as libc::c_long)
                                < len) as libc::c_int
                        }
                    }
                } else {
                    if 12 as libc::c_int == 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        if len < 0 as libc::c_int as libc::c_long {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) + 0 as libc::c_int as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) + 0 as libc::c_int as libc::c_long
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                }) + 0 as libc::c_int as libc::c_long)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                            }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((0 as libc::c_int) < 12 as libc::c_int + 0 as libc::c_int)
                                        as libc::c_int
                                } else {
                                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                                        < 12 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                }
                            } else {
                                (0 as libc::c_int as libc::c_long / len
                                    < 12 as libc::c_int as libc::c_long) as libc::c_int
                            }
                        } else {
                            ((((32767 as libc::c_int * 2 as libc::c_int
                                + 1 as libc::c_int) / 12 as libc::c_int) as libc::c_long)
                                < len) as libc::c_int
                        }
                    }
                }) != 0
                {
                    len12 = (len as libc::c_uint)
                        .wrapping_mul(12 as libc::c_int as libc::c_uint)
                        as libc::c_ushort as idx_t;
                    1 as libc::c_int
                } else {
                    len12 = (len as libc::c_uint)
                        .wrapping_mul(12 as libc::c_int as libc::c_uint)
                        as libc::c_ushort as idx_t;
                    0 as libc::c_int
                }
            }
        } else {
            if ::std::mem::size_of::<idx_t>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    len12
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    if (if (12 as libc::c_int) < 0 as libc::c_int {
                        if len < 0 as libc::c_int as libc::c_long {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2147483647 as libc::c_int
                                }) + 12 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                (len
                                    < (2147483647 as libc::c_int / 12 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    ((12 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 2147483647 as libc::c_int
                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    2147483647 as libc::c_int / -(12 as libc::c_int)
                                }) as libc::c_long
                                    <= -(1 as libc::c_int) as libc::c_long - len) as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_int
                            }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < len
                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < len
                                        && ((-(1 as libc::c_int)
                                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                            as libc::c_long) < len - 1 as libc::c_int as libc::c_long)
                                        as libc::c_int
                                }
                            } else {
                                ((((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    / 12 as libc::c_int) as libc::c_long) < len) as libc::c_int
                            }
                        }
                    } else {
                        if 12 as libc::c_int == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            if len < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    })
                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        })
                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        })
                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    })
                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            })
                                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    len
                                                })
                                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    len
                                                })
                                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        })
                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long) as libc::c_int
                                }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((0 as libc::c_int)
                                            < 12 as libc::c_int
                                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                            as libc::c_int
                                    } else {
                                        (-(1 as libc::c_int)
                                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            < 12 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                    }
                                } else {
                                    ((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long / len < 12 as libc::c_int as libc::c_long)
                                        as libc::c_int
                                }
                            } else {
                                (((2147483647 as libc::c_int / 12 as libc::c_int)
                                    as libc::c_long) < len) as libc::c_int
                            }
                        }
                    }) != 0
                    {
                        len12 = (len as libc::c_uint)
                            .wrapping_mul(12 as libc::c_int as libc::c_uint)
                            as libc::c_int as idx_t;
                        1 as libc::c_int
                    } else {
                        len12 = (len as libc::c_uint)
                            .wrapping_mul(12 as libc::c_int as libc::c_uint)
                            as libc::c_int as idx_t;
                        0 as libc::c_int
                    }
                } else {
                    if (if (12 as libc::c_int) < 0 as libc::c_int {
                        if len < 0 as libc::c_int as libc::c_long {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_uint
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    (2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint)
                                })
                                    .wrapping_add(12 as libc::c_int as libc::c_uint)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                < 0 as libc::c_int as libc::c_uint
                            {
                                (len
                                    < (2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint)
                                        .wrapping_div(12 as libc::c_int as libc::c_uint)
                                        as libc::c_long) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    ((12 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                                }) != 0
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) as libc::c_uint)
                                        .wrapping_add(
                                            (2147483647 as libc::c_int as libc::c_uint)
                                                .wrapping_mul(2 as libc::c_uint)
                                                .wrapping_add(1 as libc::c_uint),
                                        )
                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint)
                                        .wrapping_div(-(12 as libc::c_int) as libc::c_uint)
                                }) as libc::c_long
                                    <= -(1 as libc::c_int) as libc::c_long - len) as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    12 as libc::c_int
                                }) + 0 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int) as libc::c_int
                            }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    len
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < len + 0 as libc::c_int as libc::c_long) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < len
                                        && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                            as libc::c_long) < len - 1 as libc::c_int as libc::c_long)
                                        as libc::c_int
                                }
                            } else {
                                (((0 as libc::c_int / 12 as libc::c_int) as libc::c_long)
                                    < len) as libc::c_int
                            }
                        }
                    } else {
                        if 12 as libc::c_int == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            if len < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) + 0 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) + 0 as libc::c_int as libc::c_long)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    len
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    len
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((0 as libc::c_int) < 12 as libc::c_int + 0 as libc::c_int)
                                            as libc::c_int
                                    } else {
                                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                                            < 12 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                    }
                                } else {
                                    (0 as libc::c_int as libc::c_long / len
                                        < 12 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                (((2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint)
                                    .wrapping_div(12 as libc::c_int as libc::c_uint)
                                    as libc::c_long) < len) as libc::c_int
                            }
                        }
                    }) != 0
                    {
                        len12 = (len as libc::c_uint)
                            .wrapping_mul(12 as libc::c_int as libc::c_uint) as idx_t;
                        1 as libc::c_int
                    } else {
                        len12 = (len as libc::c_uint)
                            .wrapping_mul(12 as libc::c_int as libc::c_uint) as idx_t;
                        0 as libc::c_int
                    }
                }
            } else {
                if ::std::mem::size_of::<idx_t>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        len12
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        if (if (12 as libc::c_int) < 0 as libc::c_int {
                            if len < 0 as libc::c_int as libc::c_long {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        9223372036854775807 as libc::c_long
                                    }) + 12 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (len
                                        < 9223372036854775807 as libc::c_long
                                            / 12 as libc::c_int as libc::c_long) as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((12 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_long + 9223372036854775807 as libc::c_long
                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        9223372036854775807 as libc::c_long
                                            / -(12 as libc::c_int) as libc::c_long
                                    }) <= -(1 as libc::c_int) as libc::c_long - len)
                                        as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) as libc::c_long
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_long
                                            + (-(9223372036854775807 as libc::c_long)
                                                - 1 as libc::c_long)
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_long
                                            + (-(9223372036854775807 as libc::c_long)
                                                - 1 as libc::c_long)
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) as libc::c_long
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) as libc::c_long
                                                + (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) as libc::c_long
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) as libc::c_long
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_long
                                            + (-(9223372036854775807 as libc::c_long)
                                                - 1 as libc::c_long)) as libc::c_int
                                }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < len
                                                + (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < len
                                            && -(1 as libc::c_int) as libc::c_long
                                                - (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)
                                                < len - 1 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    (((-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long) / 12 as libc::c_int as libc::c_long)
                                        < len) as libc::c_int
                                }
                            }
                        } else {
                            if 12 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if len < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        })
                                            + (-(9223372036854775807 as libc::c_long)
                                                - 1 as libc::c_long)
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            })
                                                + (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            })
                                                + (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        })
                                            + (-(9223372036854775807 as libc::c_long)
                                                - 1 as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    len
                                                })
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        len
                                                    })
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        len
                                                    })
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            })
                                                + (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)) as libc::c_int
                                    }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int as libc::c_long)
                                                < 12 as libc::c_int as libc::c_long
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)) as libc::c_int
                                        } else {
                                            (-(1 as libc::c_int) as libc::c_long
                                                - (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)
                                                < (12 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                                as libc::c_int
                                        }
                                    } else {
                                        ((-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long) / len
                                            < 12 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_long
                                        / 12 as libc::c_int as libc::c_long) < len) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            len12 = (len as libc::c_ulong)
                                .wrapping_mul(12 as libc::c_int as libc::c_ulong)
                                as libc::c_long;
                            1 as libc::c_int
                        } else {
                            len12 = (len as libc::c_ulong)
                                .wrapping_mul(12 as libc::c_int as libc::c_ulong)
                                as libc::c_long;
                            0 as libc::c_int
                        }
                    } else {
                        if (if (12 as libc::c_int) < 0 as libc::c_int {
                            if len < 0 as libc::c_int as libc::c_long {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_ulong)
                                    })
                                        .wrapping_add(12 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((len as libc::c_ulong)
                                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_ulong)
                                            .wrapping_div(12 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((12 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_ulong)
                                            .wrapping_add(
                                                (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_ulong),
                                            )
                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_ulong)
                                            .wrapping_div(-(12 as libc::c_int) as libc::c_ulong)
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_long - len)
                                            as libc::c_ulong) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int) as libc::c_int
                                }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < len + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < len
                                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                as libc::c_long) < len - 1 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    (((0 as libc::c_int / 12 as libc::c_int) as libc::c_long)
                                        < len) as libc::c_int
                                }
                            }
                        } else {
                            if 12 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if len < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    len
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        len
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        len
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int) < 12 as libc::c_int + 0 as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                < 12 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        (0 as libc::c_int as libc::c_long / len
                                            < 12 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_long as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_ulong)
                                        .wrapping_div(12 as libc::c_int as libc::c_ulong)
                                        < len as libc::c_ulong) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            len12 = (len as libc::c_ulong)
                                .wrapping_mul(12 as libc::c_int as libc::c_ulong) as idx_t;
                            1 as libc::c_int
                        } else {
                            len12 = (len as libc::c_ulong)
                                .wrapping_mul(12 as libc::c_int as libc::c_ulong) as idx_t;
                            0 as libc::c_int
                        }
                    }
                } else {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        len12
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        if (if (12 as libc::c_int) < 0 as libc::c_int {
                            if len < 0 as libc::c_int as libc::c_long {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_longlong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_longlong
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                    }) + 12 as libc::c_int as libc::c_longlong
                                }) - 1 as libc::c_int as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    ((len as libc::c_longlong)
                                        < 9223372036854775807 as libc::c_longlong
                                            / 12 as libc::c_int as libc::c_longlong) as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((12 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_longlong
                                            + 9223372036854775807 as libc::c_longlong
                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                            / -(12 as libc::c_int) as libc::c_longlong
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_long - len)
                                            as libc::c_longlong) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_longlong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) as libc::c_longlong
                                        + (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong)
                                }) - 1 as libc::c_int as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_longlong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) + 1 as libc::c_int as libc::c_longlong)
                                        << (::std::mem::size_of::<libc::c_longlong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_longlong)
                                        * 2 as libc::c_int as libc::c_longlong
                                        + 1 as libc::c_int as libc::c_longlong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_longlong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) + 0 as libc::c_int as libc::c_longlong
                                }) < 0 as libc::c_int as libc::c_longlong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) as libc::c_longlong
                                        + (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_longlong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) - 1 as libc::c_int as libc::c_longlong)
                                            < 0 as libc::c_int as libc::c_longlong
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) + 1 as libc::c_int as libc::c_longlong)
                                                << (::std::mem::size_of::<libc::c_longlong>()
                                                    as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_longlong)
                                                * 2 as libc::c_int as libc::c_longlong
                                                + 1 as libc::c_int as libc::c_longlong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as libc::c_int as libc::c_longlong
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_longlong)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)) as libc::c_int
                                }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_longlong)
                                            < len as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < len
                                            && -(1 as libc::c_int) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                < (len - 1 as libc::c_int as libc::c_long)
                                                    as libc::c_longlong) as libc::c_int
                                    }
                                } else {
                                    (((-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong)
                                        / 12 as libc::c_int as libc::c_longlong)
                                        < len as libc::c_longlong) as libc::c_int
                                }
                            }
                        } else {
                            if 12 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if len < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_longlong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) - 1 as libc::c_int as libc::c_longlong)
                                        < 0 as libc::c_int as libc::c_longlong
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_longlong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) + 1 as libc::c_int as libc::c_longlong)
                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_longlong)
                                            * 2 as libc::c_int as libc::c_longlong
                                            + 1 as libc::c_int as libc::c_longlong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_longlong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) + 0 as libc::c_int as libc::c_longlong
                                    }) < 0 as libc::c_int as libc::c_longlong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    len
                                                }) as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as libc::c_int as libc::c_longlong)
                                                < 0 as libc::c_int as libc::c_longlong
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        len
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) + 1 as libc::c_int as libc::c_longlong)
                                                    << (::std::mem::size_of::<libc::c_longlong>()
                                                        as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_longlong)
                                                    * 2 as libc::c_int as libc::c_longlong
                                                    + 1 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        len
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) - 1 as libc::c_int as libc::c_longlong
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_longlong)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as libc::c_int
                                    }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int as libc::c_longlong)
                                                < 12 as libc::c_int as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)) as libc::c_int
                                        } else {
                                            (-(1 as libc::c_int) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                < (12 as libc::c_int - 1 as libc::c_int)
                                                    as libc::c_longlong) as libc::c_int
                                        }
                                    } else {
                                        (((-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong) / len as libc::c_longlong)
                                            < 12 as libc::c_int as libc::c_longlong) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        / 12 as libc::c_int as libc::c_longlong)
                                        < len as libc::c_longlong) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            len12 = (len as libc::c_ulonglong)
                                .wrapping_mul(12 as libc::c_int as libc::c_ulonglong)
                                as libc::c_longlong as idx_t;
                            1 as libc::c_int
                        } else {
                            len12 = (len as libc::c_ulonglong)
                                .wrapping_mul(12 as libc::c_int as libc::c_ulonglong)
                                as libc::c_longlong as idx_t;
                            0 as libc::c_int
                        }
                    } else {
                        if (if (12 as libc::c_int) < 0 as libc::c_int {
                            if len < 0 as libc::c_int as libc::c_long {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulonglong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulonglong
                                    } else {
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                    })
                                        .wrapping_add(12 as libc::c_int as libc::c_ulonglong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                    < 0 as libc::c_int as libc::c_ulonglong
                                {
                                    ((len as libc::c_ulonglong)
                                        < (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(12 as libc::c_int as libc::c_ulonglong))
                                        as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((12 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 12 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) as libc::c_ulonglong)
                                            .wrapping_add(
                                                (9223372036854775807 as libc::c_longlong
                                                    as libc::c_ulonglong)
                                                    .wrapping_mul(2 as libc::c_ulonglong)
                                                    .wrapping_add(1 as libc::c_ulonglong),
                                            )
                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(-(12 as libc::c_int) as libc::c_ulonglong)
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_long - len)
                                            as libc::c_ulonglong) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        12 as libc::c_int
                                    }) + 0 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                12 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    12 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) + 0 as libc::c_int) as libc::c_int
                                }) != 0 && 12 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        len
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < len + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < len
                                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                as libc::c_long) < len - 1 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    (((0 as libc::c_int / 12 as libc::c_int) as libc::c_long)
                                        < len) as libc::c_int
                                }
                            }
                        } else {
                            if 12 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if len < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            len
                                        }) + 0 as libc::c_int as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    len
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        len
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        len
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                len
                                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    }) != 0 && len == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            12 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int) < 12 as libc::c_int + 0 as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                < 12 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        (0 as libc::c_int as libc::c_long / len
                                            < 12 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        as libc::c_ulonglong)
                                        .wrapping_mul(2 as libc::c_ulonglong)
                                        .wrapping_add(1 as libc::c_ulonglong)
                                        .wrapping_div(12 as libc::c_int as libc::c_ulonglong)
                                        < len as libc::c_ulonglong) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            len12 = (len as libc::c_ulonglong)
                                .wrapping_mul(12 as libc::c_int as libc::c_ulonglong)
                                as idx_t;
                            1 as libc::c_int
                        } else {
                            len12 = (len as libc::c_ulonglong)
                                .wrapping_mul(12 as libc::c_int as libc::c_ulonglong)
                                as idx_t;
                            0 as libc::c_int
                        }
                    }
                }
            }
        }
    }) != 0) && len12 < size
    {
        ep = text
            .offset(size as isize)
            .offset(-((11 as libc::c_int as libc::c_long * len) as isize));
        while tp <= ep {
            let mut tp0: *const libc::c_char = tp;
            d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                as libc::c_int;
            tp = tp.offset(d as isize);
            d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                as libc::c_int;
            tp = tp.offset(d as isize);
            if d != 0 as libc::c_int {
                d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                    as libc::c_int;
                tp = tp.offset(d as isize);
                d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                    as libc::c_int;
                tp = tp.offset(d as isize);
                d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                    as libc::c_int;
                tp = tp.offset(d as isize);
                if d != 0 as libc::c_int {
                    d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                        as libc::c_int;
                    tp = tp.offset(d as isize);
                    d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                        as libc::c_int;
                    tp = tp.offset(d as isize);
                    d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                        as libc::c_int;
                    tp = tp.offset(d as isize);
                    if d != 0 as libc::c_int {
                        d = *d1
                            .offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                            as libc::c_int;
                        tp = tp.offset(d as isize);
                        d = *d1
                            .offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
                            as libc::c_int;
                        tp = tp.offset(d as isize);
                        let mut advance_heuristic: libc::c_int = (16 as libc::c_int
                            as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                            ) as libc::c_int;
                        if advance_heuristic as libc::c_long
                            <= tp.offset_from(tp0) as libc::c_long
                        {
                            continue;
                        }
                        tp = tp.offset(-1);
                        tp;
                        tp = memchr_kwset(
                            tp,
                            text.offset(size as isize).offset_from(tp) as libc::c_long,
                            kwset,
                        );
                        if tp.is_null() {
                            return -(1 as libc::c_int) as ptrdiff_t;
                        }
                        tp = tp.offset(1);
                        tp;
                        if ep <= tp {
                            break;
                        }
                    }
                }
            }
            if bm_delta2_search(&mut tp, ep, sp, len, trans, gc1, gc2, d1, kwset) {
                return tp.offset_from(text) as libc::c_long;
            }
        }
    }
    ep = text.offset(size as isize);
    d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize) as libc::c_int;
    while d as libc::c_long <= ep.offset_from(tp) as libc::c_long {
        tp = tp.offset(d as isize);
        d = *d1.offset(U(*tp.offset(-(1 as libc::c_int) as isize)) as isize)
            as libc::c_int;
        if d != 0 as libc::c_int {
            continue;
        }
        if bm_delta2_search(
            &mut tp,
            ep,
            sp,
            len,
            trans,
            gc1,
            gc2,
            0 as *const libc::c_uchar,
            kwset,
        ) {
            return tp.offset_from(text) as libc::c_long;
        }
    }
    return -(1 as libc::c_int) as ptrdiff_t;
}
unsafe extern "C" fn bmexec(
    mut kwset: kwset_t,
    mut text: *const libc::c_char,
    mut size: idx_t,
    mut kwsmatch: *mut kwsmatch,
    mut longest: bool,
) -> ptrdiff_t {
    let mut ret: ptrdiff_t = if !((*kwset).trans).is_null() {
        bmexec_trans(kwset, text, size)
    } else {
        bmexec_trans(kwset, text, size)
    };
    (*kwsmatch).index = 0 as libc::c_int as idx_t;
    (*kwsmatch).offset = ret;
    (*kwsmatch).size = (*kwset).mind;
    return ret;
}
#[inline]
unsafe extern "C" fn acexec_trans(
    mut kwset: kwset_t,
    mut text: *const libc::c_char,
    mut len: idx_t,
    mut kwsmatch: *mut kwsmatch,
    mut longest: bool,
) -> ptrdiff_t {
    let mut trie: *const trie = 0 as *const trie;
    let mut accept: *const trie = 0 as *const trie;
    let mut tp: *const libc::c_char = 0 as *const libc::c_char;
    let mut left: *const libc::c_char = 0 as *const libc::c_char;
    let mut lim: *const libc::c_char = 0 as *const libc::c_char;
    let mut tree: *const tree = 0 as *const tree;
    let mut trans: *const libc::c_char = 0 as *const libc::c_char;
    if len < (*kwset).mind {
        return -(1 as libc::c_int) as ptrdiff_t;
    }
    trans = (*kwset).trans;
    trie = (*kwset).trie;
    lim = text.offset(len as isize);
    tp = text;
    if (*trie).accepting == 0 {
        let mut c: libc::c_uchar = 0;
        let mut gc1: libc::c_int = (*kwset).gc1;
        's_38: loop {
            if gc1 < 0 as libc::c_int {
                loop {
                    let fresh5 = tp;
                    tp = tp.offset(1);
                    c = tr(trans, *fresh5) as libc::c_uchar;
                    trie = (*kwset).next[c as usize];
                    if !trie.is_null() {
                        break;
                    }
                    if tp >= lim {
                        return -(1 as libc::c_int) as ptrdiff_t;
                    }
                }
            } else {
                tp = memchr_kwset(tp, lim.offset_from(tp) as libc::c_long, kwset);
                if tp.is_null() {
                    return -(1 as libc::c_int) as ptrdiff_t;
                }
                let fresh6 = tp;
                tp = tp.offset(1);
                c = tr(trans, *fresh6) as libc::c_uchar;
                trie = (*kwset).next[c as usize];
            }
            's_85: loop {
                if (*trie).accepting != 0 {
                    break 's_38;
                }
                if tp >= lim {
                    return -(1 as libc::c_int) as ptrdiff_t;
                }
                let fresh7 = tp;
                tp = tp.offset(1);
                c = tr(trans, *fresh7) as libc::c_uchar;
                tree = (*trie).links;
                while c as libc::c_int != (*tree).label as libc::c_int {
                    tree = if (c as libc::c_int) < (*tree).label as libc::c_int {
                        (*tree).llink
                    } else {
                        (*tree).rlink
                    };
                    if !tree.is_null() {
                        continue;
                    }
                    trie = (*trie).fail;
                    if trie.is_null() {
                        trie = (*kwset).next[c as usize];
                        if !trie.is_null() {
                            continue 's_85;
                        }
                        if tp >= lim {
                            return -(1 as libc::c_int) as ptrdiff_t;
                        }
                        break 's_85;
                    } else if (*trie).accepting != 0 {
                        tp = tp.offset(-1);
                        tp;
                        break 's_38;
                    } else {
                        tree = (*trie).links;
                    }
                }
                trie = (*tree).trie;
            }
        }
    }
    accept = trie;
    while (*accept).accepting < 0 as libc::c_int as libc::c_long {
        accept = (*accept).fail;
    }
    left = tp.offset(-((*accept).depth as isize));
    if longest {
        while tp < lim {
            let mut accept1: *const trie = 0 as *const trie;
            let mut left1: *const libc::c_char = 0 as *const libc::c_char;
            let fresh8 = tp;
            tp = tp.offset(1);
            let mut c_0: libc::c_uchar = tr(trans, *fresh8) as libc::c_uchar;
            loop {
                tree = (*trie).links;
                while !tree.is_null()
                    && c_0 as libc::c_int != (*tree).label as libc::c_int
                {
                    tree = if (c_0 as libc::c_int) < (*tree).label as libc::c_int {
                        (*tree).llink
                    } else {
                        (*tree).rlink
                    };
                }
                if !(tree.is_null()
                    && {
                        trie = (*trie).fail;
                        !trie.is_null()
                    } && (*accept).depth <= (*trie).depth)
                {
                    break;
                }
            }
            if tree.is_null() {
                break;
            }
            trie = (*tree).trie;
            if (*trie).accepting != 0 {
                accept1 = trie;
                while (*accept1).accepting < 0 as libc::c_int as libc::c_long {
                    accept1 = (*accept1).fail;
                }
                left1 = tp.offset(-((*accept1).depth as isize));
                if left1 <= left {
                    left = left1;
                    accept = accept1;
                }
            }
        }
    }
    (*kwsmatch).index = (*accept).accepting >> 1 as libc::c_int;
    (*kwsmatch).offset = left.offset_from(text) as libc::c_long;
    (*kwsmatch).size = (*accept).depth;
    return left.offset_from(text) as libc::c_long;
}
unsafe extern "C" fn acexec(
    mut kwset: kwset_t,
    mut text: *const libc::c_char,
    mut size: idx_t,
    mut kwsmatch: *mut kwsmatch,
    mut longest: bool,
) -> ptrdiff_t {
    if 0 as libc::c_int as libc::c_long <= size {} else {
        unreachable!();
    };
    return if !((*kwset).trans).is_null() {
        acexec_trans(kwset, text, size, kwsmatch, longest)
    } else {
        acexec_trans(kwset, text, size, kwsmatch, longest)
    };
}
pub unsafe extern "C" fn kwsexec(
    mut kwset: kwset_t,
    mut text: *const libc::c_char,
    mut size: idx_t,
    mut kwsmatch: *mut kwsmatch,
    mut longest: bool,
) -> ptrdiff_t {
    return ((*kwset).kwsexec).unwrap()(kwset, text, size, kwsmatch, longest);
}
pub unsafe extern "C" fn kwsfree(mut kwset: kwset_t) {
    let mut __o: *mut obstack = &mut (*kwset).obstack;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut libc::c_char;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    }
    rpl_free(kwset as *mut libc::c_void);
}
