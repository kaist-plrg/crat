use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn nextafter(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ScaleModel(pdTime: PDOUBLE);
    fn MyStrcmp(sz1: *const libc::c_char, sz2: *const libc::c_char) -> libc::c_int;
    static mut vnStates: libc::c_int;
    static mut vnOutputs: libc::c_int;
    static mut vnModelVars: libc::c_int;
    static mut vnInputs: libc::c_int;
    static mut vnParms: libc::c_int;
    static mut vrgModelVars: [libc::c_double; 0];
    static mut vrgInputs: [IFN; 0];
    static mut vrgvmGlo: [VMMAPSTRCT; 0];
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_ulong;
pub type HANDLE = DWORD;
pub type PSTR = *mut libc::c_char;
pub type PINT = *mut libc::c_int;
pub type PVOID = *mut libc::c_void;
pub type PDOUBLE = *mut libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagIFN {
    pub iType: libc::c_int,
    pub bOn: BOOL,
    pub dTStartPeriod: libc::c_double,
    pub dVal: libc::c_double,
    pub dMag: libc::c_double,
    pub dTper: libc::c_double,
    pub dT0: libc::c_double,
    pub dTexp: libc::c_double,
    pub dDecay: libc::c_double,
    pub dNcpt: libc::c_double,
    pub hMag: HANDLE,
    pub hTper: HANDLE,
    pub hT0: HANDLE,
    pub hTexp: HANDLE,
    pub hDecay: HANDLE,
    pub hNcpt: HANDLE,
    pub nDoses: libc::c_int,
    pub iDoseCur: libc::c_int,
    pub rgT0s: PDOUBLE,
    pub rgMags: PDOUBLE,
    pub rghT0s: *mut HANDLE,
    pub rghMags: *mut HANDLE,
    pub target_state: HANDLE,
    pub rgOper: PINT,
}
pub type IFN = tagIFN;
pub type PIFN = *mut tagIFN;
pub type HVAR = HANDLE;
pub type PVMMAPSTRCT = *mut tagVM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagVM {
    pub szName: PSTR,
    pub pVar: PVOID,
    pub hvar: HVAR,
}
pub type VMMAPSTRCT = tagVM;
pub static mut vbModelReinitd: BOOL = 0 as libc::c_int;
pub unsafe extern "C" fn FixupDependentInputs() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < vnInputs {
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).hMag != 0 {
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dMag = GetVarValue((*vrgInputs.as_mut_ptr().offset(i as isize)).hMag);
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).hTper != 0 {
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dTper = GetVarValue((*vrgInputs.as_mut_ptr().offset(i as isize)).hTper);
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).hT0 != 0 {
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dT0 = GetVarValue((*vrgInputs.as_mut_ptr().offset(i as isize)).hT0);
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).hTexp != 0 {
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dTexp = GetVarValue((*vrgInputs.as_mut_ptr().offset(i as isize)).hTexp);
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).hDecay != 0 {
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dDecay = GetVarValue(
                (*vrgInputs.as_mut_ptr().offset(i as isize)).hDecay,
            );
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).hNcpt != 0 {
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dNcpt = GetVarValue((*vrgInputs.as_mut_ptr().offset(i as isize)).hNcpt);
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 4 as libc::c_int
            || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 5 as libc::c_int
            || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 6 as libc::c_int
        {
            j = 0 as libc::c_int;
            while j < (*vrgInputs.as_mut_ptr().offset(i as isize)).nDoses {
                if *((*vrgInputs.as_mut_ptr().offset(i as isize)).rghMags)
                    .offset(j as isize) != 0
                {
                    *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgMags)
                        .offset(
                            j as isize,
                        ) = GetVarValue(
                        *((*vrgInputs.as_mut_ptr().offset(i as isize)).rghMags)
                            .offset(j as isize),
                    );
                }
                if *((*vrgInputs.as_mut_ptr().offset(i as isize)).rghT0s)
                    .offset(j as isize) != 0
                {
                    *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgT0s)
                        .offset(
                            j as isize,
                        ) = GetVarValue(
                        *((*vrgInputs.as_mut_ptr().offset(i as isize)).rghT0s)
                            .offset(j as isize),
                    );
                }
                if j > 0 as libc::c_int {
                    if *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgT0s)
                        .offset(j as isize)
                        <= *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgT0s)
                            .offset((j - 1 as libc::c_int) as isize)
                    {
                        printf(
                            b"\nError: unordered pair of times (%g, %g) in %s statement - Exiting\n\0"
                                as *const u8 as *const libc::c_char,
                            *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgT0s)
                                .offset((j - 1 as libc::c_int) as isize),
                            *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgT0s)
                                .offset(j as isize),
                            if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType
                                == 4 as libc::c_int
                            {
                                b"NDoses\0" as *const u8 as *const libc::c_char
                            } else {
                                b"Spikes\0" as *const u8 as *const libc::c_char
                            },
                        );
                        exit(0 as libc::c_int);
                    }
                }
                j += 1;
                j;
            }
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 3 as libc::c_int
            || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 7 as libc::c_int
        {
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dTexp = (*vrgInputs.as_mut_ptr().offset(i as isize)).dTper;
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 7 as libc::c_int {
            if (*vrgInputs.as_mut_ptr().offset(i as isize)).dNcpt
                <= 0 as libc::c_int as libc::c_double
            {
                printf(
                    b"\nError: null or negative number of virtual compartment in PerTransit input function - Exiting\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dTexp = (*vrgInputs.as_mut_ptr().offset(i as isize)).dTper;
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).dTexp
            >= (*vrgInputs.as_mut_ptr().offset(i as isize)).dTper
        {
            (*vrgInputs.as_mut_ptr().offset(i as isize))
                .dTexp = (*vrgInputs.as_mut_ptr().offset(i as isize)).dTper;
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 4 as libc::c_int
            || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 5 as libc::c_int
        {
            (*vrgInputs.as_mut_ptr().offset(i as isize)).dMag = 1.0f64;
        }
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 2 as libc::c_int
            || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 7 as libc::c_int
            || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 3 as libc::c_int
        {
            if (*vrgInputs.as_mut_ptr().offset(i as isize)).dTexp == 0.0f64
                || (*vrgInputs.as_mut_ptr().offset(i as isize)).dT0 < 0.0f64
                || (*vrgInputs.as_mut_ptr().offset(i as isize)).dTper < 0.0f64
            {
                (*vrgInputs.as_mut_ptr().offset(i as isize)).dMag = 0.0f64;
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn GetStartPeriods(mut pdTime: PDOUBLE) {
    let mut i: libc::c_int = 0;
    let mut dTmp: libc::c_double = 0.;
    let mut dDummy: libc::c_double = 0.;
    if *pdTime == 0.0f64 {
        i = 0 as libc::c_int;
        while i < vnInputs {
            (*vrgInputs.as_mut_ptr().offset(i as isize)).dTStartPeriod = 0.0f64;
            if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 4 as libc::c_int
                || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 5 as libc::c_int
            {
                (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur = 0 as libc::c_int;
            }
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < vnInputs {
            if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 4 as libc::c_int
                || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType == 5 as libc::c_int
            {
                (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur = 0 as libc::c_int;
                while (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur
                    < (*vrgInputs.as_mut_ptr().offset(i as isize)).nDoses
                {
                    if *pdTime
                        < *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgT0s)
                            .offset(
                                (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur
                                    as isize,
                            )
                    {
                        break;
                    }
                    let ref mut fresh0 = (*vrgInputs.as_mut_ptr().offset(i as isize))
                        .iDoseCur;
                    *fresh0 += 1;
                    *fresh0;
                }
                let ref mut fresh1 = (*vrgInputs.as_mut_ptr().offset(i as isize))
                    .iDoseCur;
                *fresh1 -= 1;
                *fresh1;
                if (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur
                    < 0 as libc::c_int
                {
                    (*vrgInputs.as_mut_ptr().offset(i as isize))
                        .iDoseCur = 0 as libc::c_int;
                }
            } else if (*vrgInputs.as_mut_ptr().offset(i as isize)).dTper == 0.0f64 {
                (*vrgInputs.as_mut_ptr().offset(i as isize)).dTStartPeriod = 0.0f64;
            } else {
                dTmp = *pdTime / (*vrgInputs.as_mut_ptr().offset(i as isize)).dTper;
                (*vrgInputs.as_mut_ptr().offset(i as isize))
                    .dTStartPeriod = (*vrgInputs.as_mut_ptr().offset(i as isize)).dTper
                    * modf(dTmp, &mut dDummy);
            }
            i += 1;
            i;
        }
    };
}
pub unsafe extern "C" fn UpdateNDoses(
    mut pifn: PIFN,
    mut pdTnext: PDOUBLE,
    mut pdTime: PDOUBLE,
) {
    let mut j: libc::c_int = 0;
    j = (*pifn).iDoseCur;
    if j < (*pifn).nDoses {
        *pdTnext = *((*pifn).rgT0s).offset(j as isize);
        (*pifn).bOn = (*pdTime >= *pdTnext) as libc::c_int;
        if (*pifn).bOn != 0 {
            *pdTnext = *((*pifn).rgT0s).offset((j + 1 as libc::c_int) as isize);
            (*pifn).bOn = (*pdTime < *pdTnext) as libc::c_int;
            if (*pifn).bOn == 0 {
                (*pifn).iDoseCur += 1;
                if (*pifn).iDoseCur < (*pifn).nDoses {
                    *pdTnext = *((*pifn).rgT0s)
                        .offset(((*pifn).iDoseCur + 1 as libc::c_int) as isize);
                    (*pifn).bOn = 1 as libc::c_int;
                }
            }
        }
    } else {
        *pdTnext = 1.7976931348623157e+308f64;
    }
    if (*pifn).bOn == 0 {
        (*pifn).dVal = 0.0f64;
    }
}
pub unsafe extern "C" fn UpdateSpikes(
    mut pifn: PIFN,
    mut pdTnext: PDOUBLE,
    mut pdTime: PDOUBLE,
) -> BOOL {
    let mut rgT0s: *mut libc::c_double = (*pifn).rgT0s;
    let mut j: libc::c_int = (*pifn).iDoseCur;
    *pdTnext = 1.7976931348623157e+308f64;
    (*pifn).bOn = 0 as libc::c_int;
    if j < (*pifn).nDoses {
        if *pdTime < *rgT0s.offset(j as isize) {
            *pdTnext = *rgT0s.offset(j as isize);
        } else if *pdTime == *rgT0s.offset(j as isize) {
            (*pifn).bOn = 1 as libc::c_int;
            if (j + 1 as libc::c_int) < (*pifn).nDoses {
                *pdTnext = *rgT0s.offset((j + 1 as libc::c_int) as isize);
            }
        } else {
            let mut nextRepTime: libc::c_double = nextafter(
                *pdTime,
                *rgT0s.offset(j as isize),
            );
            if nextRepTime == *rgT0s.offset(j as isize) {
                printf(
                    b"\n UpdateSpikes: Discontinuity time failed match corrected for floating point precision at\nsimulation time = %.17f \nevent time      = %.17f \n\0"
                        as *const u8 as *const libc::c_char,
                    *pdTime,
                    *rgT0s.offset(j as isize),
                );
                (*pifn).bOn = 1 as libc::c_int;
                if (j + 1 as libc::c_int) < (*pifn).nDoses {
                    *pdTnext = *rgT0s.offset((j + 1 as libc::c_int) as isize);
                }
            } else {
                printf(
                    b"\n UpdateSpikes: Discontinuity was passed over at\nsimulation time = %.17f \nevent time      = %.17f \n\0"
                        as *const u8 as *const libc::c_char,
                    *pdTime,
                    *rgT0s.offset(j as isize),
                );
            }
        }
    }
    return (*pifn).bOn;
}
pub unsafe extern "C" fn UpdateDefaultInput(
    mut pifn: PIFN,
    mut pdTnext: PDOUBLE,
    mut pdTime: PDOUBLE,
) {
    *pdTnext = (*pifn).dTStartPeriod + (*pifn).dT0;
    (*pifn).bOn = (*pdTime >= *pdTnext) as libc::c_int;
    if (*pifn).bOn != 0 {
        *pdTnext += (*pifn).dTexp;
        (*pifn)
            .bOn = (*pdTime < *pdTnext
            && *pdTnext - *pdTime > 2.2204460492503131e-16f64 * 2.0f64 * *pdTnext)
            as libc::c_int;
        if (*pifn).bOn == 0 {
            if (*pifn).dTper != 0.0f64 {
                (*pifn).dTStartPeriod += (*pifn).dTper;
                *pdTnext = (*pifn).dTStartPeriod;
            } else {
                (*pifn).dTStartPeriod = 1.7976931348623157e+308f64 - (*pifn).dTper;
                *pdTnext = (*pifn).dTStartPeriod;
            }
        }
    }
    if (*pifn).bOn == 0 {
        (*pifn).dVal = 0.0f64;
    } else {
        (*pifn).dVal = (*pifn).dMag;
    };
}
pub unsafe extern "C" fn UpdateInputs(
    mut pdTime: PDOUBLE,
    mut pdNextTransTime: PDOUBLE,
) {
    let mut dT: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut stateID: libc::c_int = 0;
    if vbModelReinitd != 0 {
        ScaleModel(pdTime);
        FixupDependentInputs();
        GetStartPeriods(pdTime);
    }
    *pdNextTransTime = 1.7976931348623157e+308f64;
    dT = *pdNextTransTime;
    i = 0 as libc::c_int;
    while i < vnInputs {
        match (*vrgInputs.as_mut_ptr().offset(i as isize)).iType {
            1 => {}
            4 => {
                UpdateNDoses(
                    &mut *vrgInputs.as_mut_ptr().offset(i as isize),
                    &mut dT,
                    pdTime,
                );
            }
            6 => {
                if (*vrgInputs.as_mut_ptr().offset(i as isize)).bOn != 0 {
                    let ref mut fresh2 = (*vrgInputs.as_mut_ptr().offset(i as isize))
                        .iDoseCur;
                    *fresh2 += 1;
                    *fresh2;
                }
                UpdateSpikes(
                    &mut *vrgInputs.as_mut_ptr().offset(i as isize),
                    &mut dT,
                    pdTime,
                );
                j = (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur;
                stateID = ((*vrgInputs.as_mut_ptr().offset(i as isize)).target_state
                    & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int;
                if (*vrgInputs.as_mut_ptr().offset(i as isize)).bOn != 0
                    && j < (*vrgInputs.as_mut_ptr().offset(i as isize)).nDoses
                {
                    *vrgModelVars
                        .as_mut_ptr()
                        .offset(
                            stateID as isize,
                        ) = if *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgOper)
                        .offset(j as isize) == 710 as libc::c_int
                    {
                        *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgMags)
                            .offset(j as isize)
                    } else if *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgOper)
                        .offset(j as isize) == 711 as libc::c_int
                    {
                        *vrgModelVars.as_mut_ptr().offset(stateID as isize)
                            + *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgMags)
                                .offset(j as isize)
                    } else {
                        *vrgModelVars.as_mut_ptr().offset(stateID as isize)
                            * *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgMags)
                                .offset(j as isize)
                    };
                }
            }
            5 => {
                if (*vrgInputs.as_mut_ptr().offset(i as isize)).bOn != 0 {
                    let ref mut fresh3 = (*vrgInputs.as_mut_ptr().offset(i as isize))
                        .iDoseCur;
                    *fresh3 += 1;
                    *fresh3;
                }
                UpdateSpikes(
                    &mut *vrgInputs.as_mut_ptr().offset(i as isize),
                    &mut dT,
                    pdTime,
                );
            }
            2 | 3 | 7 | _ => {
                if (*vrgInputs.as_mut_ptr().offset(i as isize)).dMag != 0.0f64 {
                    UpdateDefaultInput(
                        &mut *vrgInputs.as_mut_ptr().offset(i as isize),
                        &mut dT,
                        pdTime,
                    );
                }
            }
        }
        if dT < *pdNextTransTime {
            *pdNextTransTime = dT;
        }
        i += 1;
        i;
    }
    if vbModelReinitd != 0 {
        vbModelReinitd = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn GetVarPtr(
    mut pvm: PVMMAPSTRCT,
    mut szName: PSTR,
) -> PVMMAPSTRCT {
    while *(*pvm).szName as libc::c_int != 0
        && MyStrcmp(szName as *const libc::c_char, (*pvm).szName as *const libc::c_char)
            != 0
    {
        pvm = pvm.offset(1);
        pvm;
    }
    return if *(*pvm).szName as libc::c_int != 0 { pvm } else { 0 as PVMMAPSTRCT };
}
pub unsafe extern "C" fn GetVarHandle(mut szName: PSTR) -> HVAR {
    let mut pvm: PVMMAPSTRCT = GetVarPtr(vrgvmGlo.as_mut_ptr(), szName);
    return if !pvm.is_null() { (*pvm).hvar } else { 0 as libc::c_int as libc::c_ulong };
}
pub unsafe extern "C" fn GetVarType(mut hvar: HVAR) -> libc::c_int {
    let mut bOK: BOOL = 0 as libc::c_int;
    match hvar & 0xf0000 as libc::c_int as libc::c_ulong {
        131072 => {
            bOK = (((hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int)
                < vnInputs) as libc::c_int;
        }
        65536 => {
            bOK = (((hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int)
                < vnStates) as libc::c_int;
        }
        196608 => {
            bOK = ((hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int
                >= vnStates
                && ((hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int)
                    < vnModelVars) as libc::c_int;
        }
        262144 => {
            let mut nSOI: libc::c_int = vnStates + vnOutputs + vnInputs;
            bOK = ((hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int >= nSOI
                && ((hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int)
                    < nSOI + vnParms) as libc::c_int;
        }
        _ => {}
    }
    return (if bOK != 0 {
        hvar & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
}
pub unsafe extern "C" fn GetVarName(mut hvar: HVAR) -> *mut libc::c_char {
    static mut szInvalid: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<
            &[u8; 17],
            &mut [libc::c_char; 17],
        >(b"InvalidVariable?\0")
    };
    let mut pvm: PVMMAPSTRCT = vrgvmGlo.as_mut_ptr();
    while *(*pvm).szName as libc::c_int != 0 && hvar != (*pvm).hvar {
        pvm = pvm.offset(1);
        pvm;
    }
    return if *(*pvm).szName as libc::c_int != 0 {
        (*pvm).szName
    } else {
        szInvalid.as_mut_ptr()
    };
}
pub unsafe extern "C" fn GetVarValue(mut hvar: HVAR) -> libc::c_double {
    let mut dReturn: libc::c_double = 0.0f64;
    match GetVarType(hvar) {
        131072 => {
            dReturn = (*vrgInputs
                .as_mut_ptr()
                .offset(
                    (hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int
                        as isize,
                ))
                .dVal;
        }
        196608 | 65536 => {
            dReturn = *vrgModelVars
                .as_mut_ptr()
                .offset(
                    (hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int
                        as isize,
                );
        }
        262144 => {
            let mut pvm: PVMMAPSTRCT = 0 as *mut tagVM;
            pvm = &mut *vrgvmGlo
                .as_mut_ptr()
                .offset(
                    (hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int
                        as isize,
                ) as *mut VMMAPSTRCT;
            dReturn = *((*pvm).pVar as PDOUBLE);
        }
        _ => {}
    }
    return dReturn;
}
pub unsafe extern "C" fn IsInput(mut hvar: HVAR) -> BOOL {
    return (GetVarType(hvar) == 0x20000 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn IsState(mut hvar: HVAR) -> BOOL {
    return (GetVarType(hvar) == 0x10000 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn IsOutput(mut hvar: HVAR) -> BOOL {
    return (GetVarType(hvar) == 0x30000 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn IsModelVar(mut hvar: HVAR) -> BOOL {
    let mut iType: libc::c_int = GetVarType(hvar);
    return (iType == 0x10000 as libc::c_int || iType == 0x30000 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn IsParm(mut hvar: HVAR) -> BOOL {
    return (GetVarType(hvar) == 0x40000 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn ModelIndex(mut hvar: HVAR) -> libc::c_int {
    return if IsModelVar(hvar) != 0 {
        (hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub unsafe extern "C" fn SetVar(mut hvar: HVAR, mut dVal: libc::c_double) -> BOOL {
    let mut bReturn: BOOL = 1 as libc::c_int;
    match GetVarType(hvar) {
        196608 | 65536 => {
            *vrgModelVars
                .as_mut_ptr()
                .offset(
                    (hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int
                        as isize,
                ) = dVal;
        }
        262144 => {
            let mut pvm: PVMMAPSTRCT = &mut *vrgvmGlo
                .as_mut_ptr()
                .offset(
                    (hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int
                        as isize,
                ) as *mut VMMAPSTRCT;
            *((*pvm).pVar as PDOUBLE) = dVal;
        }
        131072 | _ => {
            bReturn = 0 as libc::c_int;
        }
    }
    return bReturn;
}
pub unsafe extern "C" fn SetInput(mut hvar: HVAR, mut pifn: PIFN) -> BOOL {
    if pifn.is_null() || IsInput(hvar) == 0 {
        return 0 as libc::c_int;
    }
    memcpy(
        &mut *vrgInputs
            .as_mut_ptr()
            .offset(
                (hvar & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int as isize,
            ) as *mut IFN as *mut libc::c_void,
        pifn as *const libc::c_void,
        ::std::mem::size_of::<IFN>() as libc::c_ulong,
    );
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn GetModelVector() -> PDOUBLE {
    return vrgModelVars.as_mut_ptr();
}
pub unsafe extern "C" fn GetNModelVars() -> libc::c_int {
    return vnModelVars;
}
pub unsafe extern "C" fn GetNStates() -> libc::c_int {
    return vnStates;
}
pub unsafe extern "C" fn CalcInputs(mut pdTime: PDOUBLE) {
    let mut i: libc::c_int = 0;
    let mut dTmp: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < vnInputs {
        if (*vrgInputs.as_mut_ptr().offset(i as isize)).iType != 1 as libc::c_int
            && ((*vrgInputs.as_mut_ptr().offset(i as isize)).bOn != 0
                || (*vrgInputs.as_mut_ptr().offset(i as isize)).iType
                    != 5 as libc::c_int)
        {
            match (*vrgInputs.as_mut_ptr().offset(i as isize)).iType {
                4 => {
                    if (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur
                        < (*vrgInputs.as_mut_ptr().offset(i as isize)).nDoses
                    {
                        (*vrgInputs.as_mut_ptr().offset(i as isize))
                            .dVal = *((*vrgInputs.as_mut_ptr().offset(i as isize))
                            .rgMags)
                            .offset(
                                (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur
                                    as isize,
                            ) * (*vrgInputs.as_mut_ptr().offset(i as isize)).dMag;
                    }
                }
                2 => {}
                3 => {
                    (*vrgInputs.as_mut_ptr().offset(i as isize))
                        .dVal = (*vrgInputs.as_mut_ptr().offset(i as isize)).dMag
                        * (*vrgInputs.as_mut_ptr().offset(i as isize)).bOn
                            as libc::c_double
                        * exp(
                            ((*vrgInputs.as_mut_ptr().offset(i as isize)).dTStartPeriod
                                + (*vrgInputs.as_mut_ptr().offset(i as isize)).dT0
                                - *pdTime)
                                * (*vrgInputs.as_mut_ptr().offset(i as isize)).dDecay,
                        );
                }
                7 => {
                    dTmp = (*vrgInputs.as_mut_ptr().offset(i as isize)).dDecay
                        * (*pdTime
                            - (*vrgInputs.as_mut_ptr().offset(i as isize)).dTStartPeriod
                            - (*vrgInputs.as_mut_ptr().offset(i as isize)).dT0);
                    (*vrgInputs.as_mut_ptr().offset(i as isize))
                        .dVal = (*vrgInputs.as_mut_ptr().offset(i as isize)).dMag
                        * (*vrgInputs.as_mut_ptr().offset(i as isize)).bOn
                            as libc::c_double
                        * pow(dTmp, (*vrgInputs.as_mut_ptr().offset(i as isize)).dNcpt)
                        * exp(-dTmp)
                        / (2.506628274631000241612f64
                            * pow(
                                (*vrgInputs.as_mut_ptr().offset(i as isize)).dNcpt,
                                (*vrgInputs.as_mut_ptr().offset(i as isize)).dNcpt + 0.5f64,
                            )
                            * exp(-(*vrgInputs.as_mut_ptr().offset(i as isize)).dNcpt));
                }
                5 => {
                    if *pdTime
                        == *((*vrgInputs.as_mut_ptr().offset(i as isize)).rgT0s)
                            .offset(
                                (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur
                                    as isize,
                            )
                        && (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur
                            < (*vrgInputs.as_mut_ptr().offset(i as isize)).nDoses
                    {
                        (*vrgInputs.as_mut_ptr().offset(i as isize))
                            .dVal = *((*vrgInputs.as_mut_ptr().offset(i as isize))
                            .rgMags)
                            .offset(
                                (*vrgInputs.as_mut_ptr().offset(i as isize)).iDoseCur
                                    as isize,
                            ) * (*vrgInputs.as_mut_ptr().offset(i as isize)).dMag;
                    } else {
                        (*vrgInputs.as_mut_ptr().offset(i as isize))
                            .dVal = 0 as libc::c_int as libc::c_double;
                    }
                }
                1 | _ => {}
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn DumpSymbolTable(mut szFilename: *mut libc::c_char) {
    static mut szStderr: [libc::c_char; 9] = unsafe {
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"<stdout>\0")
    };
    let mut pfile: *mut FILE = 0 as *mut FILE;
    let mut pvm: PVMMAPSTRCT = &mut *vrgvmGlo
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut VMMAPSTRCT;
    if !szFilename.is_null() {
        pfile = fopen(szFilename, b"a\0" as *const u8 as *const libc::c_char);
    } else {
        pfile = stdout;
        szFilename = szStderr.as_mut_ptr();
    }
    if pfile.is_null() {
        printf(
            b"Cannot dump symbol table to %s\n\0" as *const u8 as *const libc::c_char,
            szFilename,
        );
        return;
    }
    fprintf(pfile, b"\nSymbol Table:\n\0" as *const u8 as *const libc::c_char);
    if pvm.is_null() {
        fprintf(pfile, b"<null>\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    while *(*pvm).szName != 0 {
        fprintf(pfile, b"%s \t= \0" as *const u8 as *const libc::c_char, (*pvm).szName);
        if IsInput((*pvm).hvar) != 0 {
            let mut pifn: PIFN = (*pvm).pVar as PIFN;
            fprintf(
                pfile,
                b"Mag=%g [Val=%g]\n\0" as *const u8 as *const libc::c_char,
                (*pifn).dMag,
                (*pifn).dVal,
            );
        } else {
            fprintf(
                pfile,
                b"%g\n\0" as *const u8 as *const libc::c_char,
                *((*pvm).pVar as *mut libc::c_double),
            );
        }
        pvm = pvm.offset(1);
        pvm;
    }
    if szFilename != szStderr.as_mut_ptr() {
        fclose(pfile);
    }
}
pub unsafe extern "C" fn GetStateHandles(mut phvar: *mut HVAR) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut pvm: *mut VMMAPSTRCT = vrgvmGlo.as_mut_ptr();
    while !((*pvm).pVar).is_null() {
        if IsState((*pvm).hvar) != 0 {
            let fresh4 = i;
            i = i + 1;
            *phvar.offset(fresh4 as isize) = (*pvm).hvar;
        }
        pvm = pvm.offset(1);
        pvm;
    }
}
