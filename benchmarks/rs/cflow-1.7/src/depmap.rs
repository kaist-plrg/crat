use ::libc;
extern "C" {
    fn xzalloc(s: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cflow_depmap {
    pub nrows: size_t,
    pub rowlen: size_t,
    pub r: [libc::c_uint; 1],
}
pub type cflow_depmap_t = *mut cflow_depmap;
unsafe extern "C" fn transitive_closure(mut R: *mut libc::c_uint, mut n: libc::c_int) {
    let mut rowsize: size_t = 0;
    let mut mask: libc::c_uint = 0;
    let mut rowj: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut rp: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut rend: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut ccol: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut relend: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut cword: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut rowi: *mut libc::c_uint = 0 as *mut libc::c_uint;
    rowsize = (n as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong);
    relend = (R as *mut libc::c_char)
        .offset((n as libc::c_ulong).wrapping_mul(rowsize) as isize)
        as *mut libc::c_uint;
    cword = R;
    mask = 1 as libc::c_int as libc::c_uint;
    rowi = R;
    while rowi < relend {
        ccol = cword;
        rowj = R;
        while rowj < relend {
            if *ccol & mask != 0 {
                rp = rowi;
                rend = (rowj as *mut libc::c_char).offset(rowsize as isize)
                    as *mut libc::c_uint;
                while rowj < rend {
                    let fresh0 = rp;
                    rp = rp.offset(1);
                    let fresh1 = rowj;
                    rowj = rowj.offset(1);
                    *fresh1 |= *fresh0;
                }
            } else {
                rowj = (rowj as *mut libc::c_char).offset(rowsize as isize)
                    as *mut libc::c_uint;
            }
            ccol = (ccol as *mut libc::c_char).offset(rowsize as isize)
                as *mut libc::c_uint;
        }
        mask <<= 1 as libc::c_int;
        if mask == 0 as libc::c_int as libc::c_uint {
            mask = 1 as libc::c_int as libc::c_uint;
            cword = cword.offset(1);
            cword;
        }
        rowi = (rowi as *mut libc::c_char).offset(rowsize as isize) as *mut libc::c_uint;
    }
}
pub unsafe extern "C" fn depmap_alloc(mut count: size_t) -> cflow_depmap_t {
    let mut size: size_t = count
        .wrapping_add(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        );
    let mut dmap: cflow_depmap_t = xzalloc(
        (::std::mem::size_of::<cflow_depmap>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                count
                    .wrapping_mul(size)
                    .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
            ),
    ) as cflow_depmap_t;
    (*dmap).nrows = count;
    (*dmap).rowlen = size;
    return dmap;
}
unsafe extern "C" fn depmap_rowptr(
    mut dmap: cflow_depmap_t,
    mut row: size_t,
) -> *mut libc::c_uint {
    return ((*dmap).r).as_mut_ptr().offset(((*dmap).rowlen).wrapping_mul(row) as isize);
}
pub unsafe extern "C" fn depmap_set(
    mut dmap: cflow_depmap_t,
    mut row: size_t,
    mut col: size_t,
) {
    let mut rptr: *mut libc::c_uint = depmap_rowptr(dmap, row);
    *rptr
        .offset(
            col
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as isize,
        )
        |= ((1 as libc::c_int)
            << col
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )) as libc::c_uint;
}
pub unsafe extern "C" fn depmap_isset(
    mut dmap: cflow_depmap_t,
    mut row: size_t,
    mut col: size_t,
) -> libc::c_int {
    let mut rptr: *mut libc::c_uint = depmap_rowptr(dmap, row);
    return (*rptr
        .offset(
            col
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as isize,
        )
        & ((1 as libc::c_int)
            << col
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )) as libc::c_uint != 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
pub unsafe extern "C" fn depmap_tc(mut dmap: cflow_depmap_t) {
    transitive_closure(((*dmap).r).as_mut_ptr(), (*dmap).nrows as libc::c_int);
}
