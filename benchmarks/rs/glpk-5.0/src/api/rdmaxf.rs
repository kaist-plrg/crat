use ::libc;
extern "C" {
    pub type glp_file;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn _glp_dmx_check_int(csa: *mut DMX, num: libc::c_double);
    fn _glp_dmx_end_of_line(csa: *mut DMX);
    fn _glp_dmx_read_field(csa: *mut DMX);
    fn _glp_dmx_read_designator(csa: *mut DMX);
    fn _glp_dmx_error(csa: *mut DMX, fmt: *const libc::c_char, _: ...);
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn glp_add_vertices(G: *mut glp_graph, nadd: libc::c_int) -> libc::c_int;
    fn glp_add_arc(G: *mut glp_graph, i: libc::c_int, j: libc::c_int) -> *mut glp_arc;
    fn glp_erase_graph(G: *mut glp_graph, v_size: libc::c_int, a_size: libc::c_int);
    fn _glp_str2int(str: *const libc::c_char, val: *mut libc::c_int) -> libc::c_int;
    fn _glp_str2num(str: *const libc::c_char, val: *mut libc::c_double) -> libc::c_int;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DMX {
    pub jump: jmp_buf,
    pub fname: *const libc::c_char,
    pub fp: *mut glp_file,
    pub count: libc::c_int,
    pub c: libc::c_int,
    pub field: [libc::c_char; 256],
    pub empty: libc::c_int,
    pub nonint: libc::c_int,
}
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
pub unsafe extern "C" fn glp_read_maxflow(
    mut G: *mut glp_graph,
    mut _s: *mut libc::c_int,
    mut _t: *mut libc::c_int,
    mut a_cap: libc::c_int,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut _csa: DMX = DMX {
        jump: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        fname: 0 as *const libc::c_char,
        fp: 0 as *mut glp_file,
        count: 0,
        c: 0,
        field: [0; 256],
        empty: 0,
        nonint: 0,
    };
    let mut csa: *mut DMX = &mut _csa;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut cap: libc::c_double = 0.;
    if a_cap >= 0 as libc::c_int
        && a_cap
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/rdmaxf.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_maxflow: a_cap = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cap,
        );
    }
    glp_erase_graph(G, (*G).v_size, (*G).a_size);
    if _setjmp(((*csa).jump).as_mut_ptr()) != 0 {
        ret = 1 as libc::c_int;
    } else {
        (*csa).fname = fname;
        (*csa).fp = 0 as *mut glp_file;
        (*csa).count = 0 as libc::c_int;
        (*csa).c = '\n' as i32;
        (*csa).field[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*csa).nonint = 0 as libc::c_int;
        (*csa).empty = (*csa).nonint;
        glp_printf(
            b"Reading maximum flow problem data from '%s'...\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        (*csa).fp = _glp_open(fname, b"r\0" as *const u8 as *const libc::c_char);
        if ((*csa).fp).is_null() {
            glp_printf(
                b"Unable to open '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
            longjmp(((*csa).jump).as_mut_ptr(), 1 as libc::c_int);
        }
        _glp_dmx_read_designator(csa);
        if strcmp(
            ((*csa).field).as_mut_ptr(),
            b"p\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            _glp_dmx_error(
                csa,
                b"problem line missing or invalid\0" as *const u8 as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if strcmp(
            ((*csa).field).as_mut_ptr(),
            b"max\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            _glp_dmx_error(
                csa,
                b"wrong problem designator; 'max' expected\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut nv) == 0 as libc::c_int
            && nv >= 2 as libc::c_int)
        {
            _glp_dmx_error(
                csa,
                b"number of nodes missing or invalid\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut na) == 0 as libc::c_int
            && na >= 0 as libc::c_int)
        {
            _glp_dmx_error(
                csa,
                b"number of arcs missing or invalid\0" as *const u8
                    as *const libc::c_char,
            );
        }
        glp_printf(
            b"Flow network has %d node%s and %d arc%s\n\0" as *const u8
                as *const libc::c_char,
            nv,
            if nv == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
            na,
            if na == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
        );
        if nv > 0 as libc::c_int {
            glp_add_vertices(G, nv);
        }
        _glp_dmx_end_of_line(csa);
        t = 0 as libc::c_int;
        s = t;
        loop {
            _glp_dmx_read_designator(csa);
            if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"n\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                break;
            }
            _glp_dmx_read_field(csa);
            if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as libc::c_int {
                _glp_dmx_error(
                    csa,
                    b"node number missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !(1 as libc::c_int <= i && i <= nv) {
                _glp_dmx_error(
                    csa,
                    b"node number %d out of range\0" as *const u8 as *const libc::c_char,
                    i,
                );
            }
            _glp_dmx_read_field(csa);
            if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"s\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                if s > 0 as libc::c_int {
                    _glp_dmx_error(
                        csa,
                        b"only one source node allowed\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                s = i;
            } else if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"t\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                if t > 0 as libc::c_int {
                    _glp_dmx_error(
                        csa,
                        b"only one sink node allowed\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                t = i;
            } else {
                _glp_dmx_error(
                    csa,
                    b"wrong node designator; 's' or 't' expected\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if s > 0 as libc::c_int && s == t {
                _glp_dmx_error(
                    csa,
                    b"source and sink nodes must be distinct\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_end_of_line(csa);
        }
        if s == 0 as libc::c_int {
            _glp_dmx_error(
                csa,
                b"source node descriptor missing\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if t == 0 as libc::c_int {
            _glp_dmx_error(
                csa,
                b"sink node descriptor missing\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if !_s.is_null() {
            *_s = s;
        }
        if !_t.is_null() {
            *_t = t;
        }
        k = 1 as libc::c_int;
        while k <= na {
            if k > 1 as libc::c_int {
                _glp_dmx_read_designator(csa);
            }
            if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"a\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                _glp_dmx_error(
                    csa,
                    b"wrong line designator; 'a' expected\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_read_field(csa);
            if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as libc::c_int {
                _glp_dmx_error(
                    csa,
                    b"starting node number missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !(1 as libc::c_int <= i && i <= nv) {
                _glp_dmx_error(
                    csa,
                    b"starting node number %d out of range\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            _glp_dmx_read_field(csa);
            if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as libc::c_int {
                _glp_dmx_error(
                    csa,
                    b"ending node number missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !(1 as libc::c_int <= j && j <= nv) {
                _glp_dmx_error(
                    csa,
                    b"ending node number %d out of range\0" as *const u8
                        as *const libc::c_char,
                    j,
                );
            }
            _glp_dmx_read_field(csa);
            if !(_glp_str2num(((*csa).field).as_mut_ptr(), &mut cap) == 0 as libc::c_int
                && cap >= 0.0f64)
            {
                _glp_dmx_error(
                    csa,
                    b"arc capacity missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_check_int(csa, cap);
            a = glp_add_arc(G, i, j);
            if a_cap >= 0 as libc::c_int {
                memcpy(
                    ((*a).data as *mut libc::c_char).offset(a_cap as isize)
                        as *mut libc::c_void,
                    &mut cap as *mut libc::c_double as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
            }
            _glp_dmx_end_of_line(csa);
            k += 1;
            k;
        }
        glp_printf(
            b"%d lines were read\n\0" as *const u8 as *const libc::c_char,
            (*csa).count,
        );
    }
    if ret != 0 {
        glp_erase_graph(G, (*G).v_size, (*G).a_size);
    }
    if !((*csa).fp).is_null() {
        _glp_close((*csa).fp);
    }
    return ret;
}
