use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprefix(
        ps: *mut libc::c_char,
        s: *mut libc::c_char,
        uppercase: libc::c_int,
    ) -> libc::c_int;
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn isnullenv(s: *mut libc::c_char) -> libc::c_int;
    fn opt_o(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt__O(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_j(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_shift(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_k(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_ks(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_t(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt__T(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_p(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt__P(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_b(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_i(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt__V(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_D(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_x(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_quote(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_rscroll(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_query(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_mousecap(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_wheel_lines(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_linenum_width(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_status_col_width(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_filesize(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_intr(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_header(type_0: libc::c_int, s: *mut libc::c_char);
    fn opt_search_type(type_0: libc::c_int, s: *mut libc::c_char);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loption {
    pub oletter: libc::c_char,
    pub onames: *mut optname,
    pub otype: libc::c_int,
    pub odefault: libc::c_int,
    pub ovar: *mut libc::c_int,
    pub ofunc: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> ()>,
    pub odesc: [*mut libc::c_char; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optname {
    pub oname: *mut libc::c_char,
    pub onext: *mut optname,
}
pub static mut quiet: libc::c_int = 0;
pub static mut no_vbell: libc::c_int = 0;
pub static mut how_search: libc::c_int = 0;
pub static mut top_scroll: libc::c_int = 0;
pub static mut pr_type: libc::c_int = 0;
pub static mut bs_mode: libc::c_int = 0;
pub static mut know_dumb: libc::c_int = 0;
pub static mut quit_at_eof: libc::c_int = 0;
pub static mut quit_if_one_screen: libc::c_int = 0;
pub static mut squeeze: libc::c_int = 0;
pub static mut tabstop: libc::c_int = 0;
pub static mut back_scroll: libc::c_int = 0;
pub static mut forw_scroll: libc::c_int = 0;
pub static mut caseless: libc::c_int = 0;
pub static mut linenums: libc::c_int = 0;
pub static mut autobuf: libc::c_int = 0;
pub static mut bufspace: libc::c_int = 0;
pub static mut ctldisp: libc::c_int = 0;
pub static mut force_open: libc::c_int = 0;
pub static mut swindow: libc::c_int = 0;
pub static mut jump_sline: libc::c_int = 0;
pub static mut jump_sline_fraction: libc::c_long = -(1 as libc::c_int) as libc::c_long;
pub static mut shift_count_fraction: libc::c_long = -(1 as libc::c_int) as libc::c_long;
pub static mut chopline: libc::c_int = 0;
pub static mut wordwrap: libc::c_int = 0;
pub static mut no_init: libc::c_int = 0;
pub static mut no_keypad: libc::c_int = 0;
pub static mut twiddle: libc::c_int = 0;
pub static mut show_attn: libc::c_int = 0;
pub static mut shift_count: libc::c_int = 0;
pub static mut status_col: libc::c_int = 0;
pub static mut use_lessopen: libc::c_int = 0;
pub static mut quit_on_intr: libc::c_int = 0;
pub static mut follow_mode: libc::c_int = 0;
pub static mut oldbot: libc::c_int = 0;
pub static mut opt_use_backslash: libc::c_int = 0;
pub static mut rscroll_char: libc::c_char = 0;
pub static mut rscroll_attr: libc::c_int = 0;
pub static mut no_hist_dups: libc::c_int = 0;
pub static mut mousecap: libc::c_int = 0;
pub static mut wheel_lines: libc::c_int = 0;
pub static mut perma_marks: libc::c_int = 0;
pub static mut linenum_width: libc::c_int = 0;
pub static mut status_col_width: libc::c_int = 0;
pub static mut incr_search: libc::c_int = 0;
pub static mut use_color: libc::c_int = 0;
pub static mut want_filesize: libc::c_int = 0;
pub static mut status_line: libc::c_int = 0;
pub static mut header_lines: libc::c_int = 0;
pub static mut header_cols: libc::c_int = 0;
pub static mut nonum_headers: libc::c_int = 0;
pub static mut nosearch_headers: libc::c_int = 0;
pub static mut redraw_on_quit: libc::c_int = 0;
pub static mut def_search_type: libc::c_int = 0;
pub static mut exit_F_on_close: libc::c_int = 0;
pub static mut modelines: libc::c_int = 0;
pub static mut show_preproc_error: libc::c_int = 0;
pub static mut proc_backspace: libc::c_int = 0;
pub static mut proc_tab: libc::c_int = 0;
pub static mut proc_return: libc::c_int = 0;
pub static mut intr_char: libc::c_char = ('X' as i32 & 0o37 as libc::c_int)
    as libc::c_char;
pub static mut hilite_search: libc::c_int = 0;
pub static mut less_is_more: libc::c_int = 0 as libc::c_int;
static mut a_optname: optname = {
    let mut init = optname {
        oname: b"search-skip-screen\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut b_optname: optname = {
    let mut init = optname {
        oname: b"buffers\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut B__optname: optname = {
    let mut init = optname {
        oname: b"auto-buffers\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut c_optname: optname = {
    let mut init = optname {
        oname: b"clear-screen\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut d_optname: optname = {
    let mut init = optname {
        oname: b"dumb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut D__optname: optname = {
    let mut init = optname {
        oname: b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut e_optname: optname = {
    let mut init = optname {
        oname: b"quit-at-eof\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut f_optname: optname = {
    let mut init = optname {
        oname: b"force\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut F__optname: optname = {
    let mut init = optname {
        oname: b"quit-if-one-screen\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut g_optname: optname = {
    let mut init = optname {
        oname: b"hilite-search\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut h_optname: optname = {
    let mut init = optname {
        oname: b"max-back-scroll\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut i_optname: optname = {
    let mut init = optname {
        oname: b"ignore-case\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut j_optname: optname = {
    let mut init = optname {
        oname: b"jump-target\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut J__optname: optname = {
    let mut init = optname {
        oname: b"status-column\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut k_optname: optname = {
    let mut init = optname {
        oname: b"lesskey-file\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut ks_optname: optname = {
    let mut init = optname {
        oname: b"lesskey-src\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut K__optname: optname = {
    let mut init = optname {
        oname: b"quit-on-intr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut L__optname: optname = {
    let mut init = optname {
        oname: b"no-lessopen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut m_optname: optname = {
    let mut init = optname {
        oname: b"long-prompt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut n_optname: optname = {
    let mut init = optname {
        oname: b"line-numbers\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut o_optname: optname = {
    let mut init = optname {
        oname: b"log-file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut O__optname: optname = {
    let mut init = optname {
        oname: b"LOG-FILE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut p_optname: optname = {
    let mut init = optname {
        oname: b"pattern\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut P__optname: optname = {
    let mut init = optname {
        oname: b"prompt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut q2_optname: optname = {
    let mut init = optname {
        oname: b"silent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut q_optname: optname = unsafe {
    {
        let mut init = optname {
            oname: b"quiet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            onext: &q2_optname as *const optname as *mut optname,
        };
        init
    }
};
static mut r_optname: optname = {
    let mut init = optname {
        oname: b"raw-control-chars\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut s_optname: optname = {
    let mut init = optname {
        oname: b"squeeze-blank-lines\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut S__optname: optname = {
    let mut init = optname {
        oname: b"chop-long-lines\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut t_optname: optname = {
    let mut init = optname {
        oname: b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut T__optname: optname = {
    let mut init = optname {
        oname: b"tag-file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut u_optname: optname = {
    let mut init = optname {
        oname: b"underline-special\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut V__optname: optname = {
    let mut init = optname {
        oname: b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut w_optname: optname = {
    let mut init = optname {
        oname: b"hilite-unread\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut x_optname: optname = {
    let mut init = optname {
        oname: b"tabs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut X__optname: optname = {
    let mut init = optname {
        oname: b"no-init\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut y_optname: optname = {
    let mut init = optname {
        oname: b"max-forw-scroll\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut z_optname: optname = {
    let mut init = optname {
        oname: b"window\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut quote_optname: optname = {
    let mut init = optname {
        oname: b"quotes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut tilde_optname: optname = {
    let mut init = optname {
        oname: b"tilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut query_optname: optname = {
    let mut init = optname {
        oname: b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut pound_optname: optname = {
    let mut init = optname {
        oname: b"shift\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut keypad_optname: optname = {
    let mut init = optname {
        oname: b"no-keypad\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut oldbot_optname: optname = {
    let mut init = optname {
        oname: b"old-bot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut follow_optname: optname = {
    let mut init = optname {
        oname: b"follow-name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut use_backslash_optname: optname = {
    let mut init = optname {
        oname: b"use-backslash\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut rscroll_optname: optname = {
    let mut init = optname {
        oname: b"rscroll\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut nohistdups_optname: optname = {
    let mut init = optname {
        oname: b"no-histdups\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut mousecap_optname: optname = {
    let mut init = optname {
        oname: b"mouse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut wheel_lines_optname: optname = {
    let mut init = optname {
        oname: b"wheel-lines\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut perma_marks_optname: optname = {
    let mut init = optname {
        oname: b"save-marks\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut linenum_width_optname: optname = {
    let mut init = optname {
        oname: b"line-num-width\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut status_col_width_optname: optname = {
    let mut init = optname {
        oname: b"status-col-width\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut incr_search_optname: optname = {
    let mut init = optname {
        oname: b"incsearch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut use_color_optname: optname = {
    let mut init = optname {
        oname: b"use-color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut want_filesize_optname: optname = {
    let mut init = optname {
        oname: b"file-size\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut status_line_optname: optname = {
    let mut init = optname {
        oname: b"status-line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut header_optname: optname = {
    let mut init = optname {
        oname: b"header\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut nonum_headers_optname: optname = {
    let mut init = optname {
        oname: b"no-number-headers\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut nosearch_headers_optname: optname = {
    let mut init = optname {
        oname: b"no-search-headers\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut redraw_on_quit_optname: optname = {
    let mut init = optname {
        oname: b"redraw-on-quit\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut search_type_optname: optname = {
    let mut init = optname {
        oname: b"search-options\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut exit_F_on_close_optname: optname = {
    let mut init = optname {
        oname: b"exit-follow-on-close\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut modelines_optname: optname = {
    let mut init = optname {
        oname: b"modelines\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut no_vbell_optname: optname = {
    let mut init = optname {
        oname: b"no-vbell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut intr_optname: optname = {
    let mut init = optname {
        oname: b"intr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut wordwrap_optname: optname = {
    let mut init = optname {
        oname: b"wordwrap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut show_preproc_error_optname: optname = {
    let mut init = optname {
        oname: b"show-preproc-errors\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut proc_backspace_optname: optname = {
    let mut init = optname {
        oname: b"proc-backspace\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut proc_tab_optname: optname = {
    let mut init = optname {
        oname: b"proc-tab\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut proc_return_optname: optname = {
    let mut init = optname {
        oname: b"proc-return\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        onext: 0 as *const optname as *mut optname,
    };
    init
};
static mut option: [loption; 71] = unsafe {
    [
        {
            let mut init = loption {
                oletter: 'a' as i32 as libc::c_char,
                onames: &a_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int,
                odefault: 2 as libc::c_int,
                ovar: &how_search as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Search includes displayed screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Search skips displayed screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Search includes all of displayed screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'b' as i32 as libc::c_char,
                onames: &b_optname as *const optname as *mut optname,
                otype: 0o4 as libc::c_int | 0o1000 as libc::c_int,
                odefault: 64 as libc::c_int,
                ovar: &bufspace as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_b as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Max buffer space per file (K): \0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Max buffer space per file: %dK\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'B' as i32 as libc::c_char,
                onames: &B__optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 1 as libc::c_int,
                ovar: &autobuf as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't automatically allocate buffers\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Automatically allocate buffers when needed\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'c' as i32 as libc::c_char,
                onames: &c_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &top_scroll as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Repaint by scrolling from bottom of screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Repaint by painting from top of screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Repaint by painting from top of screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'd' as i32 as libc::c_char,
                onames: &d_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o100 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &know_dumb as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Assume intelligent terminal\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Assume dumb terminal\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'D' as i32 as libc::c_char,
                onames: &D__optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int | 0o40 as libc::c_int | 0o400 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_D as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"color desc: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'e' as i32 as libc::c_char,
                onames: &e_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &quit_at_eof as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't quit at end-of-file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Quit at end-of-file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Quit immediately at end-of-file\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'f' as i32 as libc::c_char,
                onames: &f_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &force_open as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Open only regular files\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Open even non-regular files\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'F' as i32 as libc::c_char,
                onames: &F__optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &quit_if_one_screen as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't quit if end-of-file on first screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Quit if end-of-file on first screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'g' as i32 as libc::c_char,
                onames: &g_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o200 as libc::c_int,
                odefault: 2 as libc::c_int,
                ovar: &hilite_search as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't highlight search matches\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Highlight matches for previous search only\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Highlight all matches for previous search pattern\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'h' as i32 as libc::c_char,
                onames: &h_optname as *const optname as *mut optname,
                otype: 0o4 as libc::c_int,
                odefault: -(1 as libc::c_int),
                ovar: &back_scroll as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Backwards scroll limit: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Backwards scroll limit is %d lines\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'i' as i32 as libc::c_char,
                onames: &i_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o200 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &caseless as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_i as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Case is significant in searches\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Ignore case in searches\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Ignore case in searches and in patterns\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'j' as i32 as libc::c_char,
                onames: &j_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_j as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Target line: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"0123456789.-\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'J' as i32 as libc::c_char,
                onames: &J__optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &status_col as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't display a status column\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Display a status column\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'k' as i32 as libc::c_char,
                onames: &k_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int | 0o100 as libc::c_int | 0o400 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_k as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &ks_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int | 0o100 as libc::c_int | 0o400 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_ks as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'K' as i32 as libc::c_char,
                onames: &K__optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &quit_on_intr as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Interrupt (ctrl-C) returns to prompt\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Interrupt (ctrl-C) exits less\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'L' as i32 as libc::c_char,
                onames: &L__optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 1 as libc::c_int,
                ovar: &use_lessopen as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't use the LESSOPEN filter\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Use the LESSOPEN filter\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'm' as i32 as libc::c_char,
                onames: &m_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &pr_type as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Short prompt\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Medium prompt\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Long prompt\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'n' as i32 as libc::c_char,
                onames: &n_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o40 as libc::c_int,
                odefault: 1 as libc::c_int,
                ovar: &linenums as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't use line numbers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Use line numbers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Constantly display line numbers\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'o' as i32 as libc::c_char,
                onames: &o_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_o as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"log file: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'O' as i32 as libc::c_char,
                onames: &O__optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt__O as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Log file: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'p' as i32 as libc::c_char,
                onames: &p_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int | 0o100 as libc::c_int | 0o400 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_p as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'P' as i32 as libc::c_char,
                onames: &P__optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt__P as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"prompt: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'q' as i32 as libc::c_char,
                onames: &q_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &quiet as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Ring the bell for errors AND at eof/bof\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Ring the bell for errors but not at eof/bof\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Never ring the bell\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'r' as i32 as libc::c_char,
                onames: &r_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &ctldisp as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Display control characters as ^X\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Display control characters directly (not recommended)\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"Display ANSI sequences directly, other control characters as ^X\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 's' as i32 as libc::c_char,
                onames: &s_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &squeeze as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Display all blank lines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Squeeze multiple blank lines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'S' as i32 as libc::c_char,
                onames: &S__optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &chopline as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Fold long lines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Chop long lines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 't' as i32 as libc::c_char,
                onames: &t_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int | 0o400 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_t as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"tag: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'T' as i32 as libc::c_char,
                onames: &T__optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt__T as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"tags file: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'u' as i32 as libc::c_char,
                onames: &u_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o40 as libc::c_int | 0o200 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &bs_mode as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Display underlined text in underline mode\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Backspaces cause overstrike\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Print backspace as ^H\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'V' as i32 as libc::c_char,
                onames: &V__optname as *const optname as *mut optname,
                otype: 0o20 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt__V as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'w' as i32 as libc::c_char,
                onames: &w_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &show_attn as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't highlight first unread line\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Highlight first unread line after forward-screen\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Highlight first unread line after any forward movement\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'x' as i32 as libc::c_char,
                onames: &x_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_x as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Tab stops: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"0123456789,\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'X' as i32 as libc::c_char,
                onames: &X__optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o100 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &no_init as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Send init/deinit strings to terminal\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Don't use init/deinit strings\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'y' as i32 as libc::c_char,
                onames: &y_optname as *const optname as *mut optname,
                otype: 0o4 as libc::c_int,
                odefault: -(1 as libc::c_int),
                ovar: &forw_scroll as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Forward scroll limit: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Forward scroll limit is %d lines\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: 'z' as i32 as libc::c_char,
                onames: &z_optname as *const optname as *mut optname,
                otype: 0o4 as libc::c_int,
                odefault: -(1 as libc::c_int),
                ovar: &swindow as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Scroll window size: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Scroll window size is %d lines\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '"' as i32 as libc::c_char,
                onames: &quote_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_quote
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"quotes: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '~' as i32 as libc::c_char,
                onames: &tilde_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 1 as libc::c_int,
                ovar: &twiddle as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't show tildes after end of file\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Show tildes after end of file\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '?' as i32 as libc::c_char,
                onames: &query_optname as *const optname as *mut optname,
                otype: 0o20 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_query
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '#' as i32 as libc::c_char,
                onames: &pound_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_shift
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Horizontal shift: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"0123456789.\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &keypad_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o100 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &no_keypad as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Use keypad mode\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Don't use keypad mode\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &oldbot_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &oldbot as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Use new bottom of screen behavior\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Use old bottom of screen behavior\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &follow_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &follow_mode as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"F command follows file descriptor\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"F command follows file name\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &use_backslash_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &opt_use_backslash as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Use backslash escaping in command line parameters\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Don't use backslash escaping in command line parameters\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &rscroll_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int | 0o40 as libc::c_int | 0o1000 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_rscroll
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"rscroll character: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &nohistdups_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &no_hist_dups as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Allow duplicates in history list\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Remove duplicates from history list\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &mousecap_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &mousecap as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_mousecap
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Ignore mouse input\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Use the mouse for scrolling\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Use the mouse for scrolling (reverse)\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &wheel_lines_optname as *const optname as *mut optname,
                otype: 0o4 as libc::c_int | 0o1000 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &wheel_lines as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_wheel_lines
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Lines to scroll on mouse wheel: \0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Scroll %d line(s) on mouse wheel\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &perma_marks_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &perma_marks as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't save marks in history file\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Save marks in history file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &linenum_width_optname as *const optname as *mut optname,
                otype: 0o4 as libc::c_int | 0o40 as libc::c_int,
                odefault: 7 as libc::c_int,
                ovar: &linenum_width as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_linenum_width
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Line number width: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Line number width is %d chars\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &status_col_width_optname as *const optname as *mut optname,
                otype: 0o4 as libc::c_int | 0o40 as libc::c_int,
                odefault: 2 as libc::c_int,
                ovar: &status_col_width as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_status_col_width
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Status column width: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Status column width is %d chars\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &incr_search_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &incr_search as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Incremental search is off\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Incremental search is on\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &use_color_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &use_color as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't use color\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Use color\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &want_filesize_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &want_filesize as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_filesize
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Don't get size of each file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Get size of each file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &status_line_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &status_line as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't color each line with its status column color\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Color each line with its status column color\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &header_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_header
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Header lines: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &nonum_headers_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &nonum_headers as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Number header lines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Don't number header lines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &nosearch_headers_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o200 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &nosearch_headers as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Search includes header lines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Search does not include header lines\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &redraw_on_quit_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &redraw_on_quit as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't redraw screen when quitting\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Redraw last screen when quitting\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &search_type_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_search_type
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"Search options: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &exit_F_on_close_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &exit_F_on_close as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't exit F command when input closes\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Exit F command when input closes\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &no_vbell_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &no_vbell as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Display visual bell\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Don't display visual bell\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &modelines_optname as *const optname as *mut optname,
                otype: 0o4 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &modelines as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Lines to read looking for modelines: \0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Read %d lines looking for modelines\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &intr_optname as *const optname as *mut optname,
                otype: 0o10 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: Some(
                    opt_intr
                        as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
                ),
                odesc: [
                    b"interrupt character: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &wordwrap_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int | 0o40 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &wordwrap as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Wrap lines at any character\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Wrap lines at spaces\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &show_preproc_error_optname as *const optname as *mut optname,
                otype: 0o1 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &show_preproc_error as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Don't show error message if preprocessor fails\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Show error message if preprocessor fails\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &proc_backspace_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o40 as libc::c_int | 0o200 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &proc_backspace as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Backspace handling is specified by the -U option\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Display underline text in underline mode\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Print backspaces as ^H\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &proc_tab_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o40 as libc::c_int | 0o200 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &proc_tab as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Tab handling is specified by the -U option\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Expand tabs to spaces\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Print tabs as ^I\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\u{1}' as i32 as libc::c_char,
                onames: &proc_return_optname as *const optname as *mut optname,
                otype: 0o2 as libc::c_int | 0o40 as libc::c_int | 0o200 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: &proc_return as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    b"Carriage return handling is specified by the -U option\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"Delete carriage return before newline\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    b"Print carriage return as ^M\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = loption {
                oletter: '\0' as i32 as libc::c_char,
                onames: 0 as *const optname as *mut optname,
                otype: 0o20 as libc::c_int,
                odefault: 0 as libc::c_int,
                ovar: 0 as *const libc::c_int as *mut libc::c_int,
                ofunc: None,
                odesc: [
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char as *mut libc::c_char,
                ],
            };
            init
        },
    ]
};
pub unsafe extern "C" fn init_option() {
    let mut o: *mut loption = 0 as *mut loption;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = lgetenv(
        b"LESS_IS_MORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if isnullenv(p) == 0 {
        less_is_more = 1 as libc::c_int;
    }
    o = option.as_mut_ptr();
    while (*o).oletter as libc::c_int != '\0' as i32 {
        if !((*o).ovar).is_null() {
            *(*o).ovar = (*o).odefault;
        }
        if (*o).otype & 0o1000 as libc::c_int != 0 {
            (Some(((*o).ofunc).unwrap()))
                .unwrap()(0 as libc::c_int, 0 as *mut libc::c_void as *mut libc::c_char);
        }
        o = o.offset(1);
        o;
    }
}
pub unsafe extern "C" fn findopt(mut c: libc::c_int) -> *mut loption {
    let mut o: *mut loption = 0 as *mut loption;
    o = option.as_mut_ptr();
    while (*o).oletter as libc::c_int != '\0' as i32 {
        if (*o).oletter as libc::c_int == c {
            return o;
        }
        if (*o).otype & 0o2 as libc::c_int != 0
            && (*o).oletter as libc::c_int - 'a' as i32 + 'A' as i32 == c
        {
            return o;
        }
        o = o.offset(1);
        o;
    }
    return 0 as *mut loption;
}
unsafe extern "C" fn is_optchar(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
        return 1 as libc::c_int;
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
        return 1 as libc::c_int;
    }
    if c as libc::c_int == '-' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn findopt_name(
    mut p_optname_0: *mut *mut libc::c_char,
    mut p_oname: *mut *mut libc::c_char,
    mut p_err: *mut libc::c_int,
) -> *mut loption {
    let mut optname: *mut libc::c_char = *p_optname_0;
    let mut o: *mut loption = 0 as *mut loption;
    let mut oname: *mut optname = 0 as *mut optname;
    let mut len: libc::c_int = 0;
    let mut uppercase: libc::c_int = 0;
    let mut maxo: *mut loption = 0 as *mut loption;
    let mut maxoname: *mut optname = 0 as *mut optname;
    let mut maxlen: libc::c_int = 0 as libc::c_int;
    let mut ambig: libc::c_int = 0 as libc::c_int;
    let mut exact: libc::c_int = 0 as libc::c_int;
    o = option.as_mut_ptr();
    while (*o).oletter as libc::c_int != '\0' as i32 {
        oname = (*o).onames;
        while !oname.is_null() {
            uppercase = 0 as libc::c_int;
            while uppercase <= 1 as libc::c_int {
                len = sprefix(optname, (*oname).oname, uppercase);
                if !(len <= 0 as libc::c_int
                    || is_optchar(*optname.offset(len as isize)) != 0)
                {
                    if exact == 0 && len == maxlen {
                        ambig = 1 as libc::c_int;
                    } else if len > maxlen {
                        maxo = o;
                        maxoname = oname;
                        maxlen = len;
                        ambig = 0 as libc::c_int;
                        exact = (len == strlen((*oname).oname) as libc::c_int)
                            as libc::c_int;
                    }
                    if (*o).otype & 0o2 as libc::c_int == 0 {
                        break;
                    }
                }
                uppercase += 1;
                uppercase;
            }
            oname = (*oname).onext;
        }
        o = o.offset(1);
        o;
    }
    if ambig != 0 {
        if !p_err.is_null() {
            *p_err = 1 as libc::c_int;
        }
        return 0 as *mut loption;
    }
    *p_optname_0 = optname.offset(maxlen as isize);
    if !p_oname.is_null() {
        *p_oname = if maxoname.is_null() {
            0 as *mut libc::c_char
        } else {
            (*maxoname).oname
        };
    }
    return maxo;
}
