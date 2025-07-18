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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
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
pub unsafe extern "C" fn glp_read_mincost(
    mut G: *mut glp_graph,
    mut v_rhs: libc::c_int,
    mut a_low: libc::c_int,
    mut a_cap: libc::c_int,
    mut a_cost: libc::c_int,
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
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut rhs: libc::c_double = 0.;
    let mut low: libc::c_double = 0.;
    let mut cap: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    if v_rhs >= 0 as libc::c_int
        && v_rhs
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/rdmcf.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_mincost: v_rhs = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_rhs,
        );
    }
    if a_low >= 0 as libc::c_int
        && a_low
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/rdmcf.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_mincost: a_low = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_low,
        );
    }
    if a_cap >= 0 as libc::c_int
        && a_cap
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/rdmcf.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_mincost: a_cap = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cap,
        );
    }
    if a_cost >= 0 as libc::c_int
        && a_cost
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/rdmcf.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_mincost: a_cost = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cost,
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
            b"Reading min-cost flow problem data from '%s'...\n\0" as *const u8
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
            b"min\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            _glp_dmx_error(
                csa,
                b"wrong problem designator; 'min' expected\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut nv) == 0 as libc::c_int
            && nv >= 0 as libc::c_int)
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
        flag = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        memset(
            &mut *flag.offset(1 as libc::c_int as isize) as *mut libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
            (nv as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        if v_rhs >= 0 as libc::c_int {
            rhs = 0.0f64;
            i = 1 as libc::c_int;
            while i <= nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut libc::c_char).offset(v_rhs as isize)
                        as *mut libc::c_void,
                    &mut rhs as *mut libc::c_double as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                i += 1;
                i;
            }
        }
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
            if *flag.offset(i as isize) != 0 {
                _glp_dmx_error(
                    csa,
                    b"duplicate descriptor of node %d\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            _glp_dmx_read_field(csa);
            if _glp_str2num(((*csa).field).as_mut_ptr(), &mut rhs) != 0 as libc::c_int {
                _glp_dmx_error(
                    csa,
                    b"node supply/demand missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_check_int(csa, rhs);
            if v_rhs >= 0 as libc::c_int {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut libc::c_char).offset(v_rhs as isize)
                        as *mut libc::c_void,
                    &mut rhs as *mut libc::c_double as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
            }
            *flag.offset(i as isize) = 1 as libc::c_int as libc::c_char;
            _glp_dmx_end_of_line(csa);
        }
        glp_free(flag as *mut libc::c_void);
        flag = 0 as *mut libc::c_char;
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
            if !(_glp_str2num(((*csa).field).as_mut_ptr(), &mut low) == 0 as libc::c_int
                && low >= 0.0f64)
            {
                _glp_dmx_error(
                    csa,
                    b"lower bound of arc flow missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_check_int(csa, low);
            _glp_dmx_read_field(csa);
            if !(_glp_str2num(((*csa).field).as_mut_ptr(), &mut cap) == 0 as libc::c_int
                && cap >= low)
            {
                _glp_dmx_error(
                    csa,
                    b"upper bound of arc flow missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_check_int(csa, cap);
            _glp_dmx_read_field(csa);
            if _glp_str2num(((*csa).field).as_mut_ptr(), &mut cost) != 0 as libc::c_int {
                _glp_dmx_error(
                    csa,
                    b"per-unit cost of arc flow missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_check_int(csa, cost);
            a = glp_add_arc(G, i, j);
            if a_low >= 0 as libc::c_int {
                memcpy(
                    ((*a).data as *mut libc::c_char).offset(a_low as isize)
                        as *mut libc::c_void,
                    &mut low as *mut libc::c_double as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
            }
            if a_cap >= 0 as libc::c_int {
                memcpy(
                    ((*a).data as *mut libc::c_char).offset(a_cap as isize)
                        as *mut libc::c_void,
                    &mut cap as *mut libc::c_double as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
            }
            if a_cost >= 0 as libc::c_int {
                memcpy(
                    ((*a).data as *mut libc::c_char).offset(a_cost as isize)
                        as *mut libc::c_void,
                    &mut cost as *mut libc::c_double as *const libc::c_void,
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
    if !flag.is_null() {
        glp_free(flag as *mut libc::c_void);
    }
    return ret;
}
