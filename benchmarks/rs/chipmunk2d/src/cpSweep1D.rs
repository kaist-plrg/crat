use ::libc;
extern "C" {
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn cpSpatialIndexCollideStatic(
        dynamicIndex: *mut cpSpatialIndex,
        staticIndex: *mut cpSpatialIndex,
        func: cpSpatialIndexQueryFunc,
        data: *mut libc::c_void,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
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
pub struct cpSweep1D {
    pub spatialIndex: cpSpatialIndex,
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub table: *mut TableCell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableCell {
    pub obj: *mut libc::c_void,
    pub bounds: Bounds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bounds {
    pub min: cpFloat,
    pub max: cpFloat,
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
unsafe extern "C" fn cpBBNew(l: cpFloat, b: cpFloat, r: cpFloat, t: cpFloat) -> cpBB {
    let mut bb: cpBB = {
        let mut init = cpBB { l: l, b: b, r: r, t: t };
        init
    };
    return bb;
}
#[inline]
unsafe extern "C" fn cpBBExpand(bb: cpBB, v: cpVect) -> cpBB {
    return cpBBNew(
        cpfmin(bb.l, v.x),
        cpfmin(bb.b, v.y),
        cpfmax(bb.r, v.x),
        cpfmax(bb.t, v.y),
    );
}
#[inline]
unsafe extern "C" fn BoundsOverlap(mut a: Bounds, mut b: Bounds) -> cpBool {
    return (a.min <= b.max && b.min <= a.max) as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn BBToBounds(mut sweep: *mut cpSweep1D, mut bb: cpBB) -> Bounds {
    let mut bounds: Bounds = {
        let mut init = Bounds { min: bb.l, max: bb.r };
        init
    };
    return bounds;
}
#[inline]
unsafe extern "C" fn MakeTableCell(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
) -> TableCell {
    let mut cell: TableCell = {
        let mut init = TableCell {
            obj: obj,
            bounds: BBToBounds(sweep, ((*sweep).spatialIndex.bbfunc).unwrap()(obj)),
        };
        init
    };
    return cell;
}
pub unsafe extern "C" fn cpSweep1DAlloc() -> *mut cpSweep1D {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpSweep1D>() as libc::c_ulong,
    ) as *mut cpSweep1D;
}
unsafe extern "C" fn ResizeTable(mut sweep: *mut cpSweep1D, mut size: libc::c_int) {
    (*sweep).max = size;
    (*sweep)
        .table = realloc(
        (*sweep).table as *mut libc::c_void,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TableCell>() as libc::c_ulong),
    ) as *mut TableCell;
}
pub unsafe extern "C" fn cpSweep1DInit(
    mut sweep: *mut cpSweep1D,
    mut bbfunc: cpSpatialIndexBBFunc,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    cpSpatialIndexInit(sweep as *mut cpSpatialIndex, Klass(), bbfunc, staticIndex);
    (*sweep).num = 0 as libc::c_int;
    ResizeTable(sweep, 32 as libc::c_int);
    return sweep as *mut cpSpatialIndex;
}
pub unsafe extern "C" fn cpSweep1DNew(
    mut bbfunc: cpSpatialIndexBBFunc,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    return cpSweep1DInit(cpSweep1DAlloc(), bbfunc, staticIndex);
}
unsafe extern "C" fn cpSweep1DDestroy(mut sweep: *mut cpSweep1D) {
    free((*sweep).table as *mut libc::c_void);
    (*sweep).table = 0 as *mut TableCell;
}
unsafe extern "C" fn cpSweep1DCount(mut sweep: *mut cpSweep1D) -> libc::c_int {
    return (*sweep).num;
}
unsafe extern "C" fn cpSweep1DEach(
    mut sweep: *mut cpSweep1D,
    mut func: cpSpatialIndexIteratorFunc,
    mut data: *mut libc::c_void,
) {
    let mut table: *mut TableCell = (*sweep).table;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = (*sweep).num;
    while i < count {
        func.unwrap()((*table.offset(i as isize)).obj, data);
        i += 1;
        i;
    }
}
unsafe extern "C" fn cpSweep1DContains(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) -> libc::c_int {
    let mut table: *mut TableCell = (*sweep).table;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = (*sweep).num;
    while i < count {
        if (*table.offset(i as isize)).obj == obj {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cpSweep1DInsert(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    if (*sweep).num == (*sweep).max {
        ResizeTable(sweep, (*sweep).max * 2 as libc::c_int);
    }
    *((*sweep).table).offset((*sweep).num as isize) = MakeTableCell(sweep, obj);
    (*sweep).num += 1;
    (*sweep).num;
}
unsafe extern "C" fn cpSweep1DRemove(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut table: *mut TableCell = (*sweep).table;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = (*sweep).num;
    while i < count {
        if (*table.offset(i as isize)).obj == obj {
            (*sweep).num -= 1;
            let mut num: libc::c_int = (*sweep).num;
            *table.offset(i as isize) = *table.offset(num as isize);
            let ref mut fresh0 = (*table.offset(num as isize)).obj;
            *fresh0 = 0 as *mut libc::c_void;
            return;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn cpSweep1DReindexObject(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {}
unsafe extern "C" fn cpSweep1DReindex(mut sweep: *mut cpSweep1D) {}
unsafe extern "C" fn cpSweep1DQuery(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    let mut bounds: Bounds = BBToBounds(sweep, bb);
    let mut table: *mut TableCell = (*sweep).table;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = (*sweep).num;
    while i < count {
        let mut cell: TableCell = *table.offset(i as isize);
        if BoundsOverlap(bounds, cell.bounds) as libc::c_int != 0 && obj != cell.obj {
            func.unwrap()(obj, cell.obj, 0 as libc::c_int as cpCollisionID, data);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn cpSweep1DSegmentQuery(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: cpSpatialIndexSegmentQueryFunc,
    mut data: *mut libc::c_void,
) {
    let mut bb: cpBB = cpBBExpand(cpBBNew(a.x, a.y, a.x, a.y), b);
    let mut bounds: Bounds = BBToBounds(sweep, bb);
    let mut table: *mut TableCell = (*sweep).table;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = (*sweep).num;
    while i < count {
        let mut cell: TableCell = *table.offset(i as isize);
        if BoundsOverlap(bounds, cell.bounds) != 0 {
            func.unwrap()(obj, cell.obj, data);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn TableSort(
    mut a: *mut TableCell,
    mut b: *mut TableCell,
) -> libc::c_int {
    return if (*a).bounds.min < (*b).bounds.min {
        -(1 as libc::c_int)
    } else if (*a).bounds.min > (*b).bounds.min {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn cpSweep1DReindexQuery(
    mut sweep: *mut cpSweep1D,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    let mut table: *mut TableCell = (*sweep).table;
    let mut count: libc::c_int = (*sweep).num;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        *table
            .offset(i as isize) = MakeTableCell(sweep, (*table.offset(i as isize)).obj);
        i += 1;
        i;
    }
    qsort(
        table as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<TableCell>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut TableCell, *mut TableCell) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
        >(
            Some(
                TableSort
                    as unsafe extern "C" fn(
                        *mut TableCell,
                        *mut TableCell,
                    ) -> libc::c_int,
            ),
        ),
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < count {
        let mut cell: TableCell = *table.offset(i_0 as isize);
        let mut max: cpFloat = cell.bounds.max;
        let mut j: libc::c_int = i_0 + 1 as libc::c_int;
        while (*table.offset(j as isize)).bounds.min < max && j < count {
            func
                .unwrap()(
                cell.obj,
                (*table.offset(j as isize)).obj,
                0 as libc::c_int as cpCollisionID,
                data,
            );
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    cpSpatialIndexCollideStatic(
        sweep as *mut cpSpatialIndex,
        (*sweep).spatialIndex.staticIndex,
        func,
        data,
    );
}
static mut klass: cpSpatialIndexClass = unsafe {
    {
        let mut init = cpSpatialIndexClass {
            destroy: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSweep1D) -> ()>,
                cpSpatialIndexDestroyImpl,
            >(Some(cpSweep1DDestroy as unsafe extern "C" fn(*mut cpSweep1D) -> ())),
            count: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSweep1D) -> libc::c_int>,
                cpSpatialIndexCountImpl,
            >(
                Some(
                    cpSweep1DCount as unsafe extern "C" fn(*mut cpSweep1D) -> libc::c_int,
                ),
            ),
            each: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        cpSpatialIndexIteratorFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexEachImpl,
            >(
                Some(
                    cpSweep1DEach
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            cpSpatialIndexIteratorFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            contains: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> libc::c_int,
                >,
                cpSpatialIndexContainsImpl,
            >(
                Some(
                    cpSweep1DContains
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> libc::c_int,
                ),
            ),
            insert: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexInsertImpl,
            >(
                Some(
                    cpSweep1DInsert
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            remove: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexRemoveImpl,
            >(
                Some(
                    cpSweep1DRemove
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindex: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSweep1D) -> ()>,
                cpSpatialIndexReindexImpl,
            >(Some(cpSweep1DReindex as unsafe extern "C" fn(*mut cpSweep1D) -> ())),
            reindexObject: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexReindexObjectImpl,
            >(
                Some(
                    cpSweep1DReindexObject
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindexQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        cpSpatialIndexQueryFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexReindexQueryImpl,
            >(
                Some(
                    cpSweep1DReindexQuery
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            cpSpatialIndexQueryFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            query: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpBB,
                        cpSpatialIndexQueryFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexQueryImpl,
            >(
                Some(
                    cpSweep1DQuery
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
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
                        *mut cpSweep1D,
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
                    cpSweep1DSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
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
