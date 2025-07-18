#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]
#![feature(thread_local)]


extern crate libc;
pub mod examples {
pub mod glpsol;
} // mod examples
pub mod src {
pub mod amd {
pub mod amd_1;
pub mod amd_2;
pub mod amd_aat;
pub mod amd_control;
pub mod amd_defaults;
pub mod amd_dump;
pub mod amd_info;
pub mod amd_order;
pub mod amd_post_tree;
pub mod amd_postorder;
pub mod amd_preprocess;
pub mod amd_valid;
} // mod amd
pub mod api {
pub mod advbas;
pub mod asnhall;
pub mod asnlp;
pub mod asnokalg;
pub mod ckasn;
pub mod ckcnf;
pub mod cplex;
pub mod cpp;
pub mod cpxbas;
pub mod graph;
pub mod gridgen;
pub mod intfeas1;
pub mod maxffalg;
pub mod maxflp;
pub mod mcflp;
pub mod mcfokalg;
pub mod mcfrelax;
pub mod minisat1;
pub mod mpl;
pub mod mps;
pub mod netgen;
pub mod npp;
pub mod pript;
pub mod prmip;
pub mod prob1;
pub mod prob2;
pub mod prob3;
pub mod prob4;
pub mod prob5;
pub mod prrngs;
pub mod prsol;
pub mod rdasn;
pub mod rdcc;
pub mod rdcnf;
pub mod rdipt;
pub mod rdmaxf;
pub mod rdmcf;
pub mod rdmip;
pub mod rdprob;
pub mod rdsol;
pub mod rmfgen;
pub mod strong;
pub mod topsort;
pub mod wcliqex;
pub mod weak;
pub mod wrasn;
pub mod wrcc;
pub mod wrcnf;
pub mod wript;
pub mod wrmaxf;
pub mod wrmcf;
pub mod wrmip;
pub mod wrprob;
pub mod wrsol;
} // mod api
pub mod bflib {
pub mod btf;
pub mod btfint;
pub mod fhv;
pub mod fhvint;
pub mod ifu;
pub mod luf;
pub mod lufint;
pub mod scf;
pub mod scfint;
pub mod sgf;
pub mod sva;
} // mod bflib
pub mod colamd {
pub mod colamd;
} // mod colamd
pub mod draft {
pub mod bfd;
pub mod bfx;
pub mod glpapi06;
pub mod glpapi07;
pub mod glpapi08;
pub mod glpapi09;
pub mod glpapi10;
pub mod glpapi12;
pub mod glpapi13;
pub mod glpios01;
pub mod glpios02;
pub mod glpios03;
pub mod glpios07;
pub mod glpios09;
pub mod glpios11;
pub mod glpios12;
pub mod glpipm;
pub mod glpmat;
pub mod glpscl;
pub mod glpssx01;
pub mod glpssx02;
pub mod lux;
} // mod draft
pub mod env {
pub mod alloc;
pub mod dlsup;
pub mod env;
pub mod error;
pub mod stdc;
pub mod stdout;
pub mod stream;
pub mod time;
pub mod tls;
} // mod env
pub mod intopt {
pub mod cfg;
pub mod cfg1;
pub mod cfg2;
pub mod clqcut;
pub mod covgen;
pub mod fpump;
pub mod gmicut;
pub mod gmigen;
pub mod mirgen;
pub mod spv;
} // mod intopt
pub mod minisat {
pub mod minisat;
} // mod minisat
pub mod misc {
pub mod avl;
pub mod bignum;
pub mod dimacs;
pub mod dmp;
pub mod ffalg;
pub mod fp2rat;
pub mod fvs;
pub mod gcd;
pub mod hbm;
pub mod jd;
pub mod keller;
pub mod ks;
pub mod mc13d;
pub mod mc21a;
pub mod mt1;
pub mod mygmp;
pub mod okalg;
pub mod qmd;
pub mod relax4;
pub mod rgr;
pub mod rng;
pub mod rng1;
pub mod round2n;
pub mod spm;
pub mod str2int;
pub mod str2num;
pub mod strspx;
pub mod strtrim;
pub mod triang;
pub mod wclique;
pub mod wclique1;
} // mod misc
pub mod mpl {
pub mod mpl1;
pub mod mpl2;
pub mod mpl3;
pub mod mpl4;
pub mod mpl5;
pub mod mpl6;
pub mod mplsql;
} // mod mpl
pub mod npp {
pub mod npp1;
pub mod npp2;
pub mod npp3;
pub mod npp4;
pub mod npp5;
pub mod npp6;
} // mod npp
pub mod proxy {
pub mod proxy;
pub mod proxy1;
} // mod proxy
pub mod simplex {
pub mod spxat;
pub mod spxchuzc;
pub mod spxchuzr;
pub mod spxlp;
pub mod spxnt;
pub mod spxprim;
pub mod spxprob;
pub mod spychuzc;
pub mod spychuzr;
pub mod spydual;
} // mod simplex
pub mod zlib {
pub mod adler32;
pub mod compress;
pub mod crc32;
pub mod deflate;
pub mod gzclose;
pub mod gzlib;
pub mod gzread;
pub mod gzwrite;
pub mod inffast;
pub mod inflate;
pub mod inftrees;
pub mod trees;
pub mod uncompr;
pub mod zio;
pub mod zutil;
} // mod zlib
} // mod src
