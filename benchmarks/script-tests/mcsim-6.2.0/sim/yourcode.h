/* yourcode.h

   Originally written by Frederic Bois
   
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

   Header file for the "yourcode.c" file, containing customizable routines.
*/

#ifndef YOURCODE_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions
*/

#include "sim.h"


/* ----------------------------------------------------------------------------
   Typedefs
*/

typedef struct tagMCPREDOUT {
  long nbrdy;    /* number of kinetic ys */
  double *pred;  /* pointer to the data */
  int passflag;  /* typically a pass/fail flag */
} MCPREDOUT, *PMCPREDOUT;


/* ----------------------------------------------------------------------------
   Prototypes  */

void   DoStep_by_Step (/* your needed parameters here, e.g.:
                          double t, long *neq, double *y */);
void   OutspecToLinearArray (PANALYSIS panal, PMCPREDOUT pMCPredOut);
void   TransformPred (PANALYSIS, PMCPREDOUT);
double Definite_Integral (double (*Function)(double), double dFrom, double dTo);
void   Interpolate_Poly (double rgdX[], double rgdY[], int n, double x, 
                         double *pdY, double *pdDY);
double Trapezes (double (*Function)(double x), double dFrom, double dTo, 
                 int nSteps);

#define YOURCODE_H_DEFINED
#endif  /* YOURCODE_H_DEFINED */

/* End */

