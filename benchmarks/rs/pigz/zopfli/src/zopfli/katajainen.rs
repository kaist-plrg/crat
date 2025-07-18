use ::libc;
extern "C" {
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub weight: size_t,
    pub tail: *mut Node,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodePool {
    pub next: *mut Node,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
unsafe extern "C" fn InitNode(
    mut weight: size_t,
    mut count: libc::c_int,
    mut tail: *mut Node,
    mut node: *mut Node,
) {
    (*node).weight = weight;
    (*node).count = count;
    (*node).tail = tail;
}
unsafe extern "C" fn BoundaryPM(
    mut lists: *mut [*mut Node; 2],
    mut leaves: *mut Node,
    mut numsymbols: libc::c_int,
    mut pool: *mut NodePool,
    mut index: libc::c_int,
) {
    let mut newchain: *mut Node = 0 as *mut Node;
    let mut oldchain: *mut Node = 0 as *mut Node;
    let mut lastcount: libc::c_int = (*(*lists
        .offset(index as isize))[1 as libc::c_int as usize])
        .count;
    if index == 0 as libc::c_int && lastcount >= numsymbols {
        return;
    }
    let fresh0 = (*pool).next;
    (*pool).next = ((*pool).next).offset(1);
    newchain = fresh0;
    oldchain = (*lists.offset(index as isize))[1 as libc::c_int as usize];
    let ref mut fresh1 = (*lists.offset(index as isize))[0 as libc::c_int as usize];
    *fresh1 = oldchain;
    let ref mut fresh2 = (*lists.offset(index as isize))[1 as libc::c_int as usize];
    *fresh2 = newchain;
    if index == 0 as libc::c_int {
        InitNode(
            (*leaves.offset(lastcount as isize)).weight,
            lastcount + 1 as libc::c_int,
            0 as *mut Node,
            newchain,
        );
    } else {
        let mut sum: size_t = ((*(*lists
            .offset((index - 1 as libc::c_int) as isize))[0 as libc::c_int as usize])
            .weight)
            .wrapping_add(
                (*(*lists
                    .offset(
                        (index - 1 as libc::c_int) as isize,
                    ))[1 as libc::c_int as usize])
                    .weight,
            );
        if lastcount < numsymbols && sum > (*leaves.offset(lastcount as isize)).weight {
            InitNode(
                (*leaves.offset(lastcount as isize)).weight,
                lastcount + 1 as libc::c_int,
                (*oldchain).tail,
                newchain,
            );
        } else {
            InitNode(
                sum,
                lastcount,
                (*lists
                    .offset(
                        (index - 1 as libc::c_int) as isize,
                    ))[1 as libc::c_int as usize],
                newchain,
            );
            BoundaryPM(lists, leaves, numsymbols, pool, index - 1 as libc::c_int);
            BoundaryPM(lists, leaves, numsymbols, pool, index - 1 as libc::c_int);
        }
    };
}
unsafe extern "C" fn BoundaryPMFinal(
    mut lists: *mut [*mut Node; 2],
    mut leaves: *mut Node,
    mut numsymbols: libc::c_int,
    mut pool: *mut NodePool,
    mut index: libc::c_int,
) {
    let mut lastcount: libc::c_int = (*(*lists
        .offset(index as isize))[1 as libc::c_int as usize])
        .count;
    let mut sum: size_t = ((*(*lists
        .offset((index - 1 as libc::c_int) as isize))[0 as libc::c_int as usize])
        .weight)
        .wrapping_add(
            (*(*lists
                .offset((index - 1 as libc::c_int) as isize))[1 as libc::c_int as usize])
                .weight,
        );
    if lastcount < numsymbols && sum > (*leaves.offset(lastcount as isize)).weight {
        let mut newchain: *mut Node = (*pool).next;
        let mut oldchain: *mut Node = (*(*lists
            .offset(index as isize))[1 as libc::c_int as usize])
            .tail;
        let ref mut fresh3 = (*lists.offset(index as isize))[1 as libc::c_int as usize];
        *fresh3 = newchain;
        (*newchain).count = lastcount + 1 as libc::c_int;
        (*newchain).tail = oldchain;
    } else {
        let ref mut fresh4 = (*(*lists
            .offset(index as isize))[1 as libc::c_int as usize])
            .tail;
        *fresh4 = (*lists
            .offset((index - 1 as libc::c_int) as isize))[1 as libc::c_int as usize];
    };
}
unsafe extern "C" fn InitLists(
    mut pool: *mut NodePool,
    mut leaves: *const Node,
    mut maxbits: libc::c_int,
    mut lists: *mut [*mut Node; 2],
) {
    let mut i: libc::c_int = 0;
    let fresh5 = (*pool).next;
    (*pool).next = ((*pool).next).offset(1);
    let mut node0: *mut Node = fresh5;
    let fresh6 = (*pool).next;
    (*pool).next = ((*pool).next).offset(1);
    let mut node1: *mut Node = fresh6;
    InitNode(
        (*leaves.offset(0 as libc::c_int as isize)).weight,
        1 as libc::c_int,
        0 as *mut Node,
        node0,
    );
    InitNode(
        (*leaves.offset(1 as libc::c_int as isize)).weight,
        2 as libc::c_int,
        0 as *mut Node,
        node1,
    );
    i = 0 as libc::c_int;
    while i < maxbits {
        let ref mut fresh7 = (*lists.offset(i as isize))[0 as libc::c_int as usize];
        *fresh7 = node0;
        let ref mut fresh8 = (*lists.offset(i as isize))[1 as libc::c_int as usize];
        *fresh8 = node1;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ExtractBitLengths(
    mut chain: *mut Node,
    mut leaves: *mut Node,
    mut bitlengths: *mut libc::c_uint,
) {
    let mut counts: [libc::c_int; 16] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut end: libc::c_uint = 16 as libc::c_int as libc::c_uint;
    let mut ptr: libc::c_uint = 15 as libc::c_int as libc::c_uint;
    let mut value: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut node: *mut Node = 0 as *mut Node;
    let mut val: libc::c_int = 0;
    node = chain;
    while !node.is_null() {
        end = end.wrapping_sub(1);
        counts[end as usize] = (*node).count;
        node = (*node).tail;
    }
    val = counts[15 as libc::c_int as usize];
    while ptr >= end {
        while val > counts[ptr.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize] {
            *bitlengths
                .offset(
                    (*leaves.offset((val - 1 as libc::c_int) as isize)).count as isize,
                ) = value;
            val -= 1;
            val;
        }
        ptr = ptr.wrapping_sub(1);
        ptr;
        value = value.wrapping_add(1);
        value;
    }
}
unsafe extern "C" fn LeafComparator(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return ((*(a as *const Node)).weight).wrapping_sub((*(b as *const Node)).weight)
        as libc::c_int;
}
pub unsafe extern "C" fn ZopfliLengthLimitedCodeLengths(
    mut frequencies: *const size_t,
    mut n: libc::c_int,
    mut maxbits: libc::c_int,
    mut bitlengths: *mut libc::c_uint,
) -> libc::c_int {
    let mut pool: NodePool = NodePool { next: 0 as *mut Node };
    let mut i: libc::c_int = 0;
    let mut numsymbols: libc::c_int = 0 as libc::c_int;
    let mut numBoundaryPMRuns: libc::c_int = 0;
    let mut nodes: *mut Node = 0 as *mut Node;
    let mut lists: *mut [*mut Node; 2] = 0 as *mut [*mut Node; 2];
    let mut leaves: *mut Node = malloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Node>() as libc::c_ulong),
    ) as *mut Node;
    i = 0 as libc::c_int;
    while i < n {
        *bitlengths.offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n {
        if *frequencies.offset(i as isize) != 0 {
            (*leaves.offset(numsymbols as isize))
                .weight = *frequencies.offset(i as isize);
            (*leaves.offset(numsymbols as isize)).count = i;
            numsymbols += 1;
            numsymbols;
        }
        i += 1;
        i;
    }
    if (1 as libc::c_int) << maxbits < numsymbols {
        free(leaves as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    if numsymbols == 0 as libc::c_int {
        free(leaves as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    if numsymbols == 1 as libc::c_int {
        *bitlengths
            .offset(
                (*leaves.offset(0 as libc::c_int as isize)).count as isize,
            ) = 1 as libc::c_int as libc::c_uint;
        free(leaves as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    if numsymbols == 2 as libc::c_int {
        let ref mut fresh9 = *bitlengths
            .offset((*leaves.offset(0 as libc::c_int as isize)).count as isize);
        *fresh9 = (*fresh9).wrapping_add(1);
        *fresh9;
        let ref mut fresh10 = *bitlengths
            .offset((*leaves.offset(1 as libc::c_int as isize)).count as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
        *fresh10;
        free(leaves as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < numsymbols {
        if (*leaves.offset(i as isize)).weight
            >= (1 as libc::c_int as size_t)
                << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(9 as libc::c_int as libc::c_ulong)
        {
            free(leaves as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        (*leaves.offset(i as isize))
            .weight = (*leaves.offset(i as isize)).weight << 9 as libc::c_int
            | (*leaves.offset(i as isize)).count as libc::c_ulong;
        i += 1;
        i;
    }
    qsort(
        leaves as *mut libc::c_void,
        numsymbols as size_t,
        ::std::mem::size_of::<Node>() as libc::c_ulong,
        Some(
            LeafComparator
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < numsymbols {
        (*leaves.offset(i as isize)).weight >>= 9 as libc::c_int;
        i += 1;
        i;
    }
    if (numsymbols - 1 as libc::c_int) < maxbits {
        maxbits = numsymbols - 1 as libc::c_int;
    }
    nodes = malloc(
        ((maxbits * 2 as libc::c_int * numsymbols) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Node>() as libc::c_ulong),
    ) as *mut Node;
    pool.next = nodes;
    lists = malloc(
        (maxbits as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<[*mut Node; 2]>() as libc::c_ulong),
    ) as *mut [*mut Node; 2];
    InitLists(&mut pool, leaves, maxbits, lists);
    numBoundaryPMRuns = 2 as libc::c_int * numsymbols - 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numBoundaryPMRuns - 1 as libc::c_int {
        BoundaryPM(lists, leaves, numsymbols, &mut pool, maxbits - 1 as libc::c_int);
        i += 1;
        i;
    }
    BoundaryPMFinal(lists, leaves, numsymbols, &mut pool, maxbits - 1 as libc::c_int);
    ExtractBitLengths(
        (*lists
            .offset((maxbits - 1 as libc::c_int) as isize))[1 as libc::c_int as usize],
        leaves,
        bitlengths,
    );
    free(lists as *mut libc::c_void);
    free(leaves as *mut libc::c_void);
    free(nodes as *mut libc::c_void);
    return 0 as libc::c_int;
}
