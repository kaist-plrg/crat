#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]


extern crate f128;#[macro_use]
extern crate num_traits;
extern crate libc;
pub mod blas {
pub mod blas;
} // mod blas
pub mod block {
pub mod block;
pub mod file;
pub mod init;
} // mod block
pub mod bspline {
pub mod bspline;
pub mod greville;
} // mod bspline
pub mod bst {
pub mod avl;
pub mod bst;
pub mod rb;
pub mod trav;
} // mod bst
pub mod cblas {
pub mod caxpy;
pub mod ccopy;
pub mod cdotc_sub;
pub mod cdotu_sub;
pub mod cgbmv;
pub mod cgemm;
pub mod cgemv;
pub mod cgerc;
pub mod cgeru;
pub mod chbmv;
pub mod chemm;
pub mod chemv;
pub mod cher;
pub mod cher2;
pub mod cher2k;
pub mod cherk;
pub mod chpmv;
pub mod chpr;
pub mod chpr2;
pub mod cscal;
pub mod csscal;
pub mod cswap;
pub mod csymm;
pub mod csyr2k;
pub mod csyrk;
pub mod ctbmv;
pub mod ctbsv;
pub mod ctpmv;
pub mod ctpsv;
pub mod ctrmm;
pub mod ctrmv;
pub mod ctrsm;
pub mod ctrsv;
pub mod dasum;
pub mod daxpy;
pub mod dcopy;
pub mod ddot;
pub mod dgbmv;
pub mod dgemm;
pub mod dgemv;
pub mod dger;
pub mod dnrm2;
pub mod drot;
pub mod drotg;
pub mod drotm;
pub mod drotmg;
pub mod dsbmv;
pub mod dscal;
pub mod dsdot;
pub mod dspmv;
pub mod dspr;
pub mod dspr2;
pub mod dswap;
pub mod dsymm;
pub mod dsymv;
pub mod dsyr;
pub mod dsyr2;
pub mod dsyr2k;
pub mod dsyrk;
pub mod dtbmv;
pub mod dtbsv;
pub mod dtpmv;
pub mod dtpsv;
pub mod dtrmm;
pub mod dtrmv;
pub mod dtrsm;
pub mod dtrsv;
pub mod dzasum;
pub mod dznrm2;
pub mod icamax;
pub mod idamax;
pub mod isamax;
pub mod izamax;
pub mod sasum;
pub mod saxpy;
pub mod scasum;
pub mod scnrm2;
pub mod scopy;
pub mod sdot;
pub mod sdsdot;
pub mod sgbmv;
pub mod sgemm;
pub mod sgemv;
pub mod sger;
pub mod snrm2;
pub mod srot;
pub mod srotg;
pub mod srotm;
pub mod srotmg;
pub mod ssbmv;
pub mod sscal;
pub mod sspmv;
pub mod sspr;
pub mod sspr2;
pub mod sswap;
pub mod ssymm;
pub mod ssymv;
pub mod ssyr;
pub mod ssyr2;
pub mod ssyr2k;
pub mod ssyrk;
pub mod stbmv;
pub mod stbsv;
pub mod stpmv;
pub mod stpsv;
pub mod strmm;
pub mod strmv;
pub mod strsm;
pub mod strsv;
pub mod xerbla;
pub mod zaxpy;
pub mod zcopy;
pub mod zdotc_sub;
pub mod zdotu_sub;
pub mod zdscal;
pub mod zgbmv;
pub mod zgemm;
pub mod zgemv;
pub mod zgerc;
pub mod zgeru;
pub mod zhbmv;
pub mod zhemm;
pub mod zhemv;
pub mod zher;
pub mod zher2;
pub mod zher2k;
pub mod zherk;
pub mod zhpmv;
pub mod zhpr;
pub mod zhpr2;
pub mod zscal;
pub mod zswap;
pub mod zsymm;
pub mod zsyr2k;
pub mod zsyrk;
pub mod ztbmv;
pub mod ztbsv;
pub mod ztpmv;
pub mod ztpsv;
pub mod ztrmm;
pub mod ztrmv;
pub mod ztrsm;
pub mod ztrsv;
} // mod cblas
pub mod cdf {
pub mod beta;
pub mod betainv;
pub mod binomial;
pub mod cauchy;
pub mod cauchyinv;
pub mod chisq;
pub mod chisqinv;
pub mod exponential;
pub mod exponentialinv;
pub mod exppow;
pub mod fdist;
pub mod fdistinv;
pub mod flat;
pub mod flatinv;
pub mod gamma;
pub mod gammainv;
pub mod gauss;
pub mod gaussinv;
pub mod geometric;
pub mod gumbel1;
pub mod gumbel1inv;
pub mod gumbel2;
pub mod gumbel2inv;
pub mod hypergeometric;
pub mod laplace;
pub mod laplaceinv;
pub mod logistic;
pub mod logisticinv;
pub mod lognormal;
pub mod lognormalinv;
pub mod nbinomial;
pub mod pareto;
pub mod paretoinv;
pub mod pascal;
pub mod poisson;
pub mod rayleigh;
pub mod rayleighinv;
pub mod tdist;
pub mod tdistinv;
pub mod weibull;
pub mod weibullinv;
} // mod cdf
pub mod cheb {
pub mod deriv;
pub mod eval;
pub mod init;
pub mod integ;
} // mod cheb
pub mod combination {
pub mod combination;
pub mod file;
pub mod init;
pub mod inline;
} // mod combination
pub mod complex {
pub mod inline;
pub mod math;
} // mod complex
pub mod deriv {
pub mod deriv;
} // mod deriv
pub mod dht {
pub mod dht;
} // mod dht
pub mod diff {
pub mod diff;
} // mod diff
pub mod eigen {
pub mod francis;
pub mod gen;
pub mod genherm;
pub mod genhermv;
pub mod gensymm;
pub mod gensymmv;
pub mod genv;
pub mod herm;
pub mod hermv;
pub mod jacobi;
pub mod nonsymm;
pub mod nonsymmv;
pub mod schur;
pub mod sort;
pub mod symm;
pub mod symmv;
} // mod eigen
pub mod err {
pub mod error;
pub mod message;
pub mod stream;
pub mod strerror;
} // mod err
pub mod fft {
pub mod dft;
pub mod fft;
} // mod fft
pub mod filter {
pub mod gaussian;
pub mod impulse;
pub mod median;
pub mod rmedian;
} // mod filter
pub mod fit {
pub mod linear;
} // mod fit
pub mod gsl_histogram;
pub mod gsl_randist;
pub mod histogram {
pub mod add;
pub mod add2d;
pub mod calloc_range;
pub mod calloc_range2d;
pub mod copy;
pub mod copy2d;
pub mod file;
pub mod file2d;
pub mod get;
pub mod get2d;
pub mod init;
pub mod init2d;
pub mod maxval;
pub mod maxval2d;
pub mod oper;
pub mod oper2d;
pub mod params;
pub mod params2d;
pub mod pdf;
pub mod pdf2d;
pub mod reset;
pub mod reset2d;
pub mod stat;
pub mod stat2d;
} // mod histogram
pub mod ieee_utils {
pub mod env;
pub mod fp;
pub mod make_rep;
pub mod print;
pub mod read;
} // mod ieee_utils
pub mod integration {
pub mod chebyshev;
pub mod chebyshev2;
pub mod cquad;
pub mod exponential;
pub mod fixed;
pub mod gegenbauer;
pub mod glfixed;
pub mod hermite;
pub mod jacobi;
pub mod laguerre;
pub mod legendre;
pub mod qag;
pub mod qagp;
pub mod qags;
pub mod qawc;
pub mod qawf;
pub mod qawo;
pub mod qaws;
pub mod qcheb;
pub mod qk;
pub mod qk15;
pub mod qk21;
pub mod qk31;
pub mod qk41;
pub mod qk51;
pub mod qk61;
pub mod qmomo;
pub mod qmomof;
pub mod qng;
pub mod rational;
pub mod romberg;
pub mod workspace;
} // mod integration
pub mod interpolation {
pub mod accel;
pub mod akima;
pub mod bicubic;
pub mod bilinear;
pub mod cspline;
pub mod inline;
pub mod interp;
pub mod interp2d;
pub mod linear;
pub mod poly;
pub mod spline;
pub mod spline2d;
pub mod steffen;
} // mod interpolation
pub mod linalg {
pub mod balance;
pub mod balancemat;
pub mod bidiag;
pub mod cholesky;
pub mod cholesky_band;
pub mod choleskyc;
pub mod cod;
pub mod condest;
pub mod exponential;
pub mod hermtd;
pub mod hessenberg;
pub mod hesstri;
pub mod hh;
pub mod householder;
pub mod householdercomplex;
pub mod inline;
pub mod invtri;
pub mod invtri_complex;
pub mod ldlt;
pub mod ldlt_band;
pub mod lq;
pub mod lu;
pub mod lu_band;
pub mod luc;
pub mod mcholesky;
pub mod multiply;
pub mod pcholesky;
pub mod ptlq;
pub mod ql;
pub mod qr;
pub mod qr_band;
pub mod qr_ud;
pub mod qr_ur;
pub mod qr_uu;
pub mod qr_uz;
pub mod qrc;
pub mod qrpt;
pub mod rqr;
pub mod rqrc;
pub mod svd;
pub mod symmtd;
pub mod tridiag;
pub mod trimult;
pub mod trimult_complex;
} // mod linalg
pub mod matrix {
pub mod copy;
pub mod file;
pub mod getset;
pub mod init;
pub mod matrix;
pub mod minmax;
pub mod oper;
pub mod prop;
pub mod rowcol;
pub mod submatrix;
pub mod swap;
pub mod view;
} // mod matrix
pub mod min {
pub mod bracketing;
pub mod brent;
pub mod convergence;
pub mod fsolver;
pub mod golden;
pub mod quad_golden;
} // mod min
pub mod monte {
pub mod miser;
pub mod plain;
pub mod vegas;
} // mod monte
pub mod movstat {
pub mod alloc;
pub mod apply;
pub mod fill;
pub mod funcacc;
pub mod madacc;
pub mod medacc;
pub mod mmacc;
pub mod movQn;
pub mod movSn;
pub mod movmad;
pub mod movmean;
pub mod movmedian;
pub mod movminmax;
pub mod movqqr;
pub mod movsum;
pub mod movvariance;
pub mod mvacc;
pub mod qnacc;
pub mod qqracc;
pub mod snacc;
pub mod sumacc;
} // mod movstat
pub mod multifit {
pub mod convergence;
pub mod covar;
pub mod fdfridge;
pub mod fdfsolver;
pub mod fdjac;
pub mod fsolver;
pub mod gcv;
pub mod gradient;
pub mod lmder;
pub mod lmniel;
pub mod multilinear;
pub mod multireg;
pub mod multirobust;
pub mod multiwlinear;
pub mod robust_wfun;
pub mod work;
} // mod multifit
pub mod multifit_nlinear {
pub mod cholesky;
pub mod convergence;
pub mod covar;
pub mod dogleg;
pub mod fdf;
pub mod fdfvv;
pub mod fdjac;
pub mod lm;
pub mod mcholesky;
pub mod qr;
pub mod scaling;
pub mod subspace2D;
pub mod svd;
pub mod trust;
} // mod multifit_nlinear
pub mod multilarge {
pub mod multilarge;
pub mod normal;
pub mod tsqr;
} // mod multilarge
pub mod multilarge_nlinear {
pub mod cgst;
pub mod cholesky;
pub mod convergence;
pub mod dogleg;
pub mod dummy;
pub mod fdf;
pub mod lm;
pub mod mcholesky;
pub mod scaling;
pub mod subspace2D;
pub mod trust;
} // mod multilarge_nlinear
pub mod multimin {
pub mod conjugate_fr;
pub mod conjugate_pr;
pub mod convergence;
pub mod diff;
pub mod fdfminimizer;
pub mod fminimizer;
pub mod simplex;
pub mod simplex2;
pub mod steepest_descent;
pub mod vector_bfgs;
pub mod vector_bfgs2;
} // mod multimin
pub mod multiroots {
pub mod broyden;
pub mod convergence;
pub mod dnewton;
pub mod fdfsolver;
pub mod fdjac;
pub mod fsolver;
pub mod gnewton;
pub mod hybrid;
pub mod hybridj;
pub mod newton;
} // mod multiroots
pub mod multiset {
pub mod file;
pub mod init;
pub mod inline;
pub mod multiset;
} // mod multiset
pub mod ntuple {
pub mod ntuple;
} // mod ntuple
pub mod ode_initval {
pub mod bsimp;
pub mod control;
pub mod cscal;
pub mod cstd;
pub mod evolve;
pub mod gear1;
pub mod gear2;
pub mod rk2;
pub mod rk2imp;
pub mod rk2simp;
pub mod rk4;
pub mod rk4imp;
pub mod rk8pd;
pub mod rkck;
pub mod rkf45;
pub mod step;
} // mod ode_initval
pub mod ode_initval2 {
pub mod bsimp;
pub mod control;
pub mod cscal;
pub mod cstd;
pub mod driver;
pub mod evolve;
pub mod msadams;
pub mod msbdf;
pub mod rk1imp;
pub mod rk2;
pub mod rk2imp;
pub mod rk4;
pub mod rk4imp;
pub mod rk8pd;
pub mod rkck;
pub mod rkf45;
pub mod step;
} // mod ode_initval2
pub mod permutation {
pub mod canonical;
pub mod file;
pub mod init;
pub mod inline;
pub mod permutation;
pub mod permute;
} // mod permutation
pub mod poly {
pub mod dd;
pub mod deriv;
pub mod eval;
pub mod solve_cubic;
pub mod solve_quadratic;
pub mod zsolve;
pub mod zsolve_cubic;
pub mod zsolve_init;
pub mod zsolve_quadratic;
} // mod poly
pub mod qrng {
pub mod halton;
pub mod inline;
pub mod niederreiter_2;
pub mod qrng;
pub mod reversehalton;
pub mod sobol;
} // mod qrng
pub mod randist {
pub mod bernoulli;
pub mod beta;
pub mod bigauss;
pub mod binomial;
pub mod binomial_tpe;
pub mod cauchy;
pub mod chisq;
pub mod dirichlet;
pub mod discrete;
pub mod erlang;
pub mod exponential;
pub mod exppow;
pub mod fdist;
pub mod flat;
pub mod gamma;
pub mod gauss;
pub mod gausstail;
pub mod gausszig;
pub mod geometric;
pub mod gumbel;
pub mod hyperg;
pub mod landau;
pub mod laplace;
pub mod levy;
pub mod logarithmic;
pub mod logistic;
pub mod lognormal;
pub mod multinomial;
pub mod mvgauss;
pub mod nbinomial;
pub mod pareto;
pub mod pascal;
pub mod poisson;
pub mod rayleigh;
pub mod shuffle;
pub mod sphere;
pub mod tdist;
pub mod weibull;
pub mod wishart;
} // mod randist
pub mod rng {
pub mod borosh13;
pub mod cmrg;
pub mod coveyou;
pub mod default;
pub mod file;
pub mod fishman18;
pub mod fishman20;
pub mod fishman2x;
pub mod gfsr4;
pub mod inline;
pub mod knuthran;
pub mod knuthran2;
pub mod knuthran2002;
pub mod lecuyer21;
pub mod minstd;
pub mod mrg;
pub mod mt;
pub mod r250;
pub mod ran0;
pub mod ran1;
pub mod ran2;
pub mod ran3;
pub mod rand;
pub mod rand48;
pub mod random;
pub mod randu;
pub mod ranf;
pub mod ranlux;
pub mod ranlxd;
pub mod ranlxs;
pub mod ranmar;
pub mod rng;
pub mod slatec;
pub mod taus;
pub mod taus113;
pub mod transputer;
pub mod tt;
pub mod types;
pub mod uni;
pub mod uni32;
pub mod vax;
pub mod waterman14;
pub mod zuf;
} // mod rng
pub mod roots {
pub mod bisection;
pub mod brent;
pub mod convergence;
pub mod falsepos;
pub mod fdfsolver;
pub mod fsolver;
pub mod newton;
pub mod secant;
pub mod steffenson;
} // mod roots
pub mod rstat {
pub mod rquantile;
pub mod rstat;
} // mod rstat
pub mod siman {
pub mod siman;
pub mod siman_tsp;
} // mod siman
pub mod sort {
pub mod sort;
pub mod sortind;
pub mod sortvec;
pub mod sortvecind;
pub mod subset;
pub mod subsetind;
} // mod sort
pub mod spblas {
pub mod spdgemm;
pub mod spdgemv;
} // mod spblas
pub mod specfunc {
pub mod airy;
pub mod airy_der;
pub mod airy_zero;
pub mod atanint;
pub mod bessel;
pub mod bessel_I0;
pub mod bessel_I1;
pub mod bessel_In;
pub mod bessel_Inu;
pub mod bessel_J0;
pub mod bessel_J1;
pub mod bessel_Jn;
pub mod bessel_Jnu;
pub mod bessel_K0;
pub mod bessel_K1;
pub mod bessel_Kn;
pub mod bessel_Knu;
pub mod bessel_Y0;
pub mod bessel_Y1;
pub mod bessel_Yn;
pub mod bessel_Ynu;
pub mod bessel_amp_phase;
pub mod bessel_i;
pub mod bessel_j;
pub mod bessel_k;
pub mod bessel_olver;
pub mod bessel_sequence;
pub mod bessel_temme;
pub mod bessel_y;
pub mod bessel_zero;
pub mod beta;
pub mod beta_inc;
pub mod clausen;
pub mod coulomb;
pub mod coulomb_bound;
pub mod coupling;
pub mod dawson;
pub mod debye;
pub mod dilog;
pub mod elementary;
pub mod ellint;
pub mod elljac;
pub mod erfc;
pub mod exp;
pub mod expint;
pub mod expint3;
pub mod fermi_dirac;
pub mod gamma;
pub mod gamma_inc;
pub mod gegenbauer;
pub mod hermite;
pub mod hyperg;
pub mod hyperg_0F1;
pub mod hyperg_1F1;
pub mod hyperg_2F0;
pub mod hyperg_2F1;
pub mod hyperg_U;
pub mod inline;
pub mod laguerre;
pub mod lambert;
pub mod legendre_H3d;
pub mod legendre_P;
pub mod legendre_Qn;
pub mod legendre_con;
pub mod legendre_poly;
pub mod log;
pub mod mathieu_angfunc;
pub mod mathieu_charv;
pub mod mathieu_coeff;
pub mod mathieu_radfunc;
pub mod mathieu_workspace;
pub mod poch;
pub mod pow_int;
pub mod psi;
pub mod result;
pub mod shint;
pub mod sincos_pi;
pub mod sinint;
pub mod synchrotron;
pub mod transport;
pub mod trig;
pub mod zeta;
} // mod specfunc
pub mod splinalg {
pub mod gmres;
pub mod itersolve;
} // mod splinalg
pub mod spmatrix {
pub mod compress;
pub mod copy;
pub mod file;
pub mod getset;
pub mod init;
pub mod minmax;
pub mod oper;
pub mod prop;
pub mod swap;
pub mod util;
} // mod spmatrix
pub mod statistics {
pub mod Qn;
pub mod Sn;
pub mod absdev;
pub mod covariance;
pub mod gastwirth;
pub mod kurtosis;
pub mod lag1;
pub mod mad;
pub mod mean;
pub mod median;
pub mod minmax;
pub mod p_variance;
pub mod quantiles;
pub mod select;
pub mod skew;
pub mod trmean;
pub mod ttest;
pub mod variance;
pub mod wabsdev;
pub mod wkurtosis;
pub mod wmean;
pub mod wskew;
pub mod wvariance;
} // mod statistics
pub mod sum {
pub mod levin_u;
pub mod levin_utrunc;
pub mod work_u;
pub mod work_utrunc;
} // mod sum
pub mod sys {
pub mod coerce;
pub mod expm1;
pub mod fcmp;
pub mod fdiv;
pub mod hypot;
pub mod infnan;
pub mod invhyp;
pub mod ldfrexp;
pub mod log1p;
pub mod minmax;
pub mod pow_int;
pub mod prec;
} // mod sys
pub mod test {
pub mod results;
} // mod test
pub mod utils {
pub mod placeholder;
} // mod utils
pub mod vector {
pub mod copy;
pub mod file;
pub mod init;
pub mod minmax;
pub mod oper;
pub mod prop;
pub mod reim;
pub mod subvector;
pub mod swap;
pub mod vector;
pub mod view;
} // mod vector
pub mod version;
pub mod wavelet {
pub mod bspline;
pub mod daubechies;
pub mod dwt;
pub mod haar;
pub mod wavelet;
} // mod wavelet
