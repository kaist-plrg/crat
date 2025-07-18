use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut no_builtin_variables_flag: libc::c_int;
    static mut no_builtin_rules_flag: libc::c_int;
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    fn enter_file(name: *const libc::c_char) -> *mut file;
    fn enter_prereqs(prereqs: *mut dep, stem: *const libc::c_char) -> *mut dep;
    static mut current_variable_set_list: *mut variable_set_list;
    fn define_variable_in_set(
        name: *const libc::c_char,
        length: size_t,
        value: *const libc::c_char,
        origin: variable_origin,
        recursive: libc::c_int,
        set: *mut variable_set,
        flocp: *const floc,
    ) -> *mut variable;
    fn undefine_variable_in_set(
        name: *const libc::c_char,
        length: size_t,
        origin: variable_origin,
        set: *mut variable_set,
    );
    static mut suffix_file: *mut file;
    fn install_pattern_rule(p: *mut pspec, terminal: libc::c_int);
    fn parse_file_seq(
        stringp: *mut *mut libc::c_char,
        size: size_t,
        stopmap: libc::c_int,
        prefix: *const libc::c_char,
        flags: libc::c_int,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const libc::c_char,
    pub hname: *const libc::c_char,
    pub vpath: *const libc::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const libc::c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: libc::c_uint,
    pub command_flags: libc::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type cmd_state = libc::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = libc::c_uint;
pub const us_failed: update_status = 3;
pub const us_question: update_status = 2;
pub const us_none: update_status = 1;
pub const us_success: update_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: libc::c_ulong,
    pub ht_capacity: libc::c_ulong,
    pub ht_fill: libc::c_ulong,
    pub ht_empty_slots: libc::c_ulong,
    pub ht_collisions: libc::c_ulong,
    pub ht_lookups: libc::c_ulong,
    pub ht_rehashes: libc::c_uint,
}
pub type hash_cmp_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hash_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const libc::c_char,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const libc::c_char,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct commands {
    pub fileinfo: floc,
    pub commands: *mut libc::c_char,
    pub command_lines: *mut *mut libc::c_char,
    pub lines_flags: *mut libc::c_uchar,
    pub ncommand_lines: libc::c_ushort,
    pub recipe_prefix: libc::c_char,
    #[bitfield(name = "any_recurse", ty = "libc::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
pub const o_invalid: variable_origin = 7;
pub const o_automatic: variable_origin = 6;
pub const o_override: variable_origin = 5;
pub const o_command: variable_origin = 4;
pub const o_env_override: variable_origin = 3;
pub const o_file: variable_origin = 2;
pub const o_env: variable_origin = 1;
pub const o_default: variable_origin = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct variable {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub fileinfo: floc,
    pub length: libc::c_uint,
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "append", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "conditional", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "per_target", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "special", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "exportable", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "expanding", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "private_var", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "exp_count", ty = "libc::c_uint", bits = "8..=22")]
    #[bitfield(name = "flavor", ty = "variable_flavor", bits = "23..=25")]
    #[bitfield(name = "origin", ty = "variable_origin", bits = "26..=28")]
    #[bitfield(name = "export", ty = "variable_export", bits = "29..=30")]
    pub recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [u8; 4],
}
pub type variable_export = libc::c_uint;
pub const v_ifset: variable_export = 3;
pub const v_noexport: variable_export = 2;
pub const v_export: variable_export = 1;
pub const v_default: variable_export = 0;
pub type variable_origin = libc::c_uint;
pub type variable_flavor = libc::c_uint;
pub const f_append_value: variable_flavor = 7;
pub const f_shell: variable_flavor = 6;
pub const f_conditional: variable_flavor = 5;
pub const f_append: variable_flavor = 4;
pub const f_expand: variable_flavor = 3;
pub const f_recursive: variable_flavor = 2;
pub const f_simple: variable_flavor = 1;
pub const f_bogus: variable_flavor = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pspec {
    pub target: *const libc::c_char,
    pub dep: *const libc::c_char,
    pub commands: *const libc::c_char,
}
static mut default_suffixes: [libc::c_char; 147] = unsafe {
    *::std::mem::transmute::<
        &[u8; 147],
        &mut [libc::c_char; 147],
    >(
        b".out .a .ln .o .c .cc .C .cpp .p .f .F .m .r .y .l .ym .yl .s .S .mod .sym .def .h .info .dvi .tex .texinfo .texi .txinfo .w .ch .web .sh .elc .el\0",
    )
};
static mut default_pattern_rules: [pspec; 5] = [
    {
        let mut init = pspec {
            target: b"(%)\0" as *const u8 as *const libc::c_char,
            dep: b"%\0" as *const u8 as *const libc::c_char,
            commands: b"$(AR) $(ARFLAGS) $@ $<\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%.out\0" as *const u8 as *const libc::c_char,
            dep: b"%\0" as *const u8 as *const libc::c_char,
            commands: b"@rm -f $@ \n cp $< $@\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%.c\0" as *const u8 as *const libc::c_char,
            dep: b"%.w %.ch\0" as *const u8 as *const libc::c_char,
            commands: b"$(CTANGLE) $^ $@\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%.tex\0" as *const u8 as *const libc::c_char,
            dep: b"%.w %.ch\0" as *const u8 as *const libc::c_char,
            commands: b"$(CWEAVE) $^ $@\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: 0 as *const libc::c_char,
            dep: 0 as *const libc::c_char,
            commands: 0 as *const libc::c_char,
        };
        init
    },
];
static mut default_terminal_rules: [pspec; 6] = [
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const libc::c_char,
            dep: b"%,v\0" as *const u8 as *const libc::c_char,
            commands: b"$(CHECKOUT,v)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const libc::c_char,
            dep: b"RCS/%,v\0" as *const u8 as *const libc::c_char,
            commands: b"$(CHECKOUT,v)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const libc::c_char,
            dep: b"RCS/%\0" as *const u8 as *const libc::c_char,
            commands: b"$(CHECKOUT,v)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const libc::c_char,
            dep: b"s.%\0" as *const u8 as *const libc::c_char,
            commands: b"$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: b"%\0" as *const u8 as *const libc::c_char,
            dep: b"SCCS/s.%\0" as *const u8 as *const libc::c_char,
            commands: b"$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = pspec {
            target: 0 as *const libc::c_char,
            dep: 0 as *const libc::c_char,
            commands: 0 as *const libc::c_char,
        };
        init
    },
];
static mut default_suffix_rules: [*const libc::c_char; 100] = [
    b".o\0" as *const u8 as *const libc::c_char,
    b"$(LINK.o) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".s\0" as *const u8 as *const libc::c_char,
    b"$(LINK.s) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".S\0" as *const u8 as *const libc::c_char,
    b"$(LINK.S) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".c\0" as *const u8 as *const libc::c_char,
    b"$(LINK.c) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".cc\0" as *const u8 as *const libc::c_char,
    b"$(LINK.cc) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".C\0" as *const u8 as *const libc::c_char,
    b"$(LINK.C) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".cpp\0" as *const u8 as *const libc::c_char,
    b"$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".f\0" as *const u8 as *const libc::c_char,
    b"$(LINK.f) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".m\0" as *const u8 as *const libc::c_char,
    b"$(LINK.m) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".p\0" as *const u8 as *const libc::c_char,
    b"$(LINK.p) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".F\0" as *const u8 as *const libc::c_char,
    b"$(LINK.F) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".r\0" as *const u8 as *const libc::c_char,
    b"$(LINK.r) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const libc::c_char,
    b".mod\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.mod) -o $@ -e $@ $^\0" as *const u8 as *const libc::c_char,
    b".def.sym\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.def) -o $@ $<\0" as *const u8 as *const libc::c_char,
    b".sh\0" as *const u8 as *const libc::c_char,
    b"cat $< >$@ \n chmod a+x $@\0" as *const u8 as *const libc::c_char,
    b".s.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.s) -o $@ $<\0" as *const u8 as *const libc::c_char,
    b".S.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.S) -o $@ $<\0" as *const u8 as *const libc::c_char,
    b".c.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.c) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".cc.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.cc) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".C.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.C) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".cpp.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.cpp) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".f.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.f) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".m.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.m) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".p.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.p) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".F.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.F) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".r.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.r) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".mod.o\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.mod) -o $@ $<\0" as *const u8 as *const libc::c_char,
    b".c.ln\0" as *const u8 as *const libc::c_char,
    b"$(LINT.c) -C$* $<\0" as *const u8 as *const libc::c_char,
    b".y.ln\0" as *const u8 as *const libc::c_char,
    b"$(YACC.y) $< \n $(LINT.c) -C$* y.tab.c \n $(RM) y.tab.c\0" as *const u8
        as *const libc::c_char,
    b".l.ln\0" as *const u8 as *const libc::c_char,
    b"@$(RM) $*.c\n $(LEX.l) $< > $*.c\n$(LINT.c) -i $*.c -o $@\n $(RM) $*.c\0"
        as *const u8 as *const libc::c_char,
    b".y.c\0" as *const u8 as *const libc::c_char,
    b"$(YACC.y) $< \n mv -f y.tab.c $@\0" as *const u8 as *const libc::c_char,
    b".l.c\0" as *const u8 as *const libc::c_char,
    b"@$(RM) $@ \n $(LEX.l) $< > $@\0" as *const u8 as *const libc::c_char,
    b".ym.m\0" as *const u8 as *const libc::c_char,
    b"$(YACC.m) $< \n mv -f y.tab.c $@\0" as *const u8 as *const libc::c_char,
    b".lm.m\0" as *const u8 as *const libc::c_char,
    b"@$(RM) $@ \n $(LEX.m) $< > $@\0" as *const u8 as *const libc::c_char,
    b".F.f\0" as *const u8 as *const libc::c_char,
    b"$(PREPROCESS.F) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".r.f\0" as *const u8 as *const libc::c_char,
    b"$(PREPROCESS.r) $(OUTPUT_OPTION) $<\0" as *const u8 as *const libc::c_char,
    b".l.r\0" as *const u8 as *const libc::c_char,
    b"$(LEX.l) $< > $@ \n mv -f lex.yy.r $@\0" as *const u8 as *const libc::c_char,
    b".S.s\0" as *const u8 as *const libc::c_char,
    b"$(PREPROCESS.S) $< > $@\0" as *const u8 as *const libc::c_char,
    b".texinfo.info\0" as *const u8 as *const libc::c_char,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const libc::c_char,
    b".texi.info\0" as *const u8 as *const libc::c_char,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const libc::c_char,
    b".txinfo.info\0" as *const u8 as *const libc::c_char,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const libc::c_char,
    b".tex.dvi\0" as *const u8 as *const libc::c_char,
    b"$(TEX) $<\0" as *const u8 as *const libc::c_char,
    b".texinfo.dvi\0" as *const u8 as *const libc::c_char,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const libc::c_char,
    b".texi.dvi\0" as *const u8 as *const libc::c_char,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const libc::c_char,
    b".txinfo.dvi\0" as *const u8 as *const libc::c_char,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const libc::c_char,
    b".w.c\0" as *const u8 as *const libc::c_char,
    b"$(CTANGLE) $< - $@\0" as *const u8 as *const libc::c_char,
    b".web.p\0" as *const u8 as *const libc::c_char,
    b"$(TANGLE) $<\0" as *const u8 as *const libc::c_char,
    b".w.tex\0" as *const u8 as *const libc::c_char,
    b"$(CWEAVE) $< - $@\0" as *const u8 as *const libc::c_char,
    b".web.tex\0" as *const u8 as *const libc::c_char,
    b"$(WEAVE) $<\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut default_variables: [*const libc::c_char; 130] = [
    b"AR\0" as *const u8 as *const libc::c_char,
    b"ar\0" as *const u8 as *const libc::c_char,
    b"ARFLAGS\0" as *const u8 as *const libc::c_char,
    b"-rv\0" as *const u8 as *const libc::c_char,
    b"AS\0" as *const u8 as *const libc::c_char,
    b"as\0" as *const u8 as *const libc::c_char,
    b"CC\0" as *const u8 as *const libc::c_char,
    b"cc\0" as *const u8 as *const libc::c_char,
    b"OBJC\0" as *const u8 as *const libc::c_char,
    b"cc\0" as *const u8 as *const libc::c_char,
    b"CXX\0" as *const u8 as *const libc::c_char,
    b"g++\0" as *const u8 as *const libc::c_char,
    b"CHECKOUT,v\0" as *const u8 as *const libc::c_char,
    b"+$(if $(wildcard $@),,$(CO) $(COFLAGS) $< $@)\0" as *const u8
        as *const libc::c_char,
    b"CO\0" as *const u8 as *const libc::c_char,
    b"co\0" as *const u8 as *const libc::c_char,
    b"COFLAGS\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"CPP\0" as *const u8 as *const libc::c_char,
    b"$(CC) -E\0" as *const u8 as *const libc::c_char,
    b"FC\0" as *const u8 as *const libc::c_char,
    b"f77\0" as *const u8 as *const libc::c_char,
    b"F77\0" as *const u8 as *const libc::c_char,
    b"$(FC)\0" as *const u8 as *const libc::c_char,
    b"F77FLAGS\0" as *const u8 as *const libc::c_char,
    b"$(FFLAGS)\0" as *const u8 as *const libc::c_char,
    b"GET\0" as *const u8 as *const libc::c_char,
    b"get\0" as *const u8 as *const libc::c_char,
    b"LD\0" as *const u8 as *const libc::c_char,
    b"ld\0" as *const u8 as *const libc::c_char,
    b"LEX\0" as *const u8 as *const libc::c_char,
    b"lex\0" as *const u8 as *const libc::c_char,
    b"LINT\0" as *const u8 as *const libc::c_char,
    b"lint\0" as *const u8 as *const libc::c_char,
    b"M2C\0" as *const u8 as *const libc::c_char,
    b"m2c\0" as *const u8 as *const libc::c_char,
    b"PC\0" as *const u8 as *const libc::c_char,
    b"pc\0" as *const u8 as *const libc::c_char,
    b"YACC\0" as *const u8 as *const libc::c_char,
    b"yacc\0" as *const u8 as *const libc::c_char,
    b"MAKEINFO\0" as *const u8 as *const libc::c_char,
    b"makeinfo\0" as *const u8 as *const libc::c_char,
    b"TEX\0" as *const u8 as *const libc::c_char,
    b"tex\0" as *const u8 as *const libc::c_char,
    b"TEXI2DVI\0" as *const u8 as *const libc::c_char,
    b"texi2dvi\0" as *const u8 as *const libc::c_char,
    b"WEAVE\0" as *const u8 as *const libc::c_char,
    b"weave\0" as *const u8 as *const libc::c_char,
    b"CWEAVE\0" as *const u8 as *const libc::c_char,
    b"cweave\0" as *const u8 as *const libc::c_char,
    b"TANGLE\0" as *const u8 as *const libc::c_char,
    b"tangle\0" as *const u8 as *const libc::c_char,
    b"CTANGLE\0" as *const u8 as *const libc::c_char,
    b"ctangle\0" as *const u8 as *const libc::c_char,
    b"RM\0" as *const u8 as *const libc::c_char,
    b"rm -f\0" as *const u8 as *const libc::c_char,
    b"LINK.o\0" as *const u8 as *const libc::c_char,
    b"$(CC) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const libc::c_char,
    b"COMPILE.c\0" as *const u8 as *const libc::c_char,
    b"$(CC) $(CFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8
        as *const libc::c_char,
    b"LINK.c\0" as *const u8 as *const libc::c_char,
    b"$(CC) $(CFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"COMPILE.m\0" as *const u8 as *const libc::c_char,
    b"$(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8
        as *const libc::c_char,
    b"LINK.m\0" as *const u8 as *const libc::c_char,
    b"$(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"COMPILE.cc\0" as *const u8 as *const libc::c_char,
    b"$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8
        as *const libc::c_char,
    b"COMPILE.C\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.cc)\0" as *const u8 as *const libc::c_char,
    b"COMPILE.cpp\0" as *const u8 as *const libc::c_char,
    b"$(COMPILE.cc)\0" as *const u8 as *const libc::c_char,
    b"LINK.cc\0" as *const u8 as *const libc::c_char,
    b"$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"LINK.C\0" as *const u8 as *const libc::c_char,
    b"$(LINK.cc)\0" as *const u8 as *const libc::c_char,
    b"LINK.cpp\0" as *const u8 as *const libc::c_char,
    b"$(LINK.cc)\0" as *const u8 as *const libc::c_char,
    b"YACC.y\0" as *const u8 as *const libc::c_char,
    b"$(YACC) $(YFLAGS)\0" as *const u8 as *const libc::c_char,
    b"LEX.l\0" as *const u8 as *const libc::c_char,
    b"$(LEX) $(LFLAGS) -t\0" as *const u8 as *const libc::c_char,
    b"YACC.m\0" as *const u8 as *const libc::c_char,
    b"$(YACC) $(YFLAGS)\0" as *const u8 as *const libc::c_char,
    b"LEX.m\0" as *const u8 as *const libc::c_char,
    b"$(LEX) $(LFLAGS) -t\0" as *const u8 as *const libc::c_char,
    b"COMPILE.f\0" as *const u8 as *const libc::c_char,
    b"$(FC) $(FFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const libc::c_char,
    b"LINK.f\0" as *const u8 as *const libc::c_char,
    b"$(FC) $(FFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const libc::c_char,
    b"COMPILE.F\0" as *const u8 as *const libc::c_char,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8
        as *const libc::c_char,
    b"LINK.F\0" as *const u8 as *const libc::c_char,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"COMPILE.r\0" as *const u8 as *const libc::c_char,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const libc::c_char,
    b"LINK.r\0" as *const u8 as *const libc::c_char,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"COMPILE.def\0" as *const u8 as *const libc::c_char,
    b"$(M2C) $(M2FLAGS) $(DEFFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"COMPILE.mod\0" as *const u8 as *const libc::c_char,
    b"$(M2C) $(M2FLAGS) $(MODFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"COMPILE.p\0" as *const u8 as *const libc::c_char,
    b"$(PC) $(PFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8
        as *const libc::c_char,
    b"LINK.p\0" as *const u8 as *const libc::c_char,
    b"$(PC) $(PFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"LINK.s\0" as *const u8 as *const libc::c_char,
    b"$(CC) $(ASFLAGS) $(LDFLAGS) $(TARGET_MACH)\0" as *const u8 as *const libc::c_char,
    b"COMPILE.s\0" as *const u8 as *const libc::c_char,
    b"$(AS) $(ASFLAGS) $(TARGET_MACH)\0" as *const u8 as *const libc::c_char,
    b"LINK.S\0" as *const u8 as *const libc::c_char,
    b"$(CC) $(ASFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_MACH)\0" as *const u8
        as *const libc::c_char,
    b"COMPILE.S\0" as *const u8 as *const libc::c_char,
    b"$(CC) $(ASFLAGS) $(CPPFLAGS) $(TARGET_MACH) -c\0" as *const u8
        as *const libc::c_char,
    b"PREPROCESS.S\0" as *const u8 as *const libc::c_char,
    b"$(CPP) $(CPPFLAGS)\0" as *const u8 as *const libc::c_char,
    b"PREPROCESS.F\0" as *const u8 as *const libc::c_char,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -F\0" as *const u8
        as *const libc::c_char,
    b"PREPROCESS.r\0" as *const u8 as *const libc::c_char,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -F\0" as *const u8 as *const libc::c_char,
    b"LINT.c\0" as *const u8 as *const libc::c_char,
    b"$(LINT) $(LINTFLAGS) $(CPPFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const libc::c_char,
    b"OUTPUT_OPTION\0" as *const u8 as *const libc::c_char,
    b"-o $@\0" as *const u8 as *const libc::c_char,
    b".LIBPATTERNS\0" as *const u8 as *const libc::c_char,
    b"lib%.so lib%.a\0" as *const u8 as *const libc::c_char,
    b"GNUMAKEFLAGS\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub unsafe extern "C" fn set_default_suffixes() {
    suffix_file = enter_file(
        strcache_add(b".SUFFIXES\0" as *const u8 as *const libc::c_char),
    );
    (*suffix_file).set_builtin(1 as libc::c_int as libc::c_uint);
    if no_builtin_rules_flag != 0 {
        define_variable_in_set(
            b"SUFFIXES\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"\0" as *const u8 as *const libc::c_char,
            o_default,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    } else {
        let mut d: *mut dep = 0 as *mut dep;
        let mut p: *const libc::c_char = default_suffixes.as_mut_ptr();
        (*suffix_file)
            .deps = enter_prereqs(
            parse_file_seq(
                &mut p as *mut *const libc::c_char as *mut *mut libc::c_char,
                ::std::mem::size_of::<dep>() as libc::c_ulong,
                0x1 as libc::c_int,
                0 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut dep,
            0 as *const libc::c_char,
        );
        d = (*suffix_file).deps;
        while !d.is_null() {
            (*(*d).file).set_builtin(1 as libc::c_int as libc::c_uint);
            d = (*d).next;
        }
        define_variable_in_set(
            b"SUFFIXES\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            default_suffixes.as_mut_ptr(),
            o_default,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    };
}
pub unsafe extern "C" fn install_default_suffix_rules() {
    let mut s: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if no_builtin_rules_flag != 0 {
        return;
    }
    s = default_suffix_rules.as_mut_ptr();
    while !(*s).is_null() {
        let mut f: *mut file = enter_file(
            strcache_add(*s.offset(0 as libc::c_int as isize)),
        );
        if ((*f).cmds).is_null() {
            (*f)
                .cmds = xmalloc(::std::mem::size_of::<commands>() as libc::c_ulong)
                as *mut commands;
            (*(*f).cmds).fileinfo.filenm = 0 as *const libc::c_char;
            (*(*f).cmds).commands = xstrdup(*s.offset(1 as libc::c_int as isize));
            (*(*f).cmds).command_lines = 0 as *mut *mut libc::c_char;
            (*(*f).cmds).recipe_prefix = '\t' as i32 as libc::c_char;
            (*f).set_builtin(1 as libc::c_int as libc::c_uint);
        }
        s = s.offset(2 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn install_default_implicit_rules() {
    let mut p: *mut pspec = 0 as *mut pspec;
    if no_builtin_rules_flag != 0 {
        return;
    }
    p = default_pattern_rules.as_mut_ptr();
    while !((*p).target).is_null() {
        install_pattern_rule(p, 0 as libc::c_int);
        p = p.offset(1);
        p;
    }
    p = default_terminal_rules.as_mut_ptr();
    while !((*p).target).is_null() {
        install_pattern_rule(p, 1 as libc::c_int);
        p = p.offset(1);
        p;
    }
}
pub unsafe extern "C" fn define_default_variables() {
    let mut s: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if no_builtin_variables_flag != 0 {
        return;
    }
    s = default_variables.as_mut_ptr();
    while !(*s).is_null() {
        define_variable_in_set(
            *s.offset(0 as libc::c_int as isize),
            strlen(*s.offset(0 as libc::c_int as isize)),
            *s.offset(1 as libc::c_int as isize),
            o_default,
            1 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        s = s.offset(2 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn undefine_default_variables() {
    let mut s: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    s = default_variables.as_mut_ptr();
    while !(*s).is_null() {
        undefine_variable_in_set(
            *s.offset(0 as libc::c_int as isize),
            strlen(*s.offset(0 as libc::c_int as isize)),
            o_default,
            0 as *mut variable_set,
        );
        s = s.offset(2 as libc::c_int as isize);
    }
}
