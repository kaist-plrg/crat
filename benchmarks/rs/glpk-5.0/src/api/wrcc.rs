use ::libc;
extern "C" {
    pub type glp_file;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_format(f: *mut glp_file, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_ioerr(f: *mut glp_file) -> libc::c_int;
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_graph {
    pub pool: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub nv_max: libc::c_int,
    pub nv: libc::c_int,
    pub na: libc::c_int,
    pub v: *mut *mut glp_vertex,
    pub index: *mut libc::c_void,
    pub v_size: libc::c_int,
    pub a_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_vertex {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub entry: *mut libc::c_void,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub in_0: *mut glp_arc,
    pub out: *mut glp_arc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_arc {
    pub tail: *mut glp_vertex,
    pub head: *mut glp_vertex,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub t_prev: *mut glp_arc,
    pub t_next: *mut glp_arc,
    pub h_prev: *mut glp_arc,
    pub h_next: *mut glp_arc,
}
pub unsafe extern "C" fn glp_write_ccdata(
    mut G: *mut glp_graph,
    mut v_wgt: libc::c_int,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut e: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut w: libc::c_double = 0.;
    if v_wgt >= 0 as libc::c_int
        && v_wgt
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/wrcc.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
        ))
            .unwrap()(
            b"glp_write_ccdata: v_wgt = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_wgt,
        );
    }
    glp_printf(b"Writing graph to '%s'\n\0" as *const u8 as *const libc::c_char, fname);
    fp = _glp_open(fname, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        glp_printf(
            b"Unable to create '%s' - %s\n\0" as *const u8 as *const libc::c_char,
            fname,
            _glp_get_err_msg(),
        );
        ret = 1 as libc::c_int;
    } else {
        _glp_format(
            fp,
            b"c %s\n\0" as *const u8 as *const libc::c_char,
            (if ((*G).name).is_null() {
                b"unknown\0" as *const u8 as *const libc::c_char
            } else {
                (*G).name as *const libc::c_char
            }),
        );
        count += 1;
        count;
        _glp_format(
            fp,
            b"p edge %d %d\n\0" as *const u8 as *const libc::c_char,
            (*G).nv,
            (*G).na,
        );
        count += 1;
        count;
        if v_wgt >= 0 as libc::c_int {
            i = 1 as libc::c_int;
            while i <= (*G).nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    &mut w as *mut libc::c_double as *mut libc::c_void,
                    ((*v).data as *mut libc::c_char).offset(v_wgt as isize)
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                if w != 1.0f64 {
                    _glp_format(
                        fp,
                        b"n %d %.*g\n\0" as *const u8 as *const libc::c_char,
                        i,
                        15 as libc::c_int,
                        w,
                    );
                    count += 1;
                    count;
                }
                i += 1;
                i;
            }
        }
        i = 1 as libc::c_int;
        while i <= (*G).nv {
            v = *((*G).v).offset(i as isize);
            e = (*v).out;
            while !e.is_null() {
                _glp_format(
                    fp,
                    b"e %d %d\n\0" as *const u8 as *const libc::c_char,
                    (*(*e).tail).i,
                    (*(*e).head).i,
                );
                count += 1;
                count;
                e = (*e).t_next;
            }
            i += 1;
            i;
        }
        _glp_format(fp, b"c eof\n\0" as *const u8 as *const libc::c_char);
        count += 1;
        count;
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as libc::c_int;
        } else {
            glp_printf(
                b"%d lines were written\n\0" as *const u8 as *const libc::c_char,
                count,
            );
            ret = 0 as libc::c_int;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}
pub unsafe extern "C" fn glp_write_graph(
    mut G: *mut glp_graph,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    return glp_write_ccdata(G, -(1 as libc::c_int), fname);
}
