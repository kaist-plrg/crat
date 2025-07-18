use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn clock() -> clock_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn cd(a: K) -> K;
    static mut fer: I;
    static mut fer1: I;
    fn show(a: K) -> K;
    fn prompt(n: I) -> I;
    fn enumerate(a: K) -> K;
    fn wd(s: S, n: libc::c_int) -> K;
    fn ex(a: K) -> K;
    fn X(s: S) -> K;
    static mut PPON: C;
    static mut PPMAX: I;
    static mut PP: I;
    static mut d_: S;
    static mut SEED: I;
    static mut KTREE: K;
    fn kerr(s: cS) -> K;
    fn Ki(x: I) -> K;
    fn _n() -> K;
    fn newK(t: I, n: I) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn seedPRNG(s: I);
    fn StoI(s: S, n: *mut I) -> I;
    static mut fCmplt: I;
    fn getline_(s: *mut S, n: *mut I, f: *mut FILE) -> I;
    fn wds(a: *mut K, f: *mut FILE) -> I;
    fn wds_(a: *mut K, f: *mut FILE, l: I) -> I;
    fn lines(f: *mut FILE) -> I;
    fn oerr() -> I;
    static mut tmr_ival: I;
    static mut khome: [C; 0];
    fn sp(k: S) -> S;
    fn denameD(d: *mut K, t: S, create: I) -> *mut K;
    static mut scrLim: I;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type clock_t = __clock_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type UI = libc::c_ulonglong;
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type F = libc::c_double;
pub type C = libc::c_char;
pub type S = *mut C;
pub type cS = *const C;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct k0 {
    pub _c: I,
    pub t: I,
    pub n: I,
    pub k: [*mut k0; 1],
}
pub type K = *mut k0;
pub static mut fLoad: I = 0 as libc::c_int as I;
pub static mut fError: I = 1 as libc::c_int as I;
pub static mut fWksp: I = 0 as libc::c_int as I;
pub static mut fBreak: S = b"n\0" as *const u8 as *const libc::c_char as S;
pub unsafe extern "C" fn boilerplate() {
    if isatty(fileno(stdout)) == 0 || isatty(fileno(stdin)) == 0 {
        return;
    }
    printf(
        b"kona      \\ for help. \\\\ to exit.\n\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn filexist(mut s: S) -> K {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut b: [C; 4097] = [0; 4097];
    let mut p: S = 0 as *mut C;
    let mut n: I = 0 as libc::c_int as I;
    strcpy(b.as_mut_ptr(), s as *const libc::c_char);
    loop {
        if !(strstr(b.as_mut_ptr(), b".k\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            f = fopen(b.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
            if !f.is_null() {
                p = b.as_mut_ptr();
                break;
            }
        }
        strcat(b.as_mut_ptr(), b".k\0" as *const u8 as *const libc::c_char);
        f = fopen(b.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
        if !f.is_null() {
            p = b.as_mut_ptr();
            break;
        } else if n == 0 {
            n = strlen(khome.as_mut_ptr()) as I;
            if (n as libc::c_ulonglong)
                .wrapping_add(strlen(s as *const libc::c_char) as libc::c_ulonglong)
                .wrapping_add(2 as libc::c_int as libc::c_ulonglong)
                > 4096 as libc::c_int as libc::c_ulonglong
            {
                return 0 as K;
            }
            strcpy(b.as_mut_ptr(), khome.as_mut_ptr());
            strcpy(b.as_mut_ptr().offset(n as isize), s as *const libc::c_char);
        } else {
            return 0 as K
        }
    }
    fclose(f);
    let mut kp: K = newK(
        -(3 as libc::c_int) as I,
        strlen(p as *const libc::c_char) as I,
    );
    if OOM_CD(0 as libc::c_int as I, kp, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    strcpy(((*kp).k).as_mut_ptr() as *mut C, p as *const libc::c_char);
    return kp;
}
unsafe extern "C" fn loadf(mut s: S) -> *mut FILE {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut p: K = filexist(s);
    if !p.is_null() {
        f = fopen(
            ((*p).k).as_mut_ptr() as *mut C,
            b"r\0" as *const u8 as *const libc::c_char,
        );
    }
    if p.is_null() || f.is_null() {
        show(kerr(b"file\0" as *const u8 as *const libc::c_char));
    }
    cd(p);
    return f;
}
pub unsafe extern "C" fn load(mut s: S) -> K {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut ofLoad: I = fLoad;
    let mut ofCmplt: I = fCmplt;
    let mut old_: S = 0 as *mut C;
    let mut r: K = 0 as *mut k0;
    fLoad = 1 as libc::c_int as I;
    fCmplt = 0 as libc::c_int as I;
    old_ = d_;
    if (filexist(s)).is_null() {
        printf(b"%s: file not found\n\0" as *const u8 as *const libc::c_char, s);
        r = _n();
    } else if scrLim > 124 as libc::c_int as libc::c_longlong {
        printf(b"limit\n\0" as *const u8 as *const libc::c_char);
        r = kerr(b"stack\0" as *const u8 as *const libc::c_char);
    } else {
        scrLim += 1;
        scrLim;
        f = loadf(s);
        if f.is_null() {
            printf(b"%s.k: file not found\n\0" as *const u8 as *const libc::c_char, s);
            r = kerr(b"file\0" as *const u8 as *const libc::c_char);
        } else {
            lines(f);
            if fclose(f) != 0 {
                r = kerr(b"file\0" as *const u8 as *const libc::c_char);
            } else {
                scrLim -= 1;
                scrLim;
                if fCmplt == 1 as libc::c_int as libc::c_longlong {
                    kerr(b"open-in-next-line\0" as *const u8 as *const libc::c_char);
                    oerr();
                }
                kerr(b"(nil)\0" as *const u8 as *const libc::c_char);
                fer1 = 0 as libc::c_int as I;
                fer = fer1;
                r = _n();
            }
        }
    }
    fLoad = ofLoad;
    fCmplt = ofCmplt;
    d_ = old_;
    return r;
}
pub unsafe extern "C" fn stepopt(mut s: S, mut n: I) -> I {
    if n == 1 as libc::c_int as libc::c_longlong && *s as libc::c_int == '\n' as i32 {
        return 0 as libc::c_int as I
    } else if n == 1 as libc::c_int as libc::c_longlong
        && *s as libc::c_int == '/' as i32
        || n == 2 as libc::c_int as libc::c_longlong && *s as libc::c_int == '/' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
    {
        return 1 as libc::c_int as I
    } else if n == 1 as libc::c_int as libc::c_longlong
        && *s as libc::c_int == '\\' as i32
        || n == 2 as libc::c_int as libc::c_longlong && *s as libc::c_int == '\\' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
    {
        return 2 as libc::c_int as I
    } else {
        return 3 as libc::c_int as I
    };
}
pub unsafe extern "C" fn precision(mut n: UI) -> K {
    if n > PPMAX as libc::c_ulonglong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    PPON = (n != 0 as libc::c_int as libc::c_ulonglong) as libc::c_int as C;
    PP = (if PPON as libc::c_int != 0 { n } else { PPMAX as libc::c_ulonglong }) as I;
    return _n();
}
unsafe extern "C" fn precision_() -> K {
    return Ki(
        if PPON as libc::c_int != 0 { PP } else { 0 as libc::c_int as libc::c_longlong },
    );
}
pub unsafe extern "C" fn backslash(mut s: S, mut n: I, mut dict: *mut K) -> K {
    let mut t: S = 0 as *mut C;
    let mut b: C = 0;
    if 1 as libc::c_int as libc::c_longlong == n {
        printf(
            b"Backslash Commands:\n\\0        datatypes help\n\\+        verb help\n\\'        adverb help\n\\:        I/O verb help\n\\_        reserved word help\n\\.        assignment/amend, function, control flow help\n\\b [s|t]  show/set break mode (stop|trace|none)\n\\d [d|^]  change k directory (^=previous)\n\\e [n]    show/set error flag (0=off,1=on,2=exit)\n\\l f      load script f or f.k\n\\p [n]    show/set print precision (0=full)\n\\r [s]    show/set random seed (0=random)\n\\s f      step script f or f.k\n\\t [n]    show/set timer interval in msec (0=disable)\n          calls niladic .m.ts\n\\t e      measure runtime of some k expression\n\\v [d|^]  show k directory (^=previous)\n\\w        show workspace resources used\n\\[cmd]    system command (also \\[ cmd]), \\echo hello\n\\\\        exit (or ctrl+d)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return _n();
    } else {
        b = *s.offset(2 as libc::c_int as isize);
        if b == 0
            || *(*__ctype_b_loc()).offset(b as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            t = s
                .offset(
                    (if b as libc::c_int != 0 {
                        3 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) as isize,
                );
            match *s.offset(1 as libc::c_int as isize) as libc::c_int {
                92 => {
                    exit(0 as libc::c_int);
                }
                39 => {
                    printf(
                        b"Adverbs / \\ ' /: \\: ':\nUse adverbs in conjunction with verbs/functions\nOver and scan behave differently depending on whether their left argument\nis monadic or dyadic.\n/  over dyad    +/1 2 3  or 4+/1 2 3    */1+!5\n\\  scan dyad    +\\3 4 5  or 6+\\3 4 5    *\\10#2\n/  over monad   apply until fixed    f/x    (%%[;2.0])/9999.0 \n/  over monad   apply n times      n f/x    4 (2+)/0 \n/  over monad   apply while true   b f/x    {x<10} {x+1}/ 3\n\\  scan monad   trace until repeat   f\\x    (1!)\\1 2 3 4\n\\  scan monad   trace n times      n f\\x    10(|+\\)\\1 1\n/  over         {x+y+z}/[1 2 3;4;7 8 9]  f/[x;y;z]\n\\  scan         {x+y+z}[1 2 3;4;7 8 9]  f\\[x;y;z]\n/  join         \",\"/(\"a\";\"b\")\n\\  split        \",\"\\\"a,b\"\n'  each         \"abc\" ,' \"def\"    join each  \n'  each         !:' 2 3 4    enumerate each  \n/: eachright    2 #/: (\"alpha\";\"bravo\";\"charlie\")    take each right\n\\: eachleft     0 1 2 3 #\\: \"abc\"     take each left\n': eachpair     apply pairwise  -':1 3 4 8 10\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return _n();
                }
                43 => {
                    printf(
                        b"Verbs +-*%%|&^!<>=~@?_,#$.:\nVerbs work on all sensible types (ints, floats, strings, symbols, lists)\nDyadic or monadic is determined from context, default is dyadic\nAdd : after a verb to force the monadic form, + is plus, +: is flip\n+ monadic  flip. transpose a matrix (a depth-2 list)\n+ dyadic   plus. add numbers together\n- monadic  negate. invert sign\n- dyadic   minus. subtraction\n* monadic  first. first element from the list\n* dyadic   times. multiply two numbers\n%% monadic  reciprocal. 1 over x \n%% dyadic   divide. x divided by y (not mod) \n| monadic  reverse. reverse order of list\n| dyadic   max/or. MAX(x,y) or OR(x,y) \n& monadic  where. &0 0 1 0 3 yields 2 4 4 4. \n& dyadic   min/and. MIN(x,y) or AND(x,y) \n^ monadic  shape. #elts at each depth (min over) \n^ dyadic   power. x to the exponent y \n! monadic  enumerate. !4 yields 0 1 2 3\n! dyadic   mod/rotate. 5!3 yields 2;  1 ! 4 5 6 yields 5 6 4 \n< monadic  grade up. indices of list sorted ascending \n< dyadic   less. boolean is x less than y\n> monadic  grade down. indices of list sorted descending\n> dyadic   more. boolean is x greater than y\n= monadic  group. =3 4 3 4 4 yields (0 2;1 3 4)  \n= dyadic   equals. 1 2 3 = 1 2 4 yields 1 1 0 (tolerantly) \n~ monadic  not. ~ 0 1 0 2 0 yields 1 0 1 0 1 \n~ dyadic   match. 1 2 ~ 1 2 yields 1 (types must match)\n@ monadic  atom. boolean is arg an atom (as opposed to a list) \n@ dyadic   at. elts from x at indices y\n@ triadic  monadic amend. see \\. \n@ tetradic dyadic amend. see \\.\n? monadic  unique. distinct elts from a list \n? dyadic   find. x?y yields index of y in list x (or #x)\n? dyadic   invert. {x^2} ? 2 yields sqrt(2) \n? triadic  invert-guess. secant method with clues ?[{2^x};17;4]\n_ monadic  floor. tolerant floor function \n_ dyadic   drop/cut. lose x elts from list y / separate into pieces \n, monadic  enlist. put x inside a 1-element list \n, dyadic   join. \"ab\",\"cd\" yields \"abcd\"\n# monadic  count. number of elements in list \n# dyadic   take/reshape. fill x elts from y \n$ monadic  format. cast to string \n$ dyadic   form/format. cast \n. monadic  execute/make dictionary. .\"1+1\" yields 2 \n. dyadic   value. 1 2 3 . ,2 yields 3. see \\. \n. triadic  monadic amend. see \\. \n. tetradic dyadic amend. see \\. \n:          overloaded with many operations. \n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return _n();
                }
                46 => {
                    printf(
                        b"Assign/Amend, Functions, Control Flow \n\nAssign/Amend (see https://github.com/kevinlawler/kona/wiki/Amend)\na:1 is assignment\na::1 is global assignment (works in functions)\na+:1 is like a+=1, works in general\na-: negates a in place, works in general\na[]:1 sets all the entries of a to 1\na[0]:1 sets the 0th entry of a to 1\na[0;1]+:2 increments the 0th entry's 1st entry by 2\n.[a;();+;1] returns a+1 but does not affect a on the K Tree\n.[`a;();+;1] updates a in place, returns `a\n\nAmend Equivalence\n@[a;b;c;d] is .[a;,b;c;d]\na:1        is .[`a;();:;1]\na+:1       is .[`a;();+;1]\na-:        is .[`a;();-:]\na[]:1      is .[`a;_n;:;1]\na[0]:1     is .[`a;0;:;1] or .[`a;,0;:;1] \na[0;1]+:2  is .[`a;0 1;+;2]\n\nError Trap\n@[a;b;:] and .[a;b;:] are error trap\n\nFunctions\nf:{[a;b;c] a+b+c} defines a function. f[1;2;3] yields 6\nFunctions may be anonymous.\nFunctions may have default arguments x,y,z.\nSo {x^2} 3 conveniently yields 9\n\nControl Flow\n:[x1;t1;x2;t2;...;xn;tn;else] evaluate xi until true and return ti, otherwise return else \n    :[0;10;0;20;1;30;40] yields 30\nif[x;e1;...;en] if x then evaluate all e. if[j>i;a:1;b:2] \ndo[m;e1;...;en] do all e m times. do[100;f[i];i+:1] \nwhile[x;e1;...;en] while x do e.  while[a>b; f a; a-:1] \n/ starts a comment. Must begin a line or have a space before\n\\ is trace when beginning an expression inside a function (todo)\n: is early return when beginning an expression inside a function\n' is signal (todo)\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return _n();
                }
                95 => {
                    printf(
                        b"Constants:\n(Note: the K epoch is 2035-01-01T00:00:00)\n_T       [current UTC Julian day count]+[fraction complete]\n_a       arguments\n_c       message source address\n_d       K-Tree path / current working dictionary\n_f       anonymous reference to current function\n_h       host name\n_i       index of current amendment\n_k       build date as string\n_n       nil\n_p       host port\n_s       space used allocated mmapped max\n_t       current UTC time (int)\n_v       current global variable under amendment\n_w       message source handle\n\nMonadic Verbs:\n_acos    inverse cosine\n_asin    inverse sine\n_atan    inverse tangent\n_ceil    ceiling (intolerant)\n_ceiling ceiling (tolerant)\n_cos     cosine\n_cosh    hyperbolic cosine\n_exp     exponential\n_floor   largest previous integer (intolerant)\n_log     natural logarithm\n_sin     sine\n_sinh    hyperbolic sine\n_sqr     square\n_sqrt    square root\n_tan     tangent\n_tanh    hyperbolic tangent\n_abs     absolute value\n_bd      convert to binary representation\n_ci      char-of-int (octal if unprintable char)\n_db      construct from binary representation\n_dj      date from Julian day count\n_exit    exit with status x\n_getenv  get an environment variable\n_gtime   in UTC, ints: YYYMMDD,HHMMSS (_gtime _t)\n_host    host name IP address (int)\n_ic      int-of-char\n_inv     inverse of a matrix\n_jd      Julian day count from date _jd 20110315\n_lt      convert output of _t to local time\n_ltime   localized version of _gtime\n_size    size of file (bytes)\n\nDyadic Verbs:\n_bin     index of element using binary search\n_binl    search for several elements\n_di      delete element at index\n_dot     dot product\n_draw    draw x random numbers from 0 to y-1, negative y indicates without replacement\n_dv      delete value\n_dvl     delete several values\n_hash    hash, (x;_hash x)?y\n_hat     caret/without, x _hat y\n_in      true if x is in y\n_lin     _in for several values\n_lsq     matrix division\n_mul     matrix multiplication\n_setenv  set environment variable\n_sm      string match\n_ss      positions of substring y in string x\n_sv      scalar from vector with base change\n_vs      vector from scalar with base change\n_vsx     vector from scalar with base change - extended\n\nTriadic Verbs:\n_ssr     string search & replace\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return _n();
                }
                48 => {
                    printf(
                        b"Datatypes  -4 -3 -2 -1 0 1 2 3 4 5 6 7\nMonadic 4: reveals type, 4:1 2 3 yields -1\n-4 vector symbol     `a`b`c or ,`a\n-3 vector character  \"abc\" or ,\"c\" \n-2 vector float      1.0 2.0 3.33 or ,1.0\n-1 vector integer    1 2 3 or ,1\n 0 list   general    (`a;1 2 3) or (`a;(1 2 3;(3 4;\"c\")))\n 1 scalar integer    1\n 2 scalar float      1.0\n 3 scalar character  \"c\" \n 4 scalar symbol     `s\n 5 dictionary        .((`a;10;);(`b;20;))  or  .()  or  .,(`a;5;) \n 6 nil               _n or (;;) (list of 3 nils)  \n 7 verbs/functions   +  +: {1+x}  +[1;]  (|+)  {[a;b]1+a+b}  {x+y}[1;]\nEmpty Lists:\n-4 0#`\n-3 \"\"\n-2 0#0.0\n-1 !0\n 0 ()\nSpecial numeric types:\n 0N null integer\n 0n null float\n-0I infinity integer negative\n 0I infinity integer positive\n-0i infinity float   negative\n 0i infinity float   positive\nDictionaries:\nStart by making a dictionary d[`k]:4\nd[]   values\n!d    keys\nd[`k] value at k\nd@`k  value at k\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return _n();
                }
                58 => {
                    printf(
                        b"I/O Verbs 0: 1: 2: 3: 4: 5: 6:\n\nDisk\n0: dyadic   write text file `f 0: \"hello\" \n0: monadic  read text file  0: `f\n1: dyadic   write binary file `f 1: 4 5 6 \n1: monadic  read binary file  1: `f (mmapped)\n2: monadic  read binary file  2: `f (copied to memory)\n3: dyadic   append to binary file, w/o sync `f 3: ,7\n5: dyadic   append to binary file `f 5: ,7\n6: dyadic   write raw byte string `f 6: \"\\01\\02\\03\"\n6: monadic  read raw byte string  6: `f\n\nNetwork\nStart k listening for IPC on port 1234  ./k -i 1234\n3: monadic  open handle    h: 3:(`\"192.168.1.101\";1234) \n3: monadic  close handle   3: h \n            exec .m.c expression\n3: dyadic   asynchronous send, returns _n      h 3: \"a:2\"\n            calls monadic .m.s msg handler\n4: dyadic   synchronous send, returns result   h 4: \"1+1\"\n            calls monadic .m.g msg handler\n\nOther\n0: dyadic   write to console `0: \"hello\\n\" \n2: dyadic   dynamically load C function  a:`libfile 2:(`func,3); a[1;2;3]\n4: monadic  type of data [-4,7],  4: \"c\" /returns 3\n5: monadic  printed representation. 5:1 2 3 /returns \"1 2 3\"\n\n0: and 1: both have versions for reading fields\nIn all cases `f can instead be (`f;start;length)\nRead fixed-width fields: \"cbsijfd IFCSDTZMm\" (\" \" is ignore)\n(types;widths)0:`f    (\"IFC\";3 5 4)0:`f  /read rows like \"20 30.1 golf\\n\" \n(types;widths)1:`f \n\"c\" 1:`f for c in \"cid\", read bytes/ints/doubles\nLoad delimited text file (no column names):\n(types;delim)0:`f    \nLoad delimited text file (with column names):\n(types;,delim)0:`f   \nc 1-byte char, b 1-byte int, s 2-byte int, i 4-byte int, f 4-byte float,\nd 8-byte double, \" \" 1-byte skip, I int, F float, C string, S string (sym), DTZMm Y? \n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return _n();
                }
                96 => return kerr(b"nyi\0" as *const u8 as *const libc::c_char),
                97 => return kerr(b"nyi\0" as *const u8 as *const libc::c_char),
                98 => return backslash_b(s, n),
                99 => return kerr(b"nyi\0" as *const u8 as *const libc::c_char),
                100 => return backslash_d(s, n, dict),
                101 => return backslash_e(s, n),
                105 => return kerr(b"nyi\0" as *const u8 as *const libc::c_char),
                108 => return load(t),
                109 => return kerr(b"nyi\0" as *const u8 as *const libc::c_char),
                112 => {
                    if *t != 0 {
                        let mut p: I = 0;
                        if StoI(t, &mut p) == 0 {
                            return kerr(b"type\0" as *const u8 as *const libc::c_char);
                        }
                        return precision(p as UI);
                    } else {
                        return precision_()
                    }
                }
                114 => {
                    if *t != 0 {
                        let mut r: I = 0;
                        if StoI(t, &mut r) == 0 {
                            return kerr(b"type\0" as *const u8 as *const libc::c_char);
                        }
                        seedPRNG(r);
                        return _n();
                    } else {
                        seedPRNG(SEED);
                        return Ki(SEED);
                    }
                }
                115 => return backslash_s(t),
                116 => return backslash_t(t),
                118 => return backslash_v(s, n, dict),
                119 => return backslash_w(s),
                _ => {}
            }
            printf(b"domain error\n\0" as *const u8 as *const libc::c_char);
            return _n();
        }
    }
    if *(*__ctype_b_loc())
        .offset(*s.offset(1 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    return if system(s as *const libc::c_char) != 0 {
        kerr(b"domain\0" as *const u8 as *const libc::c_char)
    } else {
        _n()
    };
}
unsafe extern "C" fn backslash_b(mut s: S, mut n: I) -> K {
    if n == 2 as libc::c_int as libc::c_longlong {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, fBreak);
        return _n();
    }
    if n == 4 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int
            == *(b"n\0" as *const u8 as *const libc::c_char) as libc::c_int
    {
        fBreak = b"n\0" as *const u8 as *const libc::c_char as S;
        return _n();
    }
    if n == 4 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int
            == *(b"t\0" as *const u8 as *const libc::c_char) as libc::c_int
    {
        fBreak = b"t\0" as *const u8 as *const libc::c_char as S;
        return _n();
    }
    if n == 4 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int
            == *(b"s\0" as *const u8 as *const libc::c_char) as libc::c_int
    {
        fBreak = b"s\0" as *const u8 as *const libc::c_char as S;
        return _n();
    }
    printf(b"valid options are: n, s, t\n\0" as *const u8 as *const libc::c_char);
    return _n();
}
unsafe extern "C" fn backslash_d(mut s: S, mut n: I, mut dict: *mut K) -> K {
    let mut z: [C; 256] = [0; 256];
    if n == 2 as libc::c_int as libc::c_longlong {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, d_);
        return _n();
    }
    if n == 4 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        d_ = sp(b"\0" as *const u8 as *const libc::c_char as S);
        return _n();
    }
    if n == 4 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int == '^' as i32
    {
        if strlen(d_ as *const libc::c_char) == 0 as libc::c_int as libc::c_ulong {
            return _n();
        }
        if strlen(d_ as *const libc::c_char) == 2 as libc::c_int as libc::c_ulong {
            d_ = sp(b"\0" as *const u8 as *const libc::c_char as S);
            return _n();
        }
        if strlen(d_ as *const libc::c_char) > 3 as libc::c_int as libc::c_ulong {
            let mut c: I = 0 as libc::c_int as I;
            let mut i: I = 0 as libc::c_int as I;
            i = 0 as libc::c_int as I;
            while (i as libc::c_ulonglong)
                < strlen(d_ as *const libc::c_char) as libc::c_ulonglong
            {
                if *d_.offset(i as isize) as libc::c_int == '.' as i32 {
                    c = i;
                }
                i += 1;
                i;
            }
            strcpy(z.as_mut_ptr(), d_ as *const libc::c_char);
            z[c as usize] = '\0' as i32 as C;
            d_ = sp(z.as_mut_ptr());
            return _n();
        }
    }
    if n == 5 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *s.offset(4 as libc::c_int as isize) as libc::c_int == 'k' as i32
    {
        d_ = sp(b".k\0" as *const u8 as *const libc::c_char as S);
        return _n();
    }
    if n == 5 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *s.offset(4 as libc::c_int as isize) as libc::c_int != 'k' as i32
    {
        printf(
            b"absolute backslash-d should begin with .k\n\0" as *const u8
                as *const libc::c_char,
        );
        return _n();
    }
    if *(*__ctype_b_loc())
        .offset(*s.offset(3 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        denameD(dict, s.offset(3 as libc::c_int as isize), 1 as libc::c_int as I);
        strcpy(z.as_mut_ptr(), d_ as *const libc::c_char);
        strcat(z.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
        strcat(
            z.as_mut_ptr(),
            s.offset(3 as libc::c_int as isize) as *const libc::c_char,
        );
        d_ = sp(z.as_mut_ptr());
        return _n();
    }
    if n >= 6 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *s.offset(4 as libc::c_int as isize) as libc::c_int == 'k' as i32
        && *s.offset(5 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        denameD(&mut KTREE, s.offset(3 as libc::c_int as isize), 1 as libc::c_int as I);
        d_ = sp(s.offset(3 as libc::c_int as isize));
        return _n();
    }
    if *s.offset(3 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        denameD(&mut KTREE, s.offset(3 as libc::c_int as isize), 1 as libc::c_int as I);
        d_ = sp(s.offset(3 as libc::c_int as isize));
        return _n();
    }
    return kerr(b"nyi\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn backslash_v(mut s: S, mut n: I, mut dict: *mut K) -> K {
    if n > 2 as libc::c_int as libc::c_longlong
        && *s.offset(2 as libc::c_int as isize) as libc::c_int == '\r' as i32
    {
        return kerr(b"syntax\0" as *const u8 as *const libc::c_char);
    }
    let mut z: [C; 256] = [0; 256];
    z[0 as libc::c_int as usize] = '\0' as i32 as C;
    if 2 as libc::c_int as libc::c_longlong == n {
        strcpy(z.as_mut_ptr(), d_ as *const libc::c_char);
    }
    if 4 as libc::c_int as libc::c_longlong == n
        && *s.offset(3 as libc::c_int as isize) as libc::c_int
            == *(b"^\0" as *const u8 as *const libc::c_char) as libc::c_int
    {
        if strlen(d_ as *const libc::c_char) > 3 as libc::c_int as libc::c_ulong {
            let mut c: I = 0 as libc::c_int as I;
            let mut i: I = 0 as libc::c_int as I;
            i = 0 as libc::c_int as I;
            while (i as libc::c_ulonglong)
                < strlen(d_ as *const libc::c_char) as libc::c_ulonglong
            {
                if *d_.offset(i as isize) as libc::c_int
                    == *(b".\0" as *const u8 as *const libc::c_char) as libc::c_int
                {
                    c = i;
                }
                i += 1;
                i;
            }
            strcpy(z.as_mut_ptr(), d_ as *const libc::c_char);
            z[c as usize] = *(b"\0\0" as *const u8 as *const libc::c_char);
        } else {
            return _n()
        }
    }
    if *(*__ctype_b_loc())
        .offset(*s.offset(3 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        strcpy(z.as_mut_ptr(), d_ as *const libc::c_char);
        strcat(z.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
        strcat(
            z.as_mut_ptr(),
            s.offset(3 as libc::c_int as isize) as *const libc::c_char,
        );
    }
    if *z.as_mut_ptr() != 0 {
        let mut x: K = *denameD(&mut KTREE, z.as_mut_ptr(), 0 as libc::c_int as I);
        return if 6 as libc::c_int as libc::c_longlong == (*x).t {
            _n()
        } else {
            enumerate(x)
        };
    }
    return kerr(b"nyi\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn backslash_e(mut s: S, mut n: I) -> K {
    if n == 2 as libc::c_int as libc::c_longlong {
        printf(b"%lld\n\0" as *const u8 as *const libc::c_char, fError);
        return _n();
    }
    if n == 4 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int
            == *(b"0\0" as *const u8 as *const libc::c_char) as libc::c_int
    {
        fError = 0 as libc::c_int as I;
        return _n();
    }
    if n == 4 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int
            == *(b"1\0" as *const u8 as *const libc::c_char) as libc::c_int
    {
        fError = 1 as libc::c_int as I;
        return _n();
    }
    if n == 4 as libc::c_int as libc::c_longlong
        && *s.offset(3 as libc::c_int as isize) as libc::c_int
            == *(b"2\0" as *const u8 as *const libc::c_char) as libc::c_int
    {
        fError = 2 as libc::c_int as I;
        return _n();
    }
    printf(b"valid options are: 0, 1, 2\n\0" as *const u8 as *const libc::c_char);
    return _n();
}
unsafe extern "C" fn backslash_s(mut s: S) -> K {
    let mut t: S = 0 as *mut C;
    let mut u: S = 0 as S;
    let mut w: S = 0 as *mut C;
    let mut c: I = 0 as libc::c_int as I;
    let mut d: I = 0;
    let mut n: I = 0;
    let mut m: I = 0 as libc::c_int as I;
    let mut l: I = 0 as libc::c_int as I;
    let mut r: I = 0;
    let mut f: *mut FILE = loadf(s);
    let mut k: K = 0 as K;
    let mut y: K = 0 as K;
    let mut z: K = 0 as K;
    if f.is_null() {
        return _n();
    }
    's_23: loop {
        c = wds(&mut y, f);
        if !((0 as libc::c_int as libc::c_longlong) < c) {
            break;
        }
        n = (*y).n;
        t = ((*y).k).as_mut_ptr() as *mut C;
        w = t;
        loop {
            let fresh0 = w;
            w = w.offset(1);
            if !(*(*__ctype_b_loc()).offset(*fresh0 as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
            {
                break;
            }
            l += 1;
            l;
        }
        if l == n || n == 0 {
            if !y.is_null() {
                cd(y);
            }
            y = 0 as K;
        } else {
            printf(b"%s \0" as *const u8 as *const libc::c_char, t);
            if -(1 as libc::c_int) as libc::c_longlong == getline_(&mut u, &mut m, stdin)
            {
                break;
            }
            d = stepopt(u, m);
            if d == 1 as libc::c_int as libc::c_longlong {
                if !y.is_null() {
                    cd(y);
                }
                y = 0 as K;
            } else {
                if d == 2 as libc::c_int as libc::c_longlong {
                    break;
                }
                k = ex(wd(t, n as libc::c_int));
                show(k);
                if !k.is_null() {
                    cd(k);
                    k = 0 as K;
                }
                if !y.is_null() {
                    cd(y);
                    y = 0 as K;
                }
                loop {
                    prompt(1 as libc::c_int as I);
                    if 0 as libc::c_int as libc::c_longlong
                        > wds_(&mut z, stdin, 1 as libc::c_int as I)
                    {
                        break 's_23;
                    }
                    w = ((*z).k).as_mut_ptr() as *mut C;
                    l = (*z).n;
                    d = stepopt(w, l);
                    if d == 1 as libc::c_int as libc::c_longlong {
                        if !z.is_null() {
                            cd(z);
                        }
                        z = 0 as K;
                        break;
                    } else {
                        if d == 2 as libc::c_int as libc::c_longlong {
                            break 's_23;
                        }
                        k = ex(wd(w, l as libc::c_int));
                        show(k);
                        if !k.is_null() {
                            cd(k);
                            k = 0 as K;
                        }
                        if !z.is_null() {
                            cd(z);
                            z = 0 as K;
                        }
                        if !(d == 0 as libc::c_int as libc::c_longlong
                            || d == 3 as libc::c_int as libc::c_longlong)
                        {
                            break;
                        }
                    }
                }
            }
        }
    }
    r = fclose(f) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    free(u as *mut libc::c_void);
    if !k.is_null() {
        cd(k);
    }
    if !y.is_null() {
        cd(y);
    }
    if !z.is_null() {
        cd(z);
    }
    return _n();
}
unsafe extern "C" fn backslash_t(mut s: S) -> K {
    let mut r: I = 0;
    if *s != 0 {
        if StoI(s, &mut r) != 0 {
            tmr_ival = r;
            return _n();
        }
        let mut d: I = clock() as I;
        let mut z: K = X(s);
        d = ((clock() as libc::c_longlong - d) as libc::c_double
            / (1000000 as libc::c_int as __clock_t as F
                / 1000 as libc::c_int as libc::c_double)) as I;
        cd(z);
        return Ki(d);
    }
    return Ki(tmr_ival);
}
unsafe extern "C" fn backslash_w(mut s: S) -> K {
    fWksp = 1 as libc::c_int as I;
    return _n();
}
