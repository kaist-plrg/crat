use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn GetNModelVars() -> libc::c_int;
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn InitlVector(cVectors: libc::c_long) -> *mut libc::c_long;
    fn InitpdVector(cVectors: libc::c_long) -> *mut *mut libc::c_double;
    static mut vrgModelVars: [libc::c_double; 0];
}
pub type DWORD = libc::c_ulong;
pub type HANDLE = DWORD;
pub type HVAR = HANDLE;
static mut dInitialTime: libc::c_double = 0.;
pub static mut iCurrentTime: libc::c_int = 0;
pub static mut rgdTime: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
pub static mut rgiVars: *mut libc::c_long = 0 as *const libc::c_long
    as *mut libc::c_long;
pub static mut pdVar: *mut *mut libc::c_double = 0 as *const *mut libc::c_double
    as *mut *mut libc::c_double;
pub unsafe extern "C" fn CalcDelay(
    mut hvar: HVAR,
    mut dTime: libc::c_double,
    mut delay: libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut sentinel: libc::c_int = 0;
    let mut dTmp: libc::c_double = 0.;
    let mut oldTime: libc::c_double = 0.;
    if *rgiVars.offset(hvar as isize) == 0 {
        let ref mut fresh0 = *pdVar.offset(hvar as isize);
        *fresh0 = InitdVector(1000 as libc::c_int as libc::c_long);
        *(*pdVar.offset(hvar as isize))
            .offset(
                0 as libc::c_int as isize,
            ) = *vrgModelVars.as_mut_ptr().offset(hvar as isize);
        *rgiVars.offset(hvar as isize) = 1 as libc::c_int as libc::c_long;
    }
    if delay <= 0 as libc::c_int as libc::c_double {
        printf(
            b"\nError: negative or null delays aren't allowed - Exiting.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    } else {
        oldTime = dTime - delay;
        if oldTime <= dInitialTime {
            return *(*pdVar.offset(hvar as isize)).offset(0 as libc::c_int as isize)
        } else {
            i = iCurrentTime - 1 as libc::c_int;
            if i < 0 as libc::c_int {
                i = 1000 as libc::c_int - 1 as libc::c_int;
            }
            sentinel = 0 as libc::c_int;
            while *rgdTime.offset(i as isize) > oldTime {
                i = i - 1 as libc::c_int;
                if i < 0 as libc::c_int {
                    i = 1000 as libc::c_int - 1 as libc::c_int;
                }
                sentinel = sentinel + 1 as libc::c_int;
                if sentinel > 1000 as libc::c_int - 1 as libc::c_int {
                    printf(
                        b"Error: size MAX_DELAY of rgdTime array = %ld too small.\n\0"
                            as *const u8 as *const libc::c_char,
                        1000 as libc::c_int as libc::c_long,
                    );
                    exit(0 as libc::c_int);
                }
            }
            if i == iCurrentTime - 1 as libc::c_int {
                dTmp = *(*pdVar.offset(hvar as isize))
                    .offset((i - 1 as libc::c_int) as isize)
                    + (*(*pdVar.offset(hvar as isize)).offset(i as isize)
                        - *(*pdVar.offset(hvar as isize))
                            .offset((i - 1 as libc::c_int) as isize))
                        * (oldTime - *rgdTime.offset((i - 1 as libc::c_int) as isize))
                        / (*rgdTime.offset(i as isize)
                            - *rgdTime.offset((i - 1 as libc::c_int) as isize));
            } else {
                dTmp = *(*pdVar.offset(hvar as isize)).offset(i as isize)
                    + (*(*pdVar.offset(hvar as isize))
                        .offset((i + 1 as libc::c_int) as isize)
                        - *(*pdVar.offset(hvar as isize)).offset(i as isize))
                        * (oldTime - *rgdTime.offset(i as isize))
                        / (*rgdTime.offset((i + 1 as libc::c_int) as isize)
                            - *rgdTime.offset(i as isize));
            }
        }
        return dTmp;
    };
}
pub unsafe extern "C" fn InitDelays(mut dTime: libc::c_double) {
    let mut i: libc::c_int = 0;
    if rgdTime.is_null() {
        rgdTime = InitdVector(1000 as libc::c_int as libc::c_long);
        iCurrentTime = -(1 as libc::c_int);
        dInitialTime = dTime;
    }
    if rgiVars.is_null() {
        rgiVars = InitlVector(GetNModelVars() as libc::c_long);
        pdVar = InitpdVector(GetNModelVars() as libc::c_long);
        i = 0 as libc::c_int;
        while i < GetNModelVars() {
            *rgiVars.offset(i as isize) = 0 as libc::c_int as libc::c_long;
            i += 1;
            i;
        }
    }
}
pub unsafe extern "C" fn StoreDelayed(mut t: libc::c_double) {
    let mut i: libc::c_int = 0;
    iCurrentTime += 1;
    iCurrentTime;
    if iCurrentTime == 1000 as libc::c_int {
        iCurrentTime = 0 as libc::c_int;
    }
    *rgdTime.offset(iCurrentTime as isize) = t;
    i = 0 as libc::c_int;
    while i < GetNModelVars() {
        if *rgiVars.offset(i as isize) != 0 {
            *(*pdVar.offset(i as isize))
                .offset(
                    iCurrentTime as isize,
                ) = *vrgModelVars.as_mut_ptr().offset(i as isize);
        }
        i += 1;
        i;
    }
}
