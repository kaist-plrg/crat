/* lsodes.h

   Copyright (c) 1993-2017 Free Software Foundation, Inc.

   This file is part of GNU MCSim.

   GNU MCSim is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License
   as published by the Free Software Foundation; either version 3
   of the License, or (at your option) any later version.

   GNU MCSim is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with GNU MCSim; if not, see <http://www.gnu.org/licenses/>

   lsodes.c was translated from lsodes.f by the utility f2c in bea.
   To make lsodes.c a stand alone C routine, the following modifications were
   made:
    1. the options -lF77 -lI77 were removed from the link command line
    2. a function d_sign was written and added at the beginning of
       the function body
    3. lsodes was split into two files

    This is the header file for the two parts
*/

/* ----------------------------------------------------------------------------
   Inclusions */

#include <float.h> /* should define DBL_EPSILON */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>


/* ----------------------------------------------------------------------------
   Macros */

#define mymin(a,b) ((a) <= (b) ? (a) : (b))
#define mymax(a,b) ((a) >= (b) ? (a) : (b))
#define mydmin(a,b) (double)mymin(a,b)
#define mydmax(a,b) (double)mymax(a,b)


/* ----------------------------------------------------------------------------
   Type defs */


/* ----------------------------------------------------------------------------
   Prototypes */

int lsodes_(long *, double *, double *, double *, long *,
            double *, double *, long *, long *, long *,
            double *, long *, long *, long *, long *);


/* ----------------------------------------------------------------------------
   Prototypes private to lsodes */

int adjlr_(long *n, long *isp, long *ldif);

int cdrv_(long *n, long *r, long *c, long *ic,
          long *ia, long *ja, double *a, double *b, double *z,
          long *nsp, long *isp, double *rsp, long *esp,
          long path, long *flag_);

int cfode_(long *meth, double *elco, double * tesco);

int cntnzu_(long *n, long *ia, long *ja,
            long *nzsut);

double d_sign(double *pa1, double *pa2);

int ewset_(long *, long *, double *, double *, double *, double *);

int intdy_(double *, long, double *, long *, double *, long *);

int iprep_(long *, double *, double *, long *, long *, long *);

int jgroup_(long *n, long *ia, long *ja, long *
            maxg, long *ngrp, long *igp, long *jgp,
            long *incl, long *jdone, long *ier);

int md_(long *n, long *ia, long *ja, long *max_,
        long *v, long *l, long *head, long *last,
        long *next, long *mark, long *flag_);

int mdi_(long *n, long *ia, long *ja, long *max_,
         long *v, long *l, long *head, long *last,
         long *next, long *mark, long *tag, long *flag_);

int mdm_(long *vk, long *tail, long *v, long *l,
         long *last, long *next, long *mark);

int mdp_(long *k, long *ek, long *tail, long *v,
         long *l, long *head, long *last, long *next,
         long *mark);

int mdu_(long *ek, long *dmin_, long *v, long *l,
         long *head, long *last, long *next, long *mark);

int nnfc_(long *n, long *r, long *c, long *ic,
          long *ia, long *ja, double *a, double *z, double *b,
          long *lmax, long *il, long *jl, long *ijl, double *l,
          double *d, long *umax, long *iu, long *ju, long *iju,
          double *u, double *row, double *tmp, long *irl,
          long *jrl, long *flag_);

int nnsc_(long *n, long *r, long *c, long *il,
          long *jl, long *ijl, double *l, double *d, long *iu,
          long *ju, long *iju, double *u, double *z,
          double *b, double *tmp);

int nntc_(long *n, long *r, long *c, long *il,
          long *jl, long *ijl, double *l, double *d, long *iu,
          long *ju, long *iju, double *u, double *z,
          double *b, double *tmp);

int nroc_(long *n, long *ic, long *ia, long *ja,
          double *a, long *jar, double *ar, long *p,
          long *flag_);

int nsfc_(long *n, long *r, long *ic, long *ia,
          long *ja, long *jlmax, long *il, long *jl,
          long *ijl, long *jumax, long *iu, long *ju,
          long *iju, long *q, long *ira, long *jra,
          long *irac, long *irl, long *jrl, long *iru,
          long *jru, long *flag_);

int odrv_(long *n, long *ia, long *ja, double *a,
          long *p, long *ip, long *nsp, long *isp,
          long path, long *flag_);

int prep_(long *neq, double *y, double *yh,
          double *savf, double *ewt, double *ftem, long *ia,
          long *ja, double *wk, long *iwk, long *ipper);

int prjs_(long *, double *, double *,
          long *, double *, double *, double *,
          double *, long *);

int slss_(double *wk, long *iwk, double *x, double *tem);

int stode_(long *, double *, double *, long *, double *, double *,
       double *, double *, double *, long *);

double vnorm_(long *, double *, double *);

/* end */
