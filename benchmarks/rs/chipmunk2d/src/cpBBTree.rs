use ::libc;
extern "C" {
    pub type cpHashSet;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn cpMessage(
        condition: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        isError: libc::c_int,
        isHardError: libc::c_int,
        message: *const libc::c_char,
        _: ...
    );
    fn cpArrayNew(size: libc::c_int) -> *mut cpArray;
    fn cpHashSetNew(size: libc::c_int, eqlFunc: cpHashSetEqlFunc) -> *mut cpHashSet;
    fn cpSpatialIndexCollideStatic(
        dynamicIndex: *mut cpSpatialIndex,
        staticIndex: *mut cpSpatialIndex,
        func: cpSpatialIndexQueryFunc,
        data: *mut libc::c_void,
    );
    fn cpArrayPush(arr: *mut cpArray, object: *mut libc::c_void);
    fn cpHashSetEach(
        set: *mut cpHashSet,
        func: cpHashSetIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpHashSetFind(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
    ) -> *const libc::c_void;
    fn cpHashSetRemove(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
    ) -> *const libc::c_void;
    fn cpHashSetInsert(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
        trans: cpHashSetTransFunc,
        data: *mut libc::c_void,
    ) -> *const libc::c_void;
    fn cpHashSetCount(set: *mut cpHashSet) -> libc::c_int;
    fn cpArrayFree(arr: *mut cpArray);
    fn cpArrayFreeEach(
        arr: *mut cpArray,
        freeFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn cpHashSetFree(set: *mut cpHashSet);
    fn cpSpatialIndexInit(
        index: *mut cpSpatialIndex,
        klass_0: *mut cpSpatialIndexClass,
        bbfunc: cpSpatialIndexBBFunc,
        staticIndex: *mut cpSpatialIndex,
    ) -> *mut cpSpatialIndex;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type cpFloat = libc::c_double;
pub type cpHashValue = uintptr_t;
pub type cpCollisionID = uint32_t;
pub type cpBool = libc::c_uchar;
pub type cpTimestamp = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArray {
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub arr: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpatialIndex {
    pub klass: *mut cpSpatialIndexClass,
    pub bbfunc: cpSpatialIndexBBFunc,
    pub staticIndex: *mut cpSpatialIndex,
    pub dynamicIndex: *mut cpSpatialIndex,
}
pub type cpSpatialIndexBBFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cpBB,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpatialIndexClass {
    pub destroy: cpSpatialIndexDestroyImpl,
    pub count: cpSpatialIndexCountImpl,
    pub each: cpSpatialIndexEachImpl,
    pub contains: cpSpatialIndexContainsImpl,
    pub insert: cpSpatialIndexInsertImpl,
    pub remove: cpSpatialIndexRemoveImpl,
    pub reindex: cpSpatialIndexReindexImpl,
    pub reindexObject: cpSpatialIndexReindexObjectImpl,
    pub reindexQuery: cpSpatialIndexReindexQueryImpl,
    pub query: cpSpatialIndexQueryImpl,
    pub segmentQuery: cpSpatialIndexSegmentQueryImpl,
}
pub type cpSpatialIndexSegmentQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        *mut libc::c_void,
        cpVect,
        cpVect,
        cpFloat,
        cpSpatialIndexSegmentQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexSegmentQueryFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> cpFloat,
>;
pub type cpSpatialIndexQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        *mut libc::c_void,
        cpBB,
        cpSpatialIndexQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexQueryFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        cpCollisionID,
        *mut libc::c_void,
    ) -> cpCollisionID,
>;
pub type cpSpatialIndexReindexQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        cpSpatialIndexQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexReindexObjectImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexReindexImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> (),
>;
pub type cpSpatialIndexRemoveImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexInsertImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexContainsImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> cpBool,
>;
pub type cpSpatialIndexEachImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        cpSpatialIndexIteratorFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexIteratorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpSpatialIndexCountImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> libc::c_int,
>;
pub type cpSpatialIndexDestroyImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBBTree {
    pub spatialIndex: cpSpatialIndex,
    pub velocityFunc: cpBBTreeVelocityFunc,
    pub leaves: *mut cpHashSet,
    pub root: *mut Node,
    pub pooledNodes: *mut Node,
    pub pooledPairs: *mut Pair,
    pub allocatedBuffers: *mut cpArray,
    pub stamp: cpTimestamp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pair {
    pub a: Thread,
    pub b: Thread,
    pub id: cpCollisionID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Thread {
    pub prev: *mut Pair,
    pub leaf: *mut Node,
    pub next: *mut Pair,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub obj: *mut libc::c_void,
    pub bb: cpBB,
    pub parent: *mut Node,
    pub node: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub children: C2RustUnnamed_1,
    pub leaf: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub stamp: cpTimestamp,
    pub pairs: *mut Pair,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub a: *mut Node,
    pub b: *mut Node,
}
pub type cpBBTreeVelocityFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cpVect,
>;
pub type cpHashSetEqlFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkContext {
    pub tree: *mut cpBBTree,
    pub staticRoot: *mut Node,
    pub func: cpSpatialIndexQueryFunc,
    pub data: *mut libc::c_void,
}
pub type cpHashSetIteratorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpHashSetTransFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eachContext {
    pub func: cpSpatialIndexIteratorFunc,
    pub data: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    let mut v: cpVect = {
        let mut init = cpVect { x: x, y: y };
        init
    };
    return v;
}
#[inline]
unsafe extern "C" fn cpvsub(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x - v2.x, v1.y - v2.y);
}
#[inline]
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
#[inline]
unsafe extern "C" fn cpBBNew(l: cpFloat, b: cpFloat, r: cpFloat, t: cpFloat) -> cpBB {
    let mut bb: cpBB = {
        let mut init = cpBB { l: l, b: b, r: r, t: t };
        init
    };
    return bb;
}
#[inline]
unsafe extern "C" fn cpBBIntersects(a: cpBB, b: cpBB) -> cpBool {
    return (a.l <= b.r && b.l <= a.r && a.b <= b.t && b.b <= a.t) as libc::c_int
        as cpBool;
}
#[inline]
unsafe extern "C" fn cpBBContainsBB(bb: cpBB, other: cpBB) -> cpBool {
    return (bb.l <= other.l && bb.r >= other.r && bb.b <= other.b && bb.t >= other.t)
        as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn cpBBMerge(a: cpBB, b: cpBB) -> cpBB {
    return cpBBNew(
        cpfmin(a.l, b.l),
        cpfmin(a.b, b.b),
        cpfmax(a.r, b.r),
        cpfmax(a.t, b.t),
    );
}
#[inline]
unsafe extern "C" fn cpBBArea(mut bb: cpBB) -> cpFloat {
    return (bb.r - bb.l) * (bb.t - bb.b);
}
#[inline]
unsafe extern "C" fn cpBBMergedArea(mut a: cpBB, mut b: cpBB) -> cpFloat {
    return (cpfmax(a.r, b.r) - cpfmin(a.l, b.l)) * (cpfmax(a.t, b.t) - cpfmin(a.b, b.b));
}
#[inline]
unsafe extern "C" fn cpBBSegmentQuery(
    mut bb: cpBB,
    mut a: cpVect,
    mut b: cpVect,
) -> cpFloat {
    let mut delta: cpVect = cpvsub(b, a);
    let mut tmin: cpFloat = -::std::f32::INFINITY as cpFloat;
    let mut tmax: cpFloat = ::std::f32::INFINITY as cpFloat;
    if delta.x == 0.0f32 as libc::c_double {
        if a.x < bb.l || bb.r < a.x {
            return ::std::f32::INFINITY as cpFloat;
        }
    } else {
        let mut t1: cpFloat = (bb.l - a.x) / delta.x;
        let mut t2: cpFloat = (bb.r - a.x) / delta.x;
        tmin = cpfmax(tmin, cpfmin(t1, t2));
        tmax = cpfmin(tmax, cpfmax(t1, t2));
    }
    if delta.y == 0.0f32 as libc::c_double {
        if a.y < bb.b || bb.t < a.y {
            return ::std::f32::INFINITY as cpFloat;
        }
    } else {
        let mut t1_0: cpFloat = (bb.b - a.y) / delta.y;
        let mut t2_0: cpFloat = (bb.t - a.y) / delta.y;
        tmin = cpfmax(tmin, cpfmin(t1_0, t2_0));
        tmax = cpfmin(tmax, cpfmax(t1_0, t2_0));
    }
    if tmin <= tmax && 0.0f32 as libc::c_double <= tmax
        && tmin <= 1.0f32 as libc::c_double
    {
        return cpfmax(tmin, 0.0f32 as cpFloat)
    } else {
        return ::std::f32::INFINITY as cpFloat
    };
}
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfabs(mut f: cpFloat) -> cpFloat {
    return if f < 0 as libc::c_int as libc::c_double { -f } else { f };
}
#[inline]
unsafe extern "C" fn GetBB(mut tree: *mut cpBBTree, mut obj: *mut libc::c_void) -> cpBB {
    let mut bb: cpBB = ((*tree).spatialIndex.bbfunc).unwrap()(obj);
    let mut velocityFunc: cpBBTreeVelocityFunc = (*tree).velocityFunc;
    if velocityFunc.is_some() {
        let mut coef: cpFloat = 0.1f32 as cpFloat;
        let mut x: cpFloat = (bb.r - bb.l) * coef;
        let mut y: cpFloat = (bb.t - bb.b) * coef;
        let mut v: cpVect = cpvmult(velocityFunc.unwrap()(obj), 0.1f32 as cpFloat);
        return cpBBNew(
            bb.l + cpfmin(-x, v.x),
            bb.b + cpfmin(-y, v.y),
            bb.r + cpfmax(x, v.x),
            bb.t + cpfmax(y, v.y),
        );
    } else {
        return bb
    };
}
#[inline]
unsafe extern "C" fn GetTree(mut index: *mut cpSpatialIndex) -> *mut cpBBTree {
    return if !index.is_null() && (*index).klass == Klass() {
        index as *mut cpBBTree
    } else {
        0 as *mut cpBBTree
    };
}
#[inline]
unsafe extern "C" fn GetRootIfTree(mut index: *mut cpSpatialIndex) -> *mut Node {
    return if !index.is_null() && (*index).klass == Klass() {
        (*(index as *mut cpBBTree)).root
    } else {
        0 as *mut Node
    };
}
#[inline]
unsafe extern "C" fn GetMasterTree(mut tree: *mut cpBBTree) -> *mut cpBBTree {
    let mut dynamicTree: *mut cpBBTree = GetTree((*tree).spatialIndex.dynamicIndex);
    return if !dynamicTree.is_null() { dynamicTree } else { tree };
}
#[inline]
unsafe extern "C" fn IncrementStamp(mut tree: *mut cpBBTree) {
    let mut dynamicTree: *mut cpBBTree = GetTree((*tree).spatialIndex.dynamicIndex);
    if !dynamicTree.is_null() {
        (*dynamicTree).stamp = ((*dynamicTree).stamp).wrapping_add(1);
        (*dynamicTree).stamp;
    } else {
        (*tree).stamp = ((*tree).stamp).wrapping_add(1);
        (*tree).stamp;
    };
}
unsafe extern "C" fn PairRecycle(mut tree: *mut cpBBTree, mut pair: *mut Pair) {
    tree = GetMasterTree(tree);
    (*pair).a.next = (*tree).pooledPairs;
    (*tree).pooledPairs = pair;
}
unsafe extern "C" fn PairFromPool(mut tree: *mut cpBBTree) -> *mut Pair {
    tree = GetMasterTree(tree);
    let mut pair: *mut Pair = (*tree).pooledPairs;
    if !pair.is_null() {
        (*tree).pooledPairs = (*pair).a.next;
        return pair;
    } else {
        let mut count: libc::c_int = ((32 as libc::c_int * 1024 as libc::c_int)
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Pair>() as libc::c_ulong) as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../../src/cpBBTree.c\0" as *const u8 as *const libc::c_char,
                157 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        let mut buffer: *mut Pair = calloc(
            1 as libc::c_int as libc::c_ulong,
            (32 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
        ) as *mut Pair;
        cpArrayPush((*tree).allocatedBuffers, buffer as *mut libc::c_void);
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < count {
            PairRecycle(tree, buffer.offset(i as isize));
            i += 1;
            i;
        }
        return buffer;
    };
}
#[inline]
unsafe extern "C" fn ThreadUnlink(mut thread: Thread) {
    let mut next: *mut Pair = thread.next;
    let mut prev: *mut Pair = thread.prev;
    if !next.is_null() {
        if (*next).a.leaf == thread.leaf {
            (*next).a.prev = prev;
        } else {
            (*next).b.prev = prev;
        }
    }
    if !prev.is_null() {
        if (*prev).a.leaf == thread.leaf {
            (*prev).a.next = next;
        } else {
            (*prev).b.next = next;
        }
    } else {
        (*thread.leaf).node.leaf.pairs = next;
    };
}
unsafe extern "C" fn PairsClear(mut leaf: *mut Node, mut tree: *mut cpBBTree) {
    let mut pair: *mut Pair = (*leaf).node.leaf.pairs;
    (*leaf).node.leaf.pairs = 0 as *mut Pair;
    while !pair.is_null() {
        if (*pair).a.leaf == leaf {
            let mut next: *mut Pair = (*pair).a.next;
            ThreadUnlink((*pair).b);
            PairRecycle(tree, pair);
            pair = next;
        } else {
            let mut next_0: *mut Pair = (*pair).b.next;
            ThreadUnlink((*pair).a);
            PairRecycle(tree, pair);
            pair = next_0;
        }
    }
}
unsafe extern "C" fn PairInsert(
    mut a: *mut Node,
    mut b: *mut Node,
    mut tree: *mut cpBBTree,
) {
    let mut nextA: *mut Pair = (*a).node.leaf.pairs;
    let mut nextB: *mut Pair = (*b).node.leaf.pairs;
    let mut pair: *mut Pair = PairFromPool(tree);
    let mut temp: Pair = {
        let mut init = Pair {
            a: {
                let mut init = Thread {
                    prev: 0 as *mut Pair,
                    leaf: a,
                    next: nextA,
                };
                init
            },
            b: {
                let mut init = Thread {
                    prev: 0 as *mut Pair,
                    leaf: b,
                    next: nextB,
                };
                init
            },
            id: 0 as libc::c_int as cpCollisionID,
        };
        init
    };
    (*b).node.leaf.pairs = pair;
    (*a).node.leaf.pairs = (*b).node.leaf.pairs;
    *pair = temp;
    if !nextA.is_null() {
        if (*nextA).a.leaf == a {
            (*nextA).a.prev = pair;
        } else {
            (*nextA).b.prev = pair;
        }
    }
    if !nextB.is_null() {
        if (*nextB).a.leaf == b {
            (*nextB).a.prev = pair;
        } else {
            (*nextB).b.prev = pair;
        }
    }
}
unsafe extern "C" fn NodeRecycle(mut tree: *mut cpBBTree, mut node: *mut Node) {
    (*node).parent = (*tree).pooledNodes;
    (*tree).pooledNodes = node;
}
unsafe extern "C" fn NodeFromPool(mut tree: *mut cpBBTree) -> *mut Node {
    let mut node: *mut Node = (*tree).pooledNodes;
    if !node.is_null() {
        (*tree).pooledNodes = (*node).parent;
        return node;
    } else {
        let mut count: libc::c_int = ((32 as libc::c_int * 1024 as libc::c_int)
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Node>() as libc::c_ulong) as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../../src/cpBBTree.c\0" as *const u8 as *const libc::c_char,
                246 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        let mut buffer: *mut Node = calloc(
            1 as libc::c_int as libc::c_ulong,
            (32 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
        ) as *mut Node;
        cpArrayPush((*tree).allocatedBuffers, buffer as *mut libc::c_void);
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < count {
            NodeRecycle(tree, buffer.offset(i as isize));
            i += 1;
            i;
        }
        return buffer;
    };
}
#[inline]
unsafe extern "C" fn NodeSetA(mut node: *mut Node, mut value: *mut Node) {
    (*node).node.children.a = value;
    (*value).parent = node;
}
#[inline]
unsafe extern "C" fn NodeSetB(mut node: *mut Node, mut value: *mut Node) {
    (*node).node.children.b = value;
    (*value).parent = node;
}
unsafe extern "C" fn NodeNew(
    mut tree: *mut cpBBTree,
    mut a: *mut Node,
    mut b: *mut Node,
) -> *mut Node {
    let mut node: *mut Node = NodeFromPool(tree);
    (*node).obj = 0 as *mut libc::c_void;
    (*node).bb = cpBBMerge((*a).bb, (*b).bb);
    (*node).parent = 0 as *mut Node;
    NodeSetA(node, a);
    NodeSetB(node, b);
    return node;
}
#[inline]
unsafe extern "C" fn NodeIsLeaf(mut node: *mut Node) -> cpBool {
    return ((*node).obj != 0 as *mut libc::c_void) as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn NodeOther(mut node: *mut Node, mut child: *mut Node) -> *mut Node {
    return if (*node).node.children.a == child {
        (*node).node.children.b
    } else {
        (*node).node.children.a
    };
}
#[inline]
unsafe extern "C" fn NodeReplaceChild(
    mut parent: *mut Node,
    mut child: *mut Node,
    mut value: *mut Node,
    mut tree: *mut cpBBTree,
) {
    if (*parent).node.children.a == child {
        NodeRecycle(tree, (*parent).node.children.a);
        NodeSetA(parent, value);
    } else {
        NodeRecycle(tree, (*parent).node.children.b);
        NodeSetB(parent, value);
    }
    let mut node: *mut Node = parent;
    while !node.is_null() {
        (*node)
            .bb = cpBBMerge(
            (*(*node).node.children.a).bb,
            (*(*node).node.children.b).bb,
        );
        node = (*node).parent;
    }
}
#[inline]
unsafe extern "C" fn cpBBProximity(mut a: cpBB, mut b: cpBB) -> cpFloat {
    return cpfabs(a.l + a.r - b.l - b.r) + cpfabs(a.b + a.t - b.b - b.t);
}
unsafe extern "C" fn SubtreeInsert(
    mut subtree: *mut Node,
    mut leaf: *mut Node,
    mut tree: *mut cpBBTree,
) -> *mut Node {
    if subtree.is_null() {
        return leaf
    } else if NodeIsLeaf(subtree) != 0 {
        return NodeNew(tree, leaf, subtree)
    } else {
        let mut cost_a: cpFloat = cpBBArea((*(*subtree).node.children.b).bb)
            + cpBBMergedArea((*(*subtree).node.children.a).bb, (*leaf).bb);
        let mut cost_b: cpFloat = cpBBArea((*(*subtree).node.children.a).bb)
            + cpBBMergedArea((*(*subtree).node.children.b).bb, (*leaf).bb);
        if cost_a == cost_b {
            cost_a = cpBBProximity((*(*subtree).node.children.a).bb, (*leaf).bb);
            cost_b = cpBBProximity((*(*subtree).node.children.b).bb, (*leaf).bb);
        }
        if cost_b < cost_a {
            NodeSetB(subtree, SubtreeInsert((*subtree).node.children.b, leaf, tree));
        } else {
            NodeSetA(subtree, SubtreeInsert((*subtree).node.children.a, leaf, tree));
        }
        (*subtree).bb = cpBBMerge((*subtree).bb, (*leaf).bb);
        return subtree;
    };
}
unsafe extern "C" fn SubtreeQuery(
    mut subtree: *mut Node,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    if cpBBIntersects((*subtree).bb, bb) != 0 {
        if NodeIsLeaf(subtree) != 0 {
            func.unwrap()(obj, (*subtree).obj, 0 as libc::c_int as cpCollisionID, data);
        } else {
            SubtreeQuery((*subtree).node.children.a, obj, bb, func, data);
            SubtreeQuery((*subtree).node.children.b, obj, bb, func, data);
        }
    }
}
unsafe extern "C" fn SubtreeSegmentQuery(
    mut subtree: *mut Node,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: cpSpatialIndexSegmentQueryFunc,
    mut data: *mut libc::c_void,
) -> cpFloat {
    if NodeIsLeaf(subtree) != 0 {
        return func.unwrap()(obj, (*subtree).obj, data)
    } else {
        let mut t_a: cpFloat = cpBBSegmentQuery((*(*subtree).node.children.a).bb, a, b);
        let mut t_b: cpFloat = cpBBSegmentQuery((*(*subtree).node.children.b).bb, a, b);
        if t_a < t_b {
            if t_a < t_exit {
                t_exit = cpfmin(
                    t_exit,
                    SubtreeSegmentQuery(
                        (*subtree).node.children.a,
                        obj,
                        a,
                        b,
                        t_exit,
                        func,
                        data,
                    ),
                );
            }
            if t_b < t_exit {
                t_exit = cpfmin(
                    t_exit,
                    SubtreeSegmentQuery(
                        (*subtree).node.children.b,
                        obj,
                        a,
                        b,
                        t_exit,
                        func,
                        data,
                    ),
                );
            }
        } else {
            if t_b < t_exit {
                t_exit = cpfmin(
                    t_exit,
                    SubtreeSegmentQuery(
                        (*subtree).node.children.b,
                        obj,
                        a,
                        b,
                        t_exit,
                        func,
                        data,
                    ),
                );
            }
            if t_a < t_exit {
                t_exit = cpfmin(
                    t_exit,
                    SubtreeSegmentQuery(
                        (*subtree).node.children.a,
                        obj,
                        a,
                        b,
                        t_exit,
                        func,
                        data,
                    ),
                );
            }
        }
        return t_exit;
    };
}
unsafe extern "C" fn SubtreeRecycle(mut tree: *mut cpBBTree, mut node: *mut Node) {
    if NodeIsLeaf(node) == 0 {
        SubtreeRecycle(tree, (*node).node.children.a);
        SubtreeRecycle(tree, (*node).node.children.b);
        NodeRecycle(tree, node);
    }
}
#[inline]
unsafe extern "C" fn SubtreeRemove(
    mut subtree: *mut Node,
    mut leaf: *mut Node,
    mut tree: *mut cpBBTree,
) -> *mut Node {
    if leaf == subtree {
        return 0 as *mut Node
    } else {
        let mut parent: *mut Node = (*leaf).parent;
        if parent == subtree {
            let mut other: *mut Node = NodeOther(subtree, leaf);
            (*other).parent = (*subtree).parent;
            NodeRecycle(tree, subtree);
            return other;
        } else {
            NodeReplaceChild((*parent).parent, parent, NodeOther(parent, leaf), tree);
            return subtree;
        }
    };
}
unsafe extern "C" fn MarkLeafQuery(
    mut subtree: *mut Node,
    mut leaf: *mut Node,
    mut left: cpBool,
    mut context: *mut MarkContext,
) {
    if cpBBIntersects((*leaf).bb, (*subtree).bb) != 0 {
        if NodeIsLeaf(subtree) != 0 {
            if left != 0 {
                PairInsert(leaf, subtree, (*context).tree);
            } else {
                if (*subtree).node.leaf.stamp < (*leaf).node.leaf.stamp {
                    PairInsert(subtree, leaf, (*context).tree);
                }
                ((*context).func)
                    .unwrap()(
                    (*leaf).obj,
                    (*subtree).obj,
                    0 as libc::c_int as cpCollisionID,
                    (*context).data,
                );
            }
        } else {
            MarkLeafQuery((*subtree).node.children.a, leaf, left, context);
            MarkLeafQuery((*subtree).node.children.b, leaf, left, context);
        }
    }
}
unsafe extern "C" fn MarkLeaf(mut leaf: *mut Node, mut context: *mut MarkContext) {
    let mut tree: *mut cpBBTree = (*context).tree;
    if (*leaf).node.leaf.stamp == (*GetMasterTree(tree)).stamp {
        let mut staticRoot: *mut Node = (*context).staticRoot;
        if !staticRoot.is_null() {
            MarkLeafQuery(staticRoot, leaf, 0 as libc::c_int as cpBool, context);
        }
        let mut node: *mut Node = leaf;
        while !((*node).parent).is_null() {
            if node == (*(*node).parent).node.children.a {
                MarkLeafQuery(
                    (*(*node).parent).node.children.b,
                    leaf,
                    1 as libc::c_int as cpBool,
                    context,
                );
            } else {
                MarkLeafQuery(
                    (*(*node).parent).node.children.a,
                    leaf,
                    0 as libc::c_int as cpBool,
                    context,
                );
            }
            node = (*node).parent;
        }
    } else {
        let mut pair: *mut Pair = (*leaf).node.leaf.pairs;
        while !pair.is_null() {
            if leaf == (*pair).b.leaf {
                (*pair)
                    .id = ((*context).func)
                    .unwrap()(
                    (*(*pair).a.leaf).obj,
                    (*leaf).obj,
                    (*pair).id,
                    (*context).data,
                );
                pair = (*pair).b.next;
            } else {
                pair = (*pair).a.next;
            }
        }
    };
}
unsafe extern "C" fn MarkSubtree(mut subtree: *mut Node, mut context: *mut MarkContext) {
    if NodeIsLeaf(subtree) != 0 {
        MarkLeaf(subtree, context);
    } else {
        MarkSubtree((*subtree).node.children.a, context);
        MarkSubtree((*subtree).node.children.b, context);
    };
}
unsafe extern "C" fn LeafNew(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
) -> *mut Node {
    let mut node: *mut Node = NodeFromPool(tree);
    (*node).obj = obj;
    (*node).bb = GetBB(tree, obj);
    (*node).parent = 0 as *mut Node;
    (*node).node.leaf.stamp = 0 as libc::c_int as cpTimestamp;
    (*node).node.leaf.pairs = 0 as *mut Pair;
    return node;
}
unsafe extern "C" fn LeafUpdate(mut leaf: *mut Node, mut tree: *mut cpBBTree) -> cpBool {
    let mut root: *mut Node = (*tree).root;
    let mut bb: cpBB = ((*tree).spatialIndex.bbfunc).unwrap()((*leaf).obj);
    if cpBBContainsBB((*leaf).bb, bb) == 0 {
        (*leaf).bb = GetBB(tree, (*leaf).obj);
        root = SubtreeRemove(root, leaf, tree);
        (*tree).root = SubtreeInsert(root, leaf, tree);
        PairsClear(leaf, tree);
        (*leaf).node.leaf.stamp = (*GetMasterTree(tree)).stamp;
        return 1 as libc::c_int as cpBool;
    } else {
        return 0 as libc::c_int as cpBool
    };
}
unsafe extern "C" fn VoidQueryFunc(
    mut obj1: *mut libc::c_void,
    mut obj2: *mut libc::c_void,
    mut id: cpCollisionID,
    mut data: *mut libc::c_void,
) -> cpCollisionID {
    return id;
}
unsafe extern "C" fn LeafAddPairs(mut leaf: *mut Node, mut tree: *mut cpBBTree) {
    let mut dynamicIndex: *mut cpSpatialIndex = (*tree).spatialIndex.dynamicIndex;
    if !dynamicIndex.is_null() {
        let mut dynamicRoot: *mut Node = GetRootIfTree(dynamicIndex);
        if !dynamicRoot.is_null() {
            let mut dynamicTree: *mut cpBBTree = GetTree(dynamicIndex);
            let mut context: MarkContext = {
                let mut init = MarkContext {
                    tree: dynamicTree,
                    staticRoot: 0 as *mut Node,
                    func: None,
                    data: 0 as *mut libc::c_void,
                };
                init
            };
            MarkLeafQuery(dynamicRoot, leaf, 1 as libc::c_int as cpBool, &mut context);
        }
    } else {
        let mut staticRoot: *mut Node = GetRootIfTree((*tree).spatialIndex.staticIndex);
        let mut context_0: MarkContext = {
            let mut init = MarkContext {
                tree: tree,
                staticRoot: staticRoot,
                func: Some(
                    VoidQueryFunc
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            cpCollisionID,
                            *mut libc::c_void,
                        ) -> cpCollisionID,
                ),
                data: 0 as *mut libc::c_void,
            };
            init
        };
        MarkLeaf(leaf, &mut context_0);
    };
}
pub unsafe extern "C" fn cpBBTreeAlloc() -> *mut cpBBTree {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpBBTree>() as libc::c_ulong,
    ) as *mut cpBBTree;
}
unsafe extern "C" fn leafSetEql(
    mut obj: *mut libc::c_void,
    mut node: *mut Node,
) -> libc::c_int {
    return (obj == (*node).obj) as libc::c_int;
}
unsafe extern "C" fn leafSetTrans(
    mut obj: *mut libc::c_void,
    mut tree: *mut cpBBTree,
) -> *mut libc::c_void {
    return LeafNew(tree, obj, ((*tree).spatialIndex.bbfunc).unwrap()(obj))
        as *mut libc::c_void;
}
pub unsafe extern "C" fn cpBBTreeInit(
    mut tree: *mut cpBBTree,
    mut bbfunc: cpSpatialIndexBBFunc,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    cpSpatialIndexInit(tree as *mut cpSpatialIndex, Klass(), bbfunc, staticIndex);
    (*tree).velocityFunc = None;
    (*tree)
        .leaves = cpHashSetNew(
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut Node) -> libc::c_int>,
            cpHashSetEqlFunc,
        >(
            Some(
                leafSetEql
                    as unsafe extern "C" fn(*mut libc::c_void, *mut Node) -> libc::c_int,
            ),
        ),
    );
    (*tree).root = 0 as *mut Node;
    (*tree).pooledNodes = 0 as *mut Node;
    (*tree).allocatedBuffers = cpArrayNew(0 as libc::c_int);
    (*tree).stamp = 0 as libc::c_int as cpTimestamp;
    return tree as *mut cpSpatialIndex;
}
pub unsafe extern "C" fn cpBBTreeSetVelocityFunc(
    mut index: *mut cpSpatialIndex,
    mut func: cpBBTreeVelocityFunc,
) {
    if (*index).klass != Klass() {
        return;
    }
    let ref mut fresh0 = (*(index as *mut cpBBTree)).velocityFunc;
    *fresh0 = func;
}
pub unsafe extern "C" fn cpBBTreeNew(
    mut bbfunc: cpSpatialIndexBBFunc,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    return cpBBTreeInit(cpBBTreeAlloc(), bbfunc, staticIndex);
}
unsafe extern "C" fn cpBBTreeDestroy(mut tree: *mut cpBBTree) {
    cpHashSetFree((*tree).leaves);
    if !((*tree).allocatedBuffers).is_null() {
        cpArrayFreeEach(
            (*tree).allocatedBuffers,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    cpArrayFree((*tree).allocatedBuffers);
}
unsafe extern "C" fn cpBBTreeInsert(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut leaf: *mut Node = cpHashSetInsert(
        (*tree).leaves,
        hashid,
        obj,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cpBBTree,
                ) -> *mut libc::c_void,
            >,
            cpHashSetTransFunc,
        >(
            Some(
                leafSetTrans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cpBBTree,
                    ) -> *mut libc::c_void,
            ),
        ),
        tree as *mut libc::c_void,
    ) as *mut Node;
    let mut root: *mut Node = (*tree).root;
    (*tree).root = SubtreeInsert(root, leaf, tree);
    (*leaf).node.leaf.stamp = (*GetMasterTree(tree)).stamp;
    LeafAddPairs(leaf, tree);
    IncrementStamp(tree);
}
unsafe extern "C" fn cpBBTreeRemove(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut leaf: *mut Node = cpHashSetRemove((*tree).leaves, hashid, obj) as *mut Node;
    (*tree).root = SubtreeRemove((*tree).root, leaf, tree);
    PairsClear(leaf, tree);
    NodeRecycle(tree, leaf);
}
unsafe extern "C" fn cpBBTreeContains(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) -> cpBool {
    return (cpHashSetFind((*tree).leaves, hashid, obj)
        != 0 as *mut libc::c_void as *const libc::c_void) as libc::c_int as cpBool;
}
unsafe extern "C" fn LeafUpdateWrap(mut leaf: *mut Node, mut tree: *mut cpBBTree) {
    LeafUpdate(leaf, tree);
}
unsafe extern "C" fn cpBBTreeReindexQuery(
    mut tree: *mut cpBBTree,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    if ((*tree).root).is_null() {
        return;
    }
    cpHashSetEach(
        (*tree).leaves,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Node, *mut cpBBTree) -> ()>,
            cpHashSetIteratorFunc,
        >(Some(LeafUpdateWrap as unsafe extern "C" fn(*mut Node, *mut cpBBTree) -> ())),
        tree as *mut libc::c_void,
    );
    let mut staticIndex: *mut cpSpatialIndex = (*tree).spatialIndex.staticIndex;
    let mut staticRoot: *mut Node = if !staticIndex.is_null()
        && (*staticIndex).klass == Klass()
    {
        (*(staticIndex as *mut cpBBTree)).root
    } else {
        0 as *mut Node
    };
    let mut context: MarkContext = {
        let mut init = MarkContext {
            tree: tree,
            staticRoot: staticRoot,
            func: func,
            data: data,
        };
        init
    };
    MarkSubtree((*tree).root, &mut context);
    if !staticIndex.is_null() && staticRoot.is_null() {
        cpSpatialIndexCollideStatic(
            tree as *mut cpSpatialIndex,
            staticIndex,
            func,
            data,
        );
    }
    IncrementStamp(tree);
}
unsafe extern "C" fn cpBBTreeReindex(mut tree: *mut cpBBTree) {
    cpBBTreeReindexQuery(
        tree,
        Some(
            VoidQueryFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn cpBBTreeReindexObject(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut leaf: *mut Node = cpHashSetFind((*tree).leaves, hashid, obj) as *mut Node;
    if !leaf.is_null() {
        if LeafUpdate(leaf, tree) != 0 {
            LeafAddPairs(leaf, tree);
        }
        IncrementStamp(tree);
    }
}
unsafe extern "C" fn cpBBTreeSegmentQuery(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: cpSpatialIndexSegmentQueryFunc,
    mut data: *mut libc::c_void,
) {
    let mut root: *mut Node = (*tree).root;
    if !root.is_null() {
        SubtreeSegmentQuery(root, obj, a, b, t_exit, func, data);
    }
}
unsafe extern "C" fn cpBBTreeQuery(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    if !((*tree).root).is_null() {
        SubtreeQuery((*tree).root, obj, bb, func, data);
    }
}
unsafe extern "C" fn cpBBTreeCount(mut tree: *mut cpBBTree) -> libc::c_int {
    return cpHashSetCount((*tree).leaves);
}
unsafe extern "C" fn each_helper(mut node: *mut Node, mut context: *mut eachContext) {
    ((*context).func).unwrap()((*node).obj, (*context).data);
}
unsafe extern "C" fn cpBBTreeEach(
    mut tree: *mut cpBBTree,
    mut func: cpSpatialIndexIteratorFunc,
    mut data: *mut libc::c_void,
) {
    let mut context: eachContext = {
        let mut init = eachContext {
            func: func,
            data: data,
        };
        init
    };
    cpHashSetEach(
        (*tree).leaves,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Node, *mut eachContext) -> ()>,
            cpHashSetIteratorFunc,
        >(Some(each_helper as unsafe extern "C" fn(*mut Node, *mut eachContext) -> ())),
        &mut context as *mut eachContext as *mut libc::c_void,
    );
}
static mut klass: cpSpatialIndexClass = unsafe {
    {
        let mut init = cpSpatialIndexClass {
            destroy: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpBBTree) -> ()>,
                cpSpatialIndexDestroyImpl,
            >(Some(cpBBTreeDestroy as unsafe extern "C" fn(*mut cpBBTree) -> ())),
            count: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpBBTree) -> libc::c_int>,
                cpSpatialIndexCountImpl,
            >(Some(cpBBTreeCount as unsafe extern "C" fn(*mut cpBBTree) -> libc::c_int)),
            each: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        cpSpatialIndexIteratorFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexEachImpl,
            >(
                Some(
                    cpBBTreeEach
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            cpSpatialIndexIteratorFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            contains: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> cpBool,
                >,
                cpSpatialIndexContainsImpl,
            >(
                Some(
                    cpBBTreeContains
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> cpBool,
                ),
            ),
            insert: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexInsertImpl,
            >(
                Some(
                    cpBBTreeInsert
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            remove: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexRemoveImpl,
            >(
                Some(
                    cpBBTreeRemove
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindex: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpBBTree) -> ()>,
                cpSpatialIndexReindexImpl,
            >(Some(cpBBTreeReindex as unsafe extern "C" fn(*mut cpBBTree) -> ())),
            reindexObject: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexReindexObjectImpl,
            >(
                Some(
                    cpBBTreeReindexObject
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindexQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        cpSpatialIndexQueryFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexReindexQueryImpl,
            >(
                Some(
                    cpBBTreeReindexQuery
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            cpSpatialIndexQueryFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            query: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpBB,
                        cpSpatialIndexQueryFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexQueryImpl,
            >(
                Some(
                    cpBBTreeQuery
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpBB,
                            cpSpatialIndexQueryFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpVect,
                        cpVect,
                        cpFloat,
                        cpSpatialIndexSegmentQueryFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexSegmentQueryImpl,
            >(
                Some(
                    cpBBTreeSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpVect,
                            cpVect,
                            cpFloat,
                            cpSpatialIndexSegmentQueryFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn Klass() -> *mut cpSpatialIndexClass {
    return &mut klass;
}
unsafe extern "C" fn cpfcompare(
    mut a: *const cpFloat,
    mut b: *const cpFloat,
) -> libc::c_int {
    return if *a < *b {
        -(1 as libc::c_int)
    } else if *b < *a {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn fillNodeArray(
    mut node: *mut Node,
    mut cursor: *mut *mut *mut Node,
) {
    **cursor = node;
    *cursor = (*cursor).offset(1);
    *cursor;
}
unsafe extern "C" fn partitionNodes(
    mut tree: *mut cpBBTree,
    mut nodes: *mut *mut Node,
    mut count: libc::c_int,
) -> *mut Node {
    if count == 1 as libc::c_int {
        return *nodes.offset(0 as libc::c_int as isize)
    } else if count == 2 as libc::c_int {
        return NodeNew(
            tree,
            *nodes.offset(0 as libc::c_int as isize),
            *nodes.offset(1 as libc::c_int as isize),
        )
    }
    let mut bb: cpBB = (**nodes.offset(0 as libc::c_int as isize)).bb;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < count {
        bb = cpBBMerge(bb, (**nodes.offset(i as isize)).bb);
        i += 1;
        i;
    }
    let mut splitWidth: cpBool = (bb.r - bb.l > bb.t - bb.b) as libc::c_int as cpBool;
    let mut bounds: *mut cpFloat = calloc(
        (count * 2 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<cpFloat>() as libc::c_ulong,
    ) as *mut cpFloat;
    if splitWidth != 0 {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < count {
            *bounds
                .offset(
                    (2 as libc::c_int * i_0 + 0 as libc::c_int) as isize,
                ) = (**nodes.offset(i_0 as isize)).bb.l;
            *bounds
                .offset(
                    (2 as libc::c_int * i_0 + 1 as libc::c_int) as isize,
                ) = (**nodes.offset(i_0 as isize)).bb.r;
            i_0 += 1;
            i_0;
        }
    } else {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < count {
            *bounds
                .offset(
                    (2 as libc::c_int * i_1 + 0 as libc::c_int) as isize,
                ) = (**nodes.offset(i_1 as isize)).bb.b;
            *bounds
                .offset(
                    (2 as libc::c_int * i_1 + 1 as libc::c_int) as isize,
                ) = (**nodes.offset(i_1 as isize)).bb.t;
            i_1 += 1;
            i_1;
        }
    }
    qsort(
        bounds as *mut libc::c_void,
        (count * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<cpFloat>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const cpFloat, *const cpFloat) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
        >(
            Some(
                cpfcompare
                    as unsafe extern "C" fn(
                        *const cpFloat,
                        *const cpFloat,
                    ) -> libc::c_int,
            ),
        ),
    );
    let mut split: cpFloat = (*bounds.offset((count - 1 as libc::c_int) as isize)
        + *bounds.offset(count as isize)) * 0.5f32 as libc::c_double;
    free(bounds as *mut libc::c_void);
    let mut a: cpBB = bb;
    let mut b: cpBB = bb;
    if splitWidth != 0 {
        b.l = split;
        a.r = b.l;
    } else {
        b.b = split;
        a.t = b.b;
    }
    let mut right: libc::c_int = count;
    let mut left: libc::c_int = 0 as libc::c_int;
    while left < right {
        let mut node: *mut Node = *nodes.offset(left as isize);
        if cpBBMergedArea((*node).bb, b) < cpBBMergedArea((*node).bb, a) {
            right -= 1;
            right;
            let ref mut fresh1 = *nodes.offset(left as isize);
            *fresh1 = *nodes.offset(right as isize);
            let ref mut fresh2 = *nodes.offset(right as isize);
            *fresh2 = node;
        } else {
            left += 1;
            left;
        }
    }
    if right == count {
        let mut node_0: *mut Node = 0 as *mut Node;
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < count {
            node_0 = SubtreeInsert(node_0, *nodes.offset(i_2 as isize), tree);
            i_2 += 1;
            i_2;
        }
        return node_0;
    }
    return NodeNew(
        tree,
        partitionNodes(tree, nodes, right),
        partitionNodes(tree, nodes.offset(right as isize), count - right),
    );
}
pub unsafe extern "C" fn cpBBTreeOptimize(mut index: *mut cpSpatialIndex) {
    if (*index).klass != &mut klass as *mut cpSpatialIndexClass {
        return;
    }
    let mut tree: *mut cpBBTree = index as *mut cpBBTree;
    let mut root: *mut Node = (*tree).root;
    if root.is_null() {
        return;
    }
    let mut count: libc::c_int = cpBBTreeCount(tree);
    let mut nodes: *mut *mut Node = calloc(
        count as libc::c_ulong,
        ::std::mem::size_of::<*mut Node>() as libc::c_ulong,
    ) as *mut *mut Node;
    let mut cursor: *mut *mut Node = nodes;
    cpHashSetEach(
        (*tree).leaves,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Node, *mut *mut *mut Node) -> ()>,
            cpHashSetIteratorFunc,
        >(
            Some(
                fillNodeArray
                    as unsafe extern "C" fn(*mut Node, *mut *mut *mut Node) -> (),
            ),
        ),
        &mut cursor as *mut *mut *mut Node as *mut libc::c_void,
    );
    SubtreeRecycle(tree, root);
    (*tree).root = partitionNodes(tree, nodes, count);
    free(nodes as *mut libc::c_void);
}
