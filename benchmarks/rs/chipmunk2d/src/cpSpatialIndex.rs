use ::libc;
extern "C" {
    fn abort() -> !;
    fn free(__ptr: *mut libc::c_void);
    fn cpMessage(
        condition: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        isError: libc::c_int,
        isHardError: libc::c_int,
        message: *const libc::c_char,
        _: ...
    );
}
pub type __uint32_t = libc::c_uint;
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
pub struct dynamicToStaticContext {
    pub bbfunc: cpSpatialIndexBBFunc,
    pub staticIndex: *mut cpSpatialIndex,
    pub queryFunc: cpSpatialIndexQueryFunc,
    pub data: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn cpSpatialIndexDestroy(mut index: *mut cpSpatialIndex) {
    if !((*index).klass).is_null() {
        ((*(*index).klass).destroy).unwrap()(index);
    }
}
#[inline]
unsafe extern "C" fn cpSpatialIndexQuery(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).query).unwrap()(index, obj, bb, func, data);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexEach(
    mut index: *mut cpSpatialIndex,
    mut func: cpSpatialIndexIteratorFunc,
    mut data: *mut libc::c_void,
) {
    ((*(*index).klass).each).unwrap()(index, func, data);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexCount(mut index: *mut cpSpatialIndex) -> libc::c_int {
    return ((*(*index).klass).count).unwrap()(index);
}
pub unsafe extern "C" fn cpSpatialIndexFree(mut index: *mut cpSpatialIndex) {
    if !index.is_null() {
        cpSpatialIndexDestroy(index);
        free(index as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpSpatialIndexInit(
    mut index: *mut cpSpatialIndex,
    mut klass: *mut cpSpatialIndexClass,
    mut bbfunc: cpSpatialIndexBBFunc,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    (*index).klass = klass;
    (*index).bbfunc = bbfunc;
    (*index).staticIndex = staticIndex;
    if !staticIndex.is_null() {
        if !((*staticIndex).dynamicIndex).is_null() {
            cpMessage(
                b"!staticIndex->dynamicIndex\0" as *const u8 as *const libc::c_char,
                b"../../src/cpSpatialIndex.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"This static index is already associated with a dynamic index.\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        (*staticIndex).dynamicIndex = index;
    }
    return index;
}
unsafe extern "C" fn dynamicToStaticIter(
    mut obj: *mut libc::c_void,
    mut context: *mut dynamicToStaticContext,
) {
    cpSpatialIndexQuery(
        (*context).staticIndex,
        obj,
        ((*context).bbfunc).unwrap()(obj),
        (*context).queryFunc,
        (*context).data,
    );
}
pub unsafe extern "C" fn cpSpatialIndexCollideStatic(
    mut dynamicIndex: *mut cpSpatialIndex,
    mut staticIndex: *mut cpSpatialIndex,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    if !staticIndex.is_null() && cpSpatialIndexCount(staticIndex) > 0 as libc::c_int {
        let mut context: dynamicToStaticContext = {
            let mut init = dynamicToStaticContext {
                bbfunc: (*dynamicIndex).bbfunc,
                staticIndex: staticIndex,
                queryFunc: func,
                data: data,
            };
            init
        };
        cpSpatialIndexEach(
            dynamicIndex,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut dynamicToStaticContext,
                    ) -> (),
                >,
                cpSpatialIndexIteratorFunc,
            >(
                Some(
                    dynamicToStaticIter
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut dynamicToStaticContext,
                        ) -> (),
                ),
            ),
            &mut context as *mut dynamicToStaticContext as *mut libc::c_void,
        );
    }
}
