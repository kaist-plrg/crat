/* matutil.c

   Written by Don Robert Maszle
   18 September 1992

   Copyright (c) 1992-2017 Free Software Foundation, Inc.

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

   A bunch of matrix and vector utililties.
*/


#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#include "sim.h"

/* -----------------------------------------------------------------------------
   LogTransformArray

   log-transforms the elements of a Src array to a Destination array,
   which is returned.

   To transform in place, give the same pointer for both args.
*/

double *LogTransformArray (long nElems, double *rgdSrc, double *rgdDes)
{
  register long l;

  for (l = 0; l < nElems; l++)
    rgdDes[l] = log(rgdSrc[l]);

  return rgdDes;

} /* LogTransformArray */


/* -----------------------------------------------------------------------------
   InitdVector

   initializes a vector of double.

   The pointer to the vector is returned if no error occurs, otherwise NULL
   is returned. It is an error to call it with cVectors = 0.
*/

double *InitdVector (long cVectors)
{

  if (cVectors == 0) {
    printf ("Error: zero length array allocation in InitdVector - Exiting\n");
    exit (0);
  }
  else
    return (double *) malloc (cVectors * sizeof(double));

} /* InitdVector */


/* -----------------------------------------------------------------------------
   InitiVector

   initializes a vector of integers.

   The pointer to the vector is returned if no error occurs, otherwise NULL
   is returned.  It is an error to call it with cVectors = 0.
*/

int *InitiVector (long cVectors)
{

  if (cVectors == 0) {
    printf ("Error: zero length array allocation in InitiVector - Exiting\n");
    exit (0);
  }
  else        
   return (int *) malloc (cVectors * sizeof(int));

} /* InitiVector */


/* ----------------------------------------------------------------------------
   InitlVector

   initializes a vector of longs.

   The pointer to the vector is returned if no error occurs, otherwise NULL
   is returned. It is an error to call it with cVectors = 0.
*/

long *InitlVector (long cVectors)
{

  if (cVectors == 0) {
    printf ("Error: zero length array allocation in InitlVector - Exiting\n");
    exit (0);
  }
  else        
    return (long *) malloc (cVectors * sizeof(long));

} /* InitlVector */


/* ----------------------------------------------------------------------------
   InitpdVector

   initializes a vector of pointers to doubles.

   The pointer to the vector is returned if no error occurs, otherwise NULL
   is returned. It is an error to call it with cVectors = 0.
*/

double **InitpdVector (long cVectors)
{

  if (cVectors == 0) {
    printf ("Error: zero length array allocation in InitpdVector - Exiting\n");
    exit (0);
  }
  else
    return (double **) malloc (cVectors * sizeof(double *));

} /* InitpdVector */


/* ----------------------------------------------------------------------------
   InitdMatrix

   initializes a 2 dimensional matrix of doubles.

   The pointer to the vector of vectors is returned if no error occurs,
   otherwise the NULL value is returned. It is an error to call it with
   cVectors = 0 or cElemsEach = 0.
*/

double **InitdMatrix (long cVectors, long cElemsEach)
{
  register long i;
  double **rgp;

  if ((cVectors == 0) || (cElemsEach == 0)) {
    printf ("Error: zero length array allocation in InitdMatrix - Exiting\n");
    exit (0);
  }
      
  rgp = (double **) malloc (cVectors * sizeof(double *));

  if (rgp) {
    for (i = 0; i < cVectors; i++) {
      rgp[i] = (double *) malloc (cElemsEach * sizeof(double));
      if (!rgp[i]) {
        rgp = 0;
        break;
      } /* if */
    } /* for */
  } /* if */

  return (rgp);

} /* InitdMatrix */


/* ----------------------------------------------------------------------------
   InitlMatrix

   initializes a 2 dimensional matrix of longs.

   The pointer to the vector of vectors is returned if no error occurs,
   otherwise the NULL value is returned. It is an error to call it with
   cVectors = 0 or cElemsEach = 0.
*/

long **InitlMatrix (long cVectors, long cElemsEach)
{
  register long i;
  long **rgp;

  if ((cVectors == 0) || (cElemsEach == 0)) {
    printf ("Error: zero length array allocation in InitlMatrix - Exiting\n");
    exit (0);
  }

  rgp = (long **) malloc (cVectors * sizeof(long *));

  if (rgp) {
    for (i = 0; i < cVectors; i++) {
      rgp[i] = (long *) malloc (cElemsEach * sizeof(long));
      if (!rgp[i]) {
        rgp = 0;
        break;
      } /* if */
    } /* for */
  } /* if */

  return (rgp);

} /* InitlMatrix */


/* ----------------------------------------------------------------------------
   ColumnMeans
*/
void ColumnMeans (long cRows, long cCols, double **x, double *x_bar)
{
  register long i, l;

  for (l = 0; l < cCols; l++) x_bar[l] = 0.0;

  for (i = 0; i < cRows; i++)
    for (l = 0; l < cCols; l++) x_bar[l] += x[i][l];

  for (l = 0; l < cCols; l++) x_bar[l] /= cRows;

} /* ColumnMean */


/* -----------------------------------------------------------------------------
   Cholesky

   Does the Cholesky decomposition of a variance matrix:
   Compute the matrix A such that AAt = VAR.

   Returns 0 if successful, -1 otherwise.
   The variance matrix is destroyed in the process !
*/

int Cholesky (PDOUBLE *prgdVariance, PDOUBLE *prgdComponent, long lNparams)
{
  register int i, j, k;
  double dSum;

  for (i = 0; i < lNparams; i++)
    for (j = 0; j < lNparams; j++) prgdComponent[i][j] = 0.0;

  for (i = 0; i < lNparams; i++)
    for (j = i; j < lNparams ; j++) {
      dSum = prgdVariance[i][j];
      for (k = i - 1; k >= 0 ; k--)
        dSum = dSum - prgdVariance[i][k] * prgdVariance[j][k];

      if (i == j) {
      	if (dSum <= 0.0) {
            printf ("Warning: input matrix for Cholesky is not "
                  "positive definite\n");
            return 0;
        }
        else
          prgdComponent[i][i] = sqrt(dSum);
      }
      else prgdVariance[j][i] = dSum / prgdComponent[i][i];
    } /* fin de for j */

  for (i = 0; i < lNparams ; i++)
    for (j = i+1; j < lNparams ; j++)
      prgdComponent[j][i] = prgdVariance[j][i];

  /* success */
  return 1;

} /* Cholesky */


/* End */
