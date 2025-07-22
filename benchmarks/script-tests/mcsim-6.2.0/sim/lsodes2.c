/* lsodes2.c

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

   lsodes.c was translated from lsodes.f by the utility f2c.
   To make lsodes.c a stand-alone C routine, the following modifications were 
   made:
        1. the options -lF77 -lI77 were removed from the link command line
        2. a function d_sign was written and added at the beginning of
           the function body
        3. lsodes was split into two files

   This is the second file for the two parts
*/

#include "lsodes.h"


/* ----------------------------------------------------------------------------
   nsfc_

   symbolic ldu-factorization of nonsymmetric sparse matrix
   (compressed pointer storage)

   input variables.. n, r, ic, ia, ja, jlmax, jumax.
   output variables.. il, jl, ijl, iu, ju, iju, flag.

   parameters used internally..
   nia   - q       - suppose  m*  is the result of reordering  m.  if
         -           processing of the ith row of  m*  (hence the ith
         -           row of  u) is being done,  q(j)  is initially
         -           nonzero if  m*(i,j) is nonzero (j.ge.i).  since
         -           values need not be stored, each entry points to the
         -           next nonzero and  q(n+1)  points to the first.  n+1
         -           indicates the end of the list.  for example, if n=9
         -           and the 5th row of  m*  is
         -              0 x x 0 x 0 0 x 0
         -           then  q  will initially be
         -              a a a a 8 a a 10 5           (a - arbitrary).
         -           as the algorithm proceeds, other elements of  q
         -           are inserted in the list because of fillin.
         -           q  is used in an analogous manner to compute the
         -           ith column of  l.
         -           size = n+1.
   nia   - ira,    - vectors used to find the columns of  m.  at the kth
   nia   - jra,      step of the factorization,  irac(k)  points to the
   nia   - irac      head of a linked list in  jra  of row indices i
         -           such that i .ge. k and  m(i,k)  is nonzero.  zero
         -           indicates the end of the list.  ira(i)  (i.ge.k)
         -           points to the smallest j such that j .ge. k and
         -           m(i,j)  is nonzero.
         -           size of each = n.
   nia   - irl,    - vectors used to find the rows of  l.  at the kth step
   nia   - jrl       of the factorization,  jrl(k)  points to the head
         -           of a linked list in  jrl  of column indices j
         -           such j .lt. k and  l(k,j)  is nonzero.  zero
         -           indicates the end of the list.  irl(j)  (j.lt.k)
         -           points to the smallest i such that i .ge. k and
         -           l(i,j)  is nonzero.
         -           size of each = n.
   nia   - iru,    - vectors used in a manner analogous to  irl and jrl
   nia   - jru       to find the columns of  u.
         -           size of each = n.

   internal variables..
   jlptr - points to the last position used in  jl.
   juptr - points to the last position used in  ju.
   jmin,jmax - are the indices in  a or u  of the first and last
               elements to be examined in a given row.
               for example,  jmin=ia(k), jmax=ia(k+1)-1.
*/

int nsfc_(long *n, long *r, long *ic, long *ia,
          long *ja, long *jlmax, long *il, long *jl,
          long *ijl, long *jumax, long *iu, long *ju,
          long *iju, long *q, long *ira, long *jra,
          long *irac, long *irl, long *jrl, long *iru,
          long *jru, long *flag_)
{
  /* System generated locals */
  long i__2, i__3;

  /* Local variables */
  long cend, irai, rend, jmin, jmax, long_, irll, jtmp, irul, i, j, k, m,
       jaiak, jlmin, lasti, jumin, i1, jlptr, juptr, qm, vj, jairai,
       lastid, np1, iak, luk;
  long rk = 0;

  /* Parameter adjustments */
  --jru;
  --iru;
  --jrl;
  --irl;
  --irac;
  --jra;
  --ira;
  --q;
  --iju;
  --ju;
  --iu;
  --ijl;
  --jl;
  --il;
  --ja;
  --ia;
  --ic;
  --r;

  /* Function Body */
  np1 = *n + 1;
  jlmin = 1;
  jlptr = 0;
  il[1] = 1;
  jumin = 1;
  juptr = 0;
  iu[1] = 1;

  for (k = 1; k <= *n; ++k) {
    irac[k] = 0;
    jra[k] = 0;
    jrl[k] = 0;
    jru[k] = 0;
  }

  /* initialize column pointers for a */
  for (k = 1; k <= *n; ++k) {
    rk = r[k];
    iak = ia[rk];
    if (iak >= ia[rk + 1]) goto L101;

    jaiak = ic[ja[iak]];
    if (jaiak > k) goto L105;

    jra[k] = irac[jaiak];
    irac[jaiak] = k;

    ira[k] = iak;
  }

  /* for each column of l and row of u */
  for (k = 1; k <= *n; ++k) {

    /* initialize q for computing kth column of l */
    q[np1] = np1;
    luk = -1;

    /* by filling in kth column of a */
    vj = irac[k];
    if (vj == 0) goto L5;

L3:
    qm = np1;

L4:
    m = qm;
    qm = q[m];
    if (qm < vj) goto L4;

    if (qm == vj) goto L102;

    ++luk;
    q[m] = vj;
    q[vj] = qm;
    vj = jra[vj];
    if (vj != 0) goto L3;

    /* link through jru */
L5:
    lastid = 0;
    lasti = 0;
    ijl[k] = jlptr;
    i = k;

L6:
    i = jru[i];
    if (i == 0) goto L10;

    qm = np1;
    jmin = irl[i];
    jmax = ijl[i] + il[i + 1] - il[i] - 1;
    long_ = jmax - jmin;
    if (long_ < 0) goto L6;

    jtmp = jl[jmin];
    if (jtmp != k) ++long_;

    if (jtmp == k) r[i] = -r[i];

    if (lastid < long_) {
      lasti = i;
      lastid = long_;
    }

    /* and merge the corresponding columns into the kth column */
    for (j = jmin; j <= jmax; ++j) {
      vj = jl[j];

      do {
        m = qm;
        qm = q[m];
      }
      while (qm < vj);

      if (qm != vj) {
        ++luk;
        q[m] = vj;
        q[vj] = qm;
        qm = vj;
      }
    }

    goto L6;

L10:
    /* lasti is the longest column merged into the kth.
       see if it equals the entire kth column */
    qm = q[np1];
    if (qm != k) goto L105;

    if (luk == 0) goto L17;

    if (lastid != luk) goto L11;

    /* if so, jl can be compressed */
    irll = irl[lasti];
    ijl[k] = irll + 1;
    if (jl[irll] != k) --ijl[k];

    goto L17;

L11:
    /* if not, see if kth column can overlap the previous one */
    if (jlmin > jlptr) goto L15;

    qm = q[qm];
    for (j = jlmin; j <= jlptr; ++j) {
      if ((i__3 = jl[j] - qm) >= 0) {
        if (i__3 == 0) goto L13;
        else goto L15;
      }
    }

    goto L15;

L13:
    ijl[k] = j;
    for (i = j; i <= jlptr; ++i) {
      if (jl[i] != qm) goto L15;

      qm = q[qm];
      if (qm > *n) goto L17;
    }

    jlptr = j - 1;

L15:
  /*  *  move column indices from q to jl, update vectors  **
* */
  jlmin = jlptr + 1;
  ijl[k] = jlmin;
  if (luk == 0) {
    goto L17;
  }
  jlptr += luk;
  if (jlptr > *jlmax) {
    goto L103;
  }
  qm = q[np1];
  i__2 = jlptr;
  for (j = jlmin; j <= i__2; ++j) {
    qm = q[qm];
/* L16: */
    jl[j] = qm;
  }
L17:
  irl[k] = ijl[k];
  il[k + 1] = il[k] + luk;

/*  *  initialize q for computing kth row of u  *
* */
  q[np1] = np1;
  luk = -1;
/*  *  by filling in kth row of reordered a  *
* */
  rk = r[k];
  jmin = ira[k];
  jmax = ia[rk + 1] - 1;
  if (jmin > jmax) {
    goto L20;
  }
  i__2 = jmax;
  for (j = jmin; j <= i__2; ++j) {
    vj = ic[ja[j]];
    qm = np1;
L18:
    m = qm;
    qm = q[m];
    if (qm < vj) {
    goto L18;
    }
    if (qm == vj) {
    goto L102;
    }
    ++luk;
    q[m] = vj;
    q[vj] = qm;
/* L19: */
  }
/*  *  link through jrl,  **
* */
L20:
  lastid = 0;
  lasti = 0;
  iju[k] = juptr;
  i = k;
  i1 = jrl[k];
L21:
  i = i1;
  if (i == 0) {
    goto L26;
  }
  i1 = jrl[i];
  qm = np1;
  jmin = iru[i];
  jmax = iju[i] + iu[i + 1] - iu[i] - 1;
  long_ = jmax - jmin;
  if (long_ < 0) {
    goto L21;
  }
  jtmp = ju[jmin];
  if (jtmp == k) {
    goto L22;
  }
/*  *  update irl and jrl, **
* */
  ++long_;
  cend = ijl[i] + il[i + 1] - il[i];
  ++irl[i];
  if (irl[i] >= cend) {
    goto L22;
  }
  j = jl[irl[i]];
  jrl[i] = jrl[j];
  jrl[j] = i;
L22:
  if (lastid >= long_) {
    goto L23;
  }
  lasti = i;
  lastid = long_;
/*  *  and merge the corresponding rows into the kth row  *
* */
L23:
  i__2 = jmax;
  for (j = jmin; j <= i__2; ++j) {
    vj = ju[j];
L24:
    m = qm;
    qm = q[m];
    if (qm < vj) {
    goto L24;
    }
    if (qm == vj) {
    goto L25;
    }
    ++luk;
    q[m] = vj;
    q[vj] = qm;
    qm = vj;
L25:
    ;
  }
  goto L21;
/*  *  update jrl(k) and irl(k)  *
* */
L26:
  if (il[k + 1] <= il[k]) {
    goto L27;
  }
  j = jl[irl[k]];
  jrl[k] = jrl[j];
  jrl[j] = k;
/*  *  lasti is the longest row merged into the kth  *
* */
/*  *  see if it equals the entire kth row  **
* */
L27:
  qm = q[np1];
  if (qm != k) {
    goto L105;
  }
  if (luk == 0) {
    goto L34;
  }
  if (lastid != luk) {
    goto L28;
  }
/*  *  if so, ju can be compressed  **
* */
  irul = iru[lasti];
  iju[k] = irul + 1;
  if (ju[irul] != k) {
    --iju[k];
  }
  goto L34;

L28:
  /* if not, see if kth row can overlap the previous one */
  if (jumin > juptr) goto L32;

  qm = q[qm];
  for (j = jumin; j <= juptr; ++j) {
    if ((i__3 = ju[j] - qm) < 0) goto L29;
    else
      if (i__3 == 0) goto L30;
      else goto L32;

L29: ;
  }
  goto L32;

L30:
  iju[k] = j;
  for (i = j; i <= juptr; ++i) {
    if (ju[i] != qm) goto L32;

    qm = q[qm];
    if (qm > *n) goto L34;
  }
  juptr = j - 1;

L32:
  /* move row indices from q to ju, update vectors */
  jumin = juptr + 1;
  iju[k] = jumin;
  if (luk == 0) goto L34;

  juptr += luk;
  if (juptr > *jumax) goto L106;

  qm = q[np1];
  for (j = jumin; j <= juptr; ++j) {
    qm = q[qm];
    ju[j] = qm;
  }

L34:
  iru[k] = iju[k];
  iu[k + 1] = iu[k] + luk;

  /* update iru, jru */
  i = k;

L35:
  i1 = jru[i];
  if (r[i] < 0) goto L36;

  rend = iju[i] + iu[i + 1] - iu[i];
  if (iru[i] >= rend) goto L37;

  j = ju[iru[i]];
  jru[i] = jru[j];
  jru[j] = i;
  goto L37;

L36:
  r[i] = -r[i];

L37:
  i = i1;
  if (i == 0) goto L38;

  ++iru[i];
  goto L35;

L38:
  /* update ira, jra, irac */
  i = irac[k];
  if (i == 0) goto L41;

L39:
  i1 = jra[i];
  ++ira[i];
  if (ira[i] >= ia[r[i] + 1]) goto L40;

  irai = ira[i];
  jairai = ic[ja[irai]];
  if (jairai > i) goto L40;

  jra[i] = irac[jairai];
  irac[jairai] = i;

L40:
  i = i1;
  if (i != 0) goto L39;

L41: ;
  }

  ijl[*n] = jlptr;
  iju[*n] = juptr;
  *flag_ = 0;
  return 0;

L101:
  /* error.. null row in a */
  *flag_ = *n + rk;
  return 0;

L102:
  /* error.. duplicate entry in a */
  *flag_ = (*n << 1) + rk;
  return 0;

L103:
  /* error.. insufficient storage for jl */
  *flag_ = *n * 3 + k;
  return 0;

L105:
  /* error.. null pivot */
  *flag_ = *n * 5 + k;
  return 0;

L106:
  /* error.. insufficient storage for ju */
  *flag_ = *n * 6 + k;
  return 0;
} /* nsfc_ */

int nroc_(long *n, long *ic, long *ia, long *ja,
          double *a, long *jar, double *ar, long *p,
          long *flag_)
{
    /* System generated locals */
    long i__1, i__2;

    /* Local variables */
    long jmin, jmax, newj, i, j, k;

/* -----------------------------------------------------------------------------
*/
/*            0 0 0 0 d */
/*    would be represented as */
/*                - 1 2 3 4 5 6 */
/*            ----+------------ */
/*             iu - 1 4 6 7 8 8 */
/*             ju - 3 4 5 4 */
/*            iju - 1 2 4 3           . */
/*    the diagonal entries of l and u are assumed to be equal to one and
*/
/*    are not stored.  the array d contains the reciprocals of the */
/*    diagonal entries of the matrix d. */

/*    iii. additional storage savings */
/*         in nsfc, r and ic can be the same array in the calling */
/*    sequence if no reordering of the coefficient matrix has been done.
*/
/*         in nnfc, r, c, and ic can all be the same array if no */
/*    reordering has been done.  if only the rows have been reordered, */
/*    then c and ic can be the same array.  if the row and column */
/*    orderings are the same, then r and c can be the same array.  z and
*/
/*    row can be the same array. */
/*         in nnsc or nntc, r and c can be the same array if no */
/*    reordering has been done or if the row and column orderings are the
*/
/*    same.  z and b can be the same array.  however, then b will be */
/*    destroyed. */

/*    iv.  parameters */
/*         following is a list of parameters to the programs.  names are
*/
/*    uniform among the various subroutines.  class abbreviations are --
*/
/*       n - long variable */
/*       f - real variable */
/*       v - supplies a value to a subroutine */
/*       r - returns a result from a subroutine */
/*       i - used internally by a subroutine */
/*       a - array */

/* class - parameter */
/* ------+---------- */
/* fva   - a     - nonzero entries of the coefficient matrix m, stored */
/*       -           by rows. */
/*       -           size = number of nonzero entries in m. */
/* fva   - b     - right-hand side b. */
/*       -           size = n. */
/* nva   - c     - ordering of the columns of m. */
/*       -           size = n. */
/* fvra  - d     - reciprocals of the diagonal entries of the matrix d. */

/*       -           size = n. */
/* nr    - flag  - error flag.  values and their meanings are -- */
/*       -            0     no errors detected */
/*       -            n+k   null row in a  --  row = k */
/*       -           2n+k   duplicate entry in a  --  row = k */
/*       -           3n+k   insufficient storage for jl  --  row = k */
/*       -           4n+1   insufficient storage for l */
/*       -           5n+k   null pivot  --  row = k */
/*       -           6n+k   insufficient storage for ju  --  row = k */
/*       -           7n+1   insufficient storage for u */
/*       -           8n+k   zero pivot  --  row = k */
/* nva   - ia    - pointers to delimit the rows of a. */
/*       -           size = n+1. */
/* nvra  - ijl   - pointers to the first element in each column in jl, */
/*       -           used to compress storage in jl. */
/*       -           size = n. */
/* nvra  - iju   - pointers to the first element in each row in ju, used
*/
/*       -           to compress storage in ju. */
/*       -           size = n. */
/* nvra  - il    - pointers to delimit the columns of l. */
/*       -           size = n+1. */
/* nvra  - iu    - pointers to delimit the rows of u. */
/*       -           size = n+1. */
/* nva   - ja    - column numbers corresponding to the elements of a. */
/*       -           size = size of a. */
/* nvra  - jl    - row numbers corresponding to the elements of l. */
/*       -           size = jlmax. */
/* nv    - jlmax - declared dimension of jl.  jlmax must be larger than */

/*       -           the number of nonzeros in the strict lower triangle
*/
/*       -           of m plus fillin minus compression. */
/* nvra  - ju    - column numbers corresponding to the elements of u. */
/*       -           size = jumax. */
/* nv    - jumax - declared dimension of ju.  jumax must be larger than */

/*       -           the number of nonzeros in the strict upper triangle
*/
/*       -           of m plus fillin minus compression. */
/* fvra  - l     - nonzero entries in the strict lower triangular portion
*/
/*       -           of the matrix l, stored by columns. */
/*       -           size = lmax. */
/* nv    - lmax  - declared dimension of l.  lmax must be larger than */
/*       -           the number of nonzeros in the strict lower triangle
*/
/*       -           of m plus fillin  (il(n+1)-1 after nsfc). */
/* nv    - n     - number of variables/equations. */
/* nva   - r     - ordering of the rows of m. */
/*       -           size = n. */
/* fvra  - u     - nonzero entries in the strict upper triangular portion
*/
/*       -           of the matrix u, stored by rows. */
/*       -           size = umax. */
/* nv    - umax  - declared dimension of u.  umax must be larger than */
/*       -           the number of nonzeros in the strict upper triangle
*/
/*       -           of m plus fillin  (iu(n+1)-1 after nsfc). */
/* fra   - z     - solution x. */
/*       -           size = n. */

/*       ----------------------------------------------------------------
*/

/* subroutine nroc */
/* reorders rows of a, leaving row order unchanged */


/*       input parameters.. n, ic, ia, ja, a */
/*       output parameters.. ja, a, flag */

/*       parameters used internally.. */
/* nia   - p     - at the kth step, p is a linked list of the reordered */

/*       -           column indices of the kth row of a.  p(n+1) points */

/*       -           to the first entry in the list. */
/*       -           size = n+1. */
/* nia   - jar   - at the kth step,jar contains the elements of the */
/*       -           reordered column indices of a. */
/*       -           size = n. */
/* fia   - ar    - at the kth step, ar contains the elements of the */
/*       -           reordered row of a. */
/*       -           size = n. */


/*  *  for each nonempty row  * */
    /* Parameter adjustments */
    --p;
    --ar;
    --jar;
    --a;
    --ja;
    --ia;
    --ic;

    /* Function Body */
    i__1 = *n;
    for (k = 1; k <= i__1; ++k) {
    jmin = ia[k];
    jmax = ia[k + 1] - 1;
    if (jmin > jmax) {
        goto L5;
    }
    p[*n + 1] = *n + 1;
/*  *  insert each element in the list  * */
    i__2 = jmax;
    for (j = jmin; j <= i__2; ++j) {
        newj = ic[ja[j]];
        i = *n + 1;
L1:
        if (p[i] >= newj) {
        goto L2;
        }
        i = p[i];
        goto L1;
L2:
        if (p[i] == newj) {
        goto L102;
        }
        p[newj] = p[i];
        p[i] = newj;
        jar[newj] = ja[j];
        ar[newj] = a[j];
/* L3: */
    }
/*  *  replace old row in ja and a  ** */
    i = *n + 1;
    i__2 = jmax;
    for (j = jmin; j <= i__2; ++j) {
        i = p[i];
        ja[j] = jar[i];
/* L4: */
        a[j] = ar[i];
    }
L5:
    ;
    }
    *flag_ = 0;
    return 0;

/* error.. duplicate entry in a */
L102:
    *flag_ = *n + k;
    return 0;
} /* nroc_ */

/* -----------------------------------------------------------------------------
   odrv_                                                    5/2/83
*/
/*           the permutation returned in p.  dimension = n */

/*    nsp  - declared dimension of the one-dimensional array isp.  nsp */
/*           must be at least  3n+4k,  where k is the number of nonzeroes
*/
/*           in the strict upper triangle of m */

/*    isp  - long one-dimensional array used for working storage. */
/*           dimension = nsp */

/*    path - long path specification.  values and their meanings are -
*/
/*             1  find minimum degree ordering only */
/*             2  find minimum degree ordering and reorder symmetrically
*/
/*                  stored matrix (used when only the nonzero entries in
*/
/*                  the upper triangle of m are being stored) */
/*             3  reorder symmetrically stored matrix as specified by */
/*                  input permutation (used when an ordering has already
*/
/*                  been determined and only the nonzero entries in the */

/*                  upper triangle of m are being stored) */
/*             4  same as 2 but put diagonal entries at start of each row
*/
/*             5  same as 3 but put diagonal entries at start of each row
*/

/*    flag - long error flag.  values and their meanings are - */
/*               0    no errors detected */
/*              9n+k  insufficient storage in md */
/*             10n+1  insufficient storage in odrv */
/*             11n+1  illegal path specification */


/*  conversion from real to double precision */

/*    change the real declarations in odrv and sro to double precision */
/*    declarations. */

/* -------------------------------------------------------------------
 */
int odrv_(long *n, long *ia, long *ja, double *a,
          long *p, long *ip, long *nsp, long *isp,
          long path, long *flag_)
{
    long head, l;
    long dflag;
    long q, v;
    extern /* Subroutine */ int md_(long *, long *, long *, long *
        , long *, long *, long *, long *, long *, long *
        , long *);
    long max_, tmp;
    extern /* Subroutine */ int sro_(long *, long *, long *, long
        *, double *, long *, long *, long *);


/* ----initialize error flag and validate path specification */
    /* Parameter adjustments */
    --isp;
    --ip;
    --p;
    --a;
    --ja;
    --ia;

    /* Function Body */
    *flag_ = 0;
    if (path < 1 || 5 < path) {
    goto L111;
    }

/* ----allocate storage and find minimum degree ordering */
    if ((path - 1) * (path - 2) * (path - 4) != 0) {
    goto L1;
    }
    max_ = (*nsp - *n) / 2;
    v = 1;
    l = v + max_;
    head = l + max_;
    if (max_ < *n) goto L110;

    md_(n, &ia[1], &ja[1], &max_, &isp[v], &isp[l], &isp[head], &p[1], &ip[1],
         &isp[v], flag_);
    if (*flag_ != 0) goto L100;

/* ----allocate storage and symmetrically reorder matrix */
L1:
    if ((path  - 2) * (path  - 3) * (path  - 4) * (path  - 5) != 0) {
    goto L2;
    }
    tmp = *nsp + 1 - *n;
    q = tmp - (ia[*n + 1] - 1);
    if (q < 1) {
    goto L110;
    }

    dflag = path  == 4 || path  == 5;
    sro_(n, &ip[1], &ia[1], &ja[1], &a[1], &isp[tmp], &isp[q], &dflag);

L2:
    return 0;

/* error -- error detected in md */
L100:
    return 0;
/* error -- insufficient storage */
L110:
    *flag_ = *n * 10 + 1;
    return 0;
/* error -- illegal path specified */
L111:
    *flag_ = *n * 11 + 1;
    return 0;
} /* odrv_ */

int sro_(long *n, long *ip, long *ia, long *ja,
    double *a, long *q, long *r, long *dflag)
{
    /* System generated locals */
    long i__1, i__2;

    /* Local variables */
    long jmin, jmax, i, j, k, ilast;
    double ak;
    long jdummy, jak;

/*
 */
/*  sro -- symmetric reordering of sparse symmetric matrix */
/*
 */

/*  description */

/*    the nonzero entries of the matrix m are assumed to be stored */
/*    symmetrically in (ia,ja,a) format (i.e., not both m(i,j) and m(j,i)
*/
/*    are stored if i ne j). */

/*    sro does not rearrange the order of the rows, but does move */
/*    nonzeroes from one row to another to ensure that if m(i,j) will be
*/
/*    in the upper triangle of m with respect to the new ordering, then */

/*    m(i,j) is stored in row i (and thus m(j,i) is not stored),  whereas
*/
/*    if m(i,j) will be in the strict lower triangle of m, then m(j,i) is
*/
/*    stored in row j (and thus m(i,j) is not stored). */


/*  additional parameters */

/*    q     - long one-dimensional work array.  dimension = n */

/*    r     - long one-dimensional work array.  dimension = number of
*/
/*            nonzero entries in the upper triangle of m */

/*    dflag - logical variable.  if dflag = .true., then store nonzero */
/*            diagonal elements at the beginning of the row */

/* -------------------------------------------------------------------
 */


/* --phase 1 -- find row in which to store each nonzero */
/* ----initialize count of nonzeroes to be stored in each row */
    /* Parameter adjustments */
    --r;
    --q;
    --a;
    --ja;
    --ia;
    --ip;

    /* Function Body */
    i__1 = *n;
    for (i = 1; i <= i__1; ++i) {
/* L1: */
    q[i] = 0;
    }

/* ----for each nonzero element a(j) */
    i__1 = *n;
    for (i = 1; i <= i__1; ++i) {
    jmin = ia[i];
    jmax = ia[i + 1] - 1;
    if (jmin > jmax) {
        goto L3;
    }
    i__2 = jmax;
    for (j = jmin; j <= i__2; ++j) {

/* --------find row (=r(j)) and column (=ja(j)) in which to store
a(j) ... */
        k = ja[j];
        if (ip[k] < ip[i]) {
        ja[j] = i;
        }
        if (ip[k] >= ip[i]) {
        k = i;
        }
        r[j] = k;

/* --------... and increment count of nonzeroes (=q(r(j)) in that
row */
/* L2: */
        ++q[k];
    }
L3:
    ;
    }


/* --phase 2 -- find new ia and permutation to apply to (ja,a) */
/* ----determine pointers to delimit rows in permuted (ja,a) */
    i__1 = *n;
    for (i = 1; i <= i__1; ++i) {
    ia[i + 1] = ia[i] + q[i];
/* L4: */
    q[i] = ia[i + 1];
    }

/* ----determine where each (ja(j),a(j)) is stored in permuted (ja,a) */
/* ----for each nonzero element (in reverse order) */
    ilast = 0;
    jmin = ia[1];
    jmax = ia[*n + 1] - 1;
    j = jmax;
    i__1 = jmax;
    for (jdummy = jmin; jdummy <= i__1; ++jdummy) {
    i = r[j];
    if (! (*dflag) || ja[j] != i || i == ilast) {
        goto L5;
    }

/* ------if dflag, then put diagonal nonzero at beginning of row */
    r[j] = ia[i];
    ilast = i;
    goto L6;

/* ------put (off-diagonal) nonzero in last unused location in row */
L5:
    --q[i];
    r[j] = q[i];

L6:
    --j;
    }


/* --phase 3 -- permute (ja,a) to upper triangular form (wrt new ordering)
 */
    i__1 = jmax;
    for (j = jmin; j <= i__1; ++j) {
L7:
    if (r[j] == j) {
        goto L8;
    }
    k = r[j];
    r[j] = r[k];
    r[k] = k;
    jak = ja[k];
    ja[k] = ja[j];
    ja[j] = jak;
    ak = a[k];
    a[k] = a[j];
    a[j] = ak;
    goto L7;
L8:
    ;
    }

    return 0;
} /* sro_ */

int md_(long *n, long *ia, long *ja, long *max_,
        long *v, long *l, long *head, long *last,
        long *next, long *mark, long *flag_)
{
    /* System generated locals */
    long i__1;
    static long equiv_0[1];

    /* Local variables */
    long dmin_, tail, k;
#define ek (equiv_0)
#define vk (equiv_0)
    long tag;

/*
 */
/*  md -- minimum degree algorithm (based on element model) */
/*
 */

/*  description */

/*    md finds a minimum degree ordering of the rows and columns of a */
/*    general sparse matrix m stored in (ia,ja,a) format. */
/*    when the structure of m is nonsymmetric, the ordering is that */
/*    obtained for the symmetric matrix  m + m-transpose. */


/*  additional parameters */

/*    max  - declared dimension of the one-dimensional arrays v and l. */
/*           max must be at least  n+2k,  where k is the number of */
/*           nonzeroes in the strict upper triangle of m + m-transpose */

/*    v    - long one-dimensional work array.  dimension = max */

/*    l    - long one-dimensional work array.  dimension = max */

/*    head - long one-dimensional work array.  dimension = n */

/*    last - long one-dimensional array used to return the permutation
*/
/*           of the rows and columns of m corresponding to the minimum */
/*           degree ordering.  dimension = n */

/*    next - long one-dimensional array used to return the inverse of
*/
/*           the permutation returned in last.  dimension = n */

/*    mark - long one-dimensional work array (may be the same as v). */

/*           dimension = n */

/*    flag - long error flag.  values and their meanings are - */
/*             0     no errors detected */
/*             9n+k  insufficient storage in md */


/*  definitions of internal parameters */

/*    ---------+---------------------------------------------------------
*/
/*    v(s)     - value field of list entry */
/*    ---------+---------------------------------------------------------
*/
/*    l(s)     - link field of list entry  (0 =) end of list) */
/*    ---------+---------------------------------------------------------
*/
/*    l(vi)    - pointer to element list of uneliminated vertex vi */
/*    ---------+---------------------------------------------------------
*/
/*    l(ej)    - pointer to boundary list of active element ej */
/*    ---------+---------------------------------------------------------
*/
/*    head(d)  - vj =) vj head of d-list d */
/*             -  0 =) no vertex in d-list d */


/*             -                  vi uneliminated vertex */
/*             -          vi in ek           -       vi not in ek */
/*    ---------+-----------------------------+---------------------------
*/
/*    next(vi) - undefined but nonnegative   - vj =) vj next in d-list */
/*             -                             -  0 =) vi tail of d-list */
/*    ---------+-----------------------------+---------------------------
*/
/*    last(vi) - (not set until mdp)         - -d =) vi head of d-list d
*/
/*             --vk =) compute degree        - vj =) vj last in d-list */
/*             - ej =) vi prototype of ej    -  0 =) vi not in any d-list
*/
/*             -  0 =) do not compute degree - */
/*    ---------+-----------------------------+---------------------------
*/
/*    mark(vi) - mark(vk)                    - nonneg. tag .lt. mark(vk)
*/


/*             -                   vi eliminated vertex */
/*             -      ei active element      -           otherwise */
/*    ---------+-----------------------------+---------------------------
*/
/*    next(vi) - -j =) vi was j-th vertex    - -j =) vi was j-th vertex */

/*             -       to be eliminated      -       to be eliminated */
/*    ---------+-----------------------------+---------------------------
*/
/*    last(vi) -  m =) size of ei = m        - undefined */
/*    ---------+-----------------------------+---------------------------
*/
/*    mark(vi) - -m =) overlap count of ei   - undefined */
/*             -       with ek = m           - */
/*             - otherwise nonnegative tag   - */
/*             -       .lt. mark(vk)         - */

/* -------------------------------------------------------------------
 */


/* ----initialization */
    /* Parameter adjustments */
    --mark;
    --next;
    --last;
    --head;
    --l;
    --v;
    --ja;
    --ia;

    /* Function Body */
    tag = 0;
    mdi_(n, &ia[1], &ja[1], max_, &v[1], &l[1], &head[1], &last[1], &next[1],
        &mark[1], &tag, flag_);
    if (*flag_ != 0) {
    return 0;
    }

    k = 0;
    dmin_ = 1;

/* while  k .lt. n  do */
L1:
    if (k >= *n) {
    goto L4;
    }

/* search for vertex of minimum degree */
L2:
    if (head[dmin_] > 0) {
    goto L3;
    }
    ++dmin_;
    goto L2;

/* remove vertex vk of minimum degree from degree list */
L3:
    *vk = head[dmin_];
    head[dmin_] = next[*vk];
    if (head[dmin_] > 0) {
    last[head[dmin_]] = -dmin_;
    }

/* ------number vertex vk, adjust tag, and tag vk */
    ++k;
    next[*vk] = -k;
    last[*ek] = dmin_ - 1;
    tag += last[*ek];
    mark[*vk] = tag;

/* ------form element ek from uneliminated neighbors of vk */
    mdm_(vk, &tail, &v[1], &l[1], &last[1], &next[1], &mark[1]);

/* ------purge inactive elements and do mass elimination */
    mdp_(&k, ek, &tail, &v[1], &l[1], &head[1], &last[1], &next[1], &mark[1]);


/* ------update degrees of uneliminated vertices in ek */
    mdu_(ek, &dmin_, &v[1], &l[1], &head[1], &last[1], &next[1], &mark[1]);

    goto L1;

/* ----generate inverse permutation from permutation */
L4:
    i__1 = *n;
    for (k = 1; k <= i__1; ++k) {
    next[k] = -next[k];
/* L5: */
    last[next[k]] = k;
    }

    return 0;
} /* md_ */

#undef vk
#undef ek


int mdu_(long *ek, long *dmin_, long *v, long *l,
         long *head, long *last, long *next, long *mark)
{
    /* System generated locals */
    long i__1, i__2;
    static long equiv_0[1];

    /* Local variables */
    long b, i, s;
#define es (equiv_0)
    long vb, vi;
#define vs (equiv_0)
    long blpmax, ilpmax, tag, blp, dvi, evi, ilp;

/* lll. optimize */
/*
 */
/*  mdu -- update degrees of uneliminated vertices in ek */
/*
 */

/* ----initialize tag */
    /* Parameter adjustments */
    --mark;
    --next;
    --last;
    --head;
    --l;
    --v;

    /* Function Body */
    tag = mark[*ek] - last[*ek];

/* ----for each vertex vi in ek */
    i = *ek;
    ilpmax = last[*ek];
    if (ilpmax <= 0) {
    goto L11;
    }
    i__1 = ilpmax;
    for (ilp = 1; ilp <= i__1; ++ilp) {
    i = l[i];
    vi = v[i];
    if ((i__2 = last[vi]) < 0) {
        goto L1;
    } else if (i__2 == 0) {
        goto L10;
    } else goto L8;

/* if vi neither prototype nor duplicate vertex, then merge elements */
/* to compute degree */
L1:
    ++tag;
    dvi = last[*ek];

/* for each vertex/element vs/es in element list of vi */
    s = l[vi];
L2:
    s = l[s];
    if (s == 0) {
        goto L9;
    }
    *vs = v[s];
    if (next[*vs] < 0) {
        goto L3;
    }

/* if vs is uneliminated vertex, then tag and adjust degree */
    mark[*vs] = tag;
    ++dvi;
    goto L5;

/* ----------if es is active element, then expand */
/* ------------check for outmatched vertex */
L3:
    if (mark[*es] < 0) {
        goto L6;
    }

/* ------------for each vertex vb in es */
    b = *es;
    blpmax = last[*es];
    i__2 = blpmax;
    for (blp = 1; blp <= i__2; ++blp) {
        b = l[b];
        vb = v[b];

/* --------------if vb is untagged, then tag and adjust degree */
        if (mark[vb] >= tag) {
        goto L4;
        }
        mark[vb] = tag;
        ++dvi;
L4:
        ;
    }

L5:
    goto L2;

/* ------else if vi is outmatched vertex, then adjust overlaps but do
not */
/* ------compute degree */
L6:
    last[vi] = 0;
    --mark[*es];
L7:
    s = l[s];
    if (s == 0) {
        goto L10;
    }
    *es = v[s];
    if (mark[*es] < 0) {
        --mark[*es];
    }
    goto L7;

/* ------else if vi is prototype vertex, then calculate degree by */
/* ------inclusion/exclusion and reset overlap count */
L8:
    evi = last[vi];
    dvi = last[*ek] + last[evi] + mark[evi];
    mark[evi] = 0;

/* ------insert vi in appropriate degree list */
L9:
    next[vi] = head[dvi];
    head[dvi] = vi;
    last[vi] = -dvi;
    if (next[vi] > 0) {
        last[next[vi]] = vi;
    }
    if (dvi < *dmin_) {
        *dmin_ = dvi;
    }

L10:
    ;
    }

L11:
    return 0;
} /* mdu_ */

#undef vs
#undef es


int mdp_(long *k, long *ek, long *tail, long *v,
         long *l, long *head, long *last, long *next,
         long *mark)
{
    /* System generated locals */
    long i__1;

    /* Local variables */
    long i, s, li, es, vi, ls, ilpmax, tag, evi, ilp, lvi;
    long free = 0;

/*  mdp -- purge inactive elements and do mass elimination */

/* ----initialize tag */
    /* Parameter adjustments */
    --mark;
    --next;
    --last;
    --head;
    --l;
    --v;

    /* Function Body */
    tag = mark[*ek];

/* ----for each vertex vi in ek */
    li = *ek;
    ilpmax = last[*ek];
    if (ilpmax <= 0) {
    goto L12;
    }
    i__1 = ilpmax;
    for (ilp = 1; ilp <= i__1; ++ilp) {
    i = li;
    li = l[i];
    vi = v[li];

/* ------remove vi from degree list */
    if (last[vi] == 0) {
        goto L3;
    }
    if (last[vi] > 0) {
        goto L1;
    }
    head[-last[vi]] = next[vi];
    goto L2;
L1:
    next[last[vi]] = next[vi];
L2:
    if (next[vi] > 0) {
        last[next[vi]] = last[vi];
    }

/* ------remove inactive items from element list of vi */
L3:
    ls = vi;
L4:
    s = ls;
    ls = l[s];
    if (ls == 0) {
        goto L6;
    }
    es = v[ls];
    if (mark[es] < tag) {
        goto L5;
    }
    free = ls;
    l[s] = l[ls];
    ls = s;
L5:
    goto L4;

/* ------if vi is interior vertex, then remove from list and eliminate
 */
L6:
    lvi = l[vi];
    if (lvi != 0) {
        goto L7;
    }
    l[i] = l[li];
    li = i;

    ++(*k);
    next[vi] = -(*k);
    --last[*ek];
    goto L11;

/* ------else ... */
/* --------classify vertex vi */
L7:
    if (l[lvi] != 0) {
        goto L9;
    }
    evi = v[lvi];
    if (next[evi] >= 0) {
        goto L9;
    }
    if (mark[evi] < 0) {
        goto L8;
    }

/* ----------if vi is prototype vertex, then mark as such, initialize
*/
/* ----------overlap count for corresponding element, and move vi to e
nd */
/* ----------of boundary list */
    last[vi] = evi;
    mark[evi] = -1;
    l[*tail] = li;
    *tail = li;
    l[i] = l[li];
    li = i;
    goto L10;

/* ----------else if vi is duplicate vertex, then mark as such and adj
ust */
/* ----------overlap count for corresponding element */
L8:
    last[vi] = 0;
    --mark[evi];
    goto L10;

/* ----------else mark vi to compute degree */
L9:
    last[vi] = -(*ek);

/* --------insert ek in element list of vi */
L10:
    v[free] = *ek;
    l[free] = l[vi];
    l[vi] = free;
L11:
    ;
    }

/* ----terminate boundary list */
L12:
    l[*tail] = 0;

    return 0;
} /* mdp_ */

int mdm_(long *vk, long *tail, long *v, long *l,
         long *last, long *next, long *mark)
{
    /* System generated locals */
    long i__1;
    static long equiv_0[1];

    /* Local variables */
    long b, s, lb;
#define es (equiv_0)
    long vb, ls;
#define vs (equiv_0)
    long blpmax, tag, blp;

/* lll. optimize */
/*
 */
/*  mdm -- form element from uneliminated neighbors of vk */
/*
 */

/* ----initialize tag and list of uneliminated neighbors */
    /* Parameter adjustments */
    --mark;
    --next;
    --last;
    --l;
    --v;

    /* Function Body */
    tag = mark[*vk];
    *tail = *vk;

/* ----for each vertex/element vs/es in element list of vk */
    ls = l[*vk];
L1:
    s = ls;
    if (s == 0) {
    goto L5;
    }
    ls = l[s];
    *vs = v[s];
    if (next[*vs] < 0) {
    goto L2;
    }

/* ------if vs is uneliminated vertex, then tag and append to list of */
/* ------uneliminated neighbors */
    mark[*vs] = tag;
    l[*tail] = s;
    *tail = s;
    goto L4;

/* ------if es is active element, then ... */
/* --------for each vertex vb in boundary list of element es */
L2:
    lb = l[*es];
    blpmax = last[*es];
    i__1 = blpmax;
    for (blp = 1; blp <= i__1; ++blp) {
    b = lb;
    lb = l[b];
    vb = v[b];

/* ----------if vb is untagged vertex, then tag and append to list of
*/
/* ----------uneliminated neighbors */
    if (mark[vb] >= tag) {
        goto L3;
    }
    mark[vb] = tag;
    l[*tail] = b;
    *tail = b;
L3:
    ;
    }

/* --------mark es inactive */
    mark[*es] = tag;

L4:
    goto L1;

/* ----terminate list of uneliminated neighbors */
L5:
    l[*tail] = 0;

    return 0;
} /* mdm_ */

#undef vs
#undef es


int mdi_(long *n, long *ia, long *ja, long *max_,
         long *v, long *l, long *head, long *last,
         long *next, long *mark, long *tag, long *flag_)
{
    /* System generated locals */
    long i__1, i__2, i__3;

    /* Local variables */
    long jmin, jmax, kmax, j, k, vi, vj, nextvi, dvi, sfs, lvk;

/* lll. optimize */
/*
 */
/*  mdi -- initialization */
/*
 */

/* initialize degrees, element lists, and degree lists */
    /* Parameter adjustments */
    --mark;
    --next;
    --last;
    --head;
    --l;
    --v;
    --ja;
    --ia;

    /* Function Body */
    i__1 = *n;
    for (vi = 1; vi <= i__1; ++vi) {
    mark[vi] = 1;
    l[vi] = 0;
/* L1: */
    head[vi] = 0;
    }
    sfs = *n + 1;

/* create nonzero structure */
/* for each nonzero entry a(vi,vj) */
    i__1 = *n;
    for (vi = 1; vi <= i__1; ++vi) {
    jmin = ia[vi];
    jmax = ia[vi + 1] - 1;
    if (jmin > jmax) {
        goto L6;
    }
    i__2 = jmax;
    for (j = jmin; j <= i__2; ++j) {
        vj = ja[j];
        if ((i__3 = vj - vi) < 0) {
        goto L2;
        } else if (i__3 == 0) {
        goto L5;
        } else {
        goto L4;
        }

/* ------if a(vi,vj) is in strict lower triangle */
/* ------check for previous occurrence of a(vj,vi) */
L2:
        lvk = vi;
        kmax = mark[vi] - 1;
        if (kmax == 0) {
        goto L4;
        }
        i__3 = kmax;
        for (k = 1; k <= i__3; ++k) {
        lvk = l[lvk];
        if (v[lvk] == vj) {
            goto L5;
        }

        }
/* ----for unentered entries a(vi,vj) */
L4:
        if (sfs >= *max_) {
        goto L101;
        }

/* ------enter vj in element list for vi */
        ++mark[vi];
        v[sfs] = vj;
        l[sfs] = l[vi];
        l[vi] = sfs;
        ++sfs;

/* ------enter vi in element list for vj */
        ++mark[vj];
        v[sfs] = vi;
        l[sfs] = l[vj];
        l[vj] = sfs;
        ++sfs;
L5:
        ;
    }
L6:
    ;
    }

/* create degree lists and initialize mark vector */
    i__1 = *n;
    for (vi = 1; vi <= i__1; ++vi) {
    dvi = mark[vi];
    next[vi] = head[dvi];
    head[dvi] = vi;
    last[vi] = -dvi;
    nextvi = next[vi];
    if (nextvi > 0) {
        last[nextvi] = vi;
    }
/* L7: */
    mark[vi] = *tag;
    }

    return 0;

/* error-  insufficient storage */
L101:
    *flag_ = *n * 9 + vi;
    return 0;
} /* mdi_ */


/* -----------------------------------------------------------------------------
   jgroup_

   this subroutine constructs groupings of the column indices of
   the jacobian matrix, used in the numerical evaluation of the
   jacobian by finite differences.

   input..
   n      = the order of the matrix.
   ia,ja  = sparse structure descriptors of the matrix by rows.
   maxg   = length of available storate in the igp array.

   output..
   ngrp   = number of groups.
   jgp    = array of length n containing the column indices by groups.
   igp    = pointer array of length ngrp + 1 to the locations in jgp
            of the beginning of each group.
   ier    = error indicator.  ier = 0 if no error occurred, or 1 if
            maxg was insufficient.

   incl and jdone are working arrays of length n.
*/

int jgroup_(long *n, long *ia, long *ja, long *
            maxg, long *ngrp, long *igp, long *jgp,
            long *incl, long *jdone, long *ier)
{
  /* Local variables */
  long ncol, kmin, kmax, i, j, k, ng;

  /* Parameter adjustments */
  --jdone;
  --incl;
  --jgp;
  --igp;
  --ja;
  --ia;

  /* Function Body */
  *ier = 0;
  for (j = 1; j <= *n; ++j) jdone[j] = 0;

  ncol = 1;
  for (ng = 1; ng <= *maxg; ++ng) {
    igp[ng] = ncol;
    for (i = 1; i <= *n; ++i) incl[i] = 0;

    for (j = 1; j <= *n; ++j) {
      /* reject column j if it is already in a group */
      if (jdone[j] == 1) goto L50;

      kmin = ia[j];
      kmax = ia[j + 1] - 1;
      for (k = kmin; k <= kmax; ++k) {
        /* reject column j if it overlaps any column already in this group */
        i = ja[k];
        if (incl[i] == 1) goto L50;
      }

      /* accept column j into group ng */
      jgp[ncol] = j;
      ++ncol;
      jdone[j] = 1;
      for (k = kmin; k <= kmax; ++k) {
        i = ja[k];
        incl[i] = 1;
      }

L50: ;
    }
    /* stop if this group is empty (grouping is complete) */
    if (ncol == igp[ng]) goto L70;
  }

  /* error return if not all columns were chosen (maxg too small) */
  if (ncol <= *n) goto L80;

  ng = *maxg;

L70:
  *ngrp = ng - 1;
  return 0;

L80:
  *ier = 1;
  return 0;

} /* jgroup_ */


/* -----------------------------------------------------------------------------
   ewset_

   this subroutine sets the error weight vector ewt according to
   ewt(i) = rtol(i)*abs(ycur(i)) + atol(i),  i = 1,...,n,
   with the subscript on rtol and/or atol possibly replaced by 1 above,
   depending on the value of itol.
*/

int ewset_(long *n, long *itol, double *rtol,
           double *atol, double *ycur, double *ewt)
{
  long i;

  /* Function Body */
  switch (*itol) {
    case 1:
      for (i = 0; i < *n; ++i)
        ewt[i] = rtol[0] * fabs(ycur[i]) + atol[0];
      break;

    case 2:
      for (i = 0; i < *n; ++i)
        ewt[i] = rtol[0] * fabs(ycur[i]) + atol[i];
      break;

    case 3:
      for (i = 0; i < *n; ++i)
        ewt[i] = rtol[i] * fabs(ycur[i]) + atol[0];
      break;

    case 4:
      for (i = 0; i < *n; ++i)
        ewt[i] = rtol[i] * fabs(ycur[i]) + atol[i];
      break;
  }

  return 0;

} /* ewset_ */

/* end */
