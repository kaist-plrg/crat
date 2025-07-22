/* matutil.h

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
*/

#ifndef MATUTIL_H_DEFINED

#include "hungtype.h"

/* ----------------------------------------------------------------------------
   Prototypes */

int    Cholesky (PDOUBLE *prgdVariance, PDOUBLE *prgdComponent, long lNparams);

void   ColumnMeans (long cRows, long cCols, double **x, double *x_bar);

double **InitdMatrix (long cVectors, long cElemsEach);

long   **InitlMatrix (long cVectors, long cElemsEach);

double *InitdVector (long cVectors);

int    *InitiVector (long cVectors);

long   *InitlVector (long cVectors);

double **InitpdVector (long cVectors);

double *LogTransformArray (long nElems, double *rgdSrc, double *rgdDes);

/* End */

#define MATUTIL_H_DEFINED
#endif  /* _MATUTIL_H_ */




