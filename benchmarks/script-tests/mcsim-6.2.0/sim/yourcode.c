/* yourcode.c

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

   Contains the routines most susceptible to be modified by the user and 
   some utilities.   
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#include "lexerr.h"
#include "simmonte.h"
#include "yourcode.h"


/* ---------------------------------------------------------------------------
   DoStep_by_Step 
   
   routine called after each successful step of the integrator. 
   It can be used for interupts management, for step by step
   printing or operations such as finding a maximum etc.
*/
void DoStep_by_Step (/* your needed parameters here, e.g.:
                        double t, long *neq, double *y */)
{
  /* example of code:  
  static FILE *fout;
  int i;

  if (!fout) fout = fopen("step.out", "w");
  
  fprintf(fout, "%g\t", t);
  for (i = 1; i <= *neq; i++) fprintf(fout, "%g\t", y[i]);
  fprintf(fout, "\n");
  
  */
  
} /* DoStep_by_Step */


/* ---------------------------------------------------------------------------
   TransformPred

   At least flattens the model predictions in a simple array after a 
   Monte Carlo or a SetPoint simulation.

   Changing this routine should be avoided and using output variables 
   defined through the model specification file should be preferred.
   
   If you change it make sure that you allocate the data array of the 
   pMCPredOut structure and specify its length. See the routine
   OutspecToLinearArray for exemple.

   At most it allows the user to manipulate the data output 
   for creating summaries (e.g. sums of variables) which
   better relate to the experimental data simulated. Those summaries 
   are placed in the pMCPredOut structure and will be used and printed.
*/
   
void TransformPred (PANALYSIS panal, PMCPREDOUT pMCPredOut)
{

  OutspecToLinearArray (panal, pMCPredOut); /* no tranformation */

} /* TransformPred */


/* ---------------------------------------------------------------------------
   OutspecToLinearArray

   Flattens the panal nested output arrays on a single array.
   Allocate the data array of the pMCPredOut structure and sets the
   dimension pMCPredOut->nbrdy to the length of the data array.
*/

void OutspecToLinearArray (PANALYSIS panal, PMCPREDOUT pMCPredOut)
{
  POUTSPEC pos;
  long i, j, k;

  pMCPredOut->nbrdy = 0;

  /* get the size needed for the data array of pMCPredOut
     there should be one cell for each experiment, variable, 
     and output time
  */
  for (i = 0; i < panal->expGlobal.iExp; i++)
    for (j = 0, pos = &panal->rgpExps[i]->os; j < pos->nOutputs; j++)
      for (k = 0; k < pos->pcOutputTimes[j]; k++)
      pMCPredOut->nbrdy++;

  /* allocate data */

  if (!(pMCPredOut->pred))
    if ( !(pMCPredOut->pred = InitdVector (pMCPredOut->nbrdy)))
      ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "OutspecToLinearArray", NULL);

  pMCPredOut->nbrdy = 0;
  /* fill in pred array */
  for (i = 0; i < panal->expGlobal.iExp; i++)
    for (j = 0, pos = &panal->rgpExps[i]->os; j < pos->nOutputs; j++)
      for (k = 0; k < pos->pcOutputTimes[j]; k++)
        pMCPredOut->pred[pMCPredOut->nbrdy++] = pos->prgdOutputVals[j][k];

} /* OutspecToLinearArray */


/* ---------------------------------------------------------------------------
   Definite_Integral

   Performs the integration of an analytic function using Rhomberg algorithm.

   Adapted from the algorithm described in the book Numerical Recipes by
   Press et al. 

*/

double Definite_Integral (double (*Function)(double), double dFrom, double dTo)
{
  #define PRECISION 1.0e-6
  #define MAX_STEPS 25
  #define MIN_STEP  4

  double dTotal_Area, dDelta_Area;
  double pdArea[MAX_STEPS+2], pdTemp[MAX_STEPS+2];
  int i;

  if (dFrom >= dTo) {
    if (dFrom == dTo)
      return (0);
    else {
      printf ("\nError: inverted integration bounds in Definite_Integral - "
              "Exiting\n\n");
      exit (0);
    }
  }

  pdTemp[0] = 1;

  for (i = 0; i < MAX_STEPS; i++) {

    pdArea[i] = Trapezes (Function, dFrom, dTo, i+1);

    if (i >= MIN_STEP) {
      Interpolate_Poly (&pdTemp[i-MIN_STEP], &pdArea[i-MIN_STEP], MIN_STEP + 1, 
                        0.0, &dTotal_Area, &dDelta_Area);

      if (dTotal_Area == 0) {
        if (fabs(dDelta_Area) < PRECISION) 
          return dTotal_Area;
      }
      else {
        if (fabs(dDelta_Area) < PRECISION * fabs(dTotal_Area)) 
          return dTotal_Area;
      }
    }

    pdArea[i+1] = pdArea[i];
    pdTemp[i+1] = 0.25 * pdTemp[i];

  }

  printf ("\nError: Too many steps in routine Definite_Integral "
          "- Exiting\n\n");
  exit (0);

  #undef PRECISION
  #undef MAX_STEPS
  #undef MIN_STEP

} /* Definite_Integral */


/* ---------------------------------------------------------------------------
   Interpolate_Poly

   Adapted from the algorithm described in the book Numerical Recipes by
   Press et al. 

*/

void Interpolate_Poly (double rgdX[], double rgdY[], int n, double x, 
                       double *pdY, double *pdDY)
{
  int i, j, nIndex = 1;
  double dDenom, dDiff, dTemp1, dTemp2;
  static PDOUBLE pdTerm1 = NULL, pdTerm2 = NULL;

  if (!pdTerm1)
    if ( !(pdTerm1 = InitdVector (n+1)) || !(pdTerm2 = InitdVector (n+1)))
      ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "Interpolate_Poly", NULL);

  dDiff = fabs (x - rgdX[0]);
  pdTerm1[0] = rgdY[0];
  pdTerm2[0] = rgdY[0];

  for (i = 1; i < n; i++) { 
    /* find the smallest difference between x and components of x[] */
    if ((dTemp1 = fabs (x - rgdX[i])) < dDiff) {
      nIndex = i;
      dDiff = dTemp1;
    }
    pdTerm1[i] = rgdY[i];
    pdTerm2[i] = rgdY[i];
  }

  *pdY = rgdY[nIndex--];

  for (j = 1; j < n; j++) {
    for (i = 0; i < n - j; i++) {
      dTemp1 = rgdX[i]   - x;
      dTemp2 = rgdX[i+j] - x;
      if ((dDenom = dTemp1 - dTemp2) == 0) {
       printf ("\nError: null denominator in Interpolate_Poly - Exiting\n\n");
       exit (0);
      }
      dDenom = (pdTerm1[i+1] - pdTerm2[i]) / dDenom;
      pdTerm2[i] = dTemp2 * dDenom;
      pdTerm1[i] = dTemp1 * dDenom;
    }
    
    *pdDY = (2 * (nIndex + 1) < (n - j) ? 
             pdTerm1[nIndex+1] : pdTerm2[nIndex--]);

    *pdY = *pdY + *pdDY;
  }

  /* free(pdTerm1); free(pdTerm2); */

} /* Interpolate_Poly */


/* ---------------------------------------------------------------------------
   Trapezes

   Integrates an analytic function using the trapeze method. This is called
   recursively.
   The function to integrate is evaluated 2^(nStep - 2) times.

   Adapted from the algorithm described in the book Numerical Recipes by
   Press et al. 

*/

double Trapezes (double (*Function)(double x), double dFrom, double dTo, 
                 int nSteps)
{

  double x, dSum, dDelta;
  static double dStoredArea;
  int i, j;

  if (nSteps == 1) { /* a single trapeze */
    return (dStoredArea = 0.5 * (dTo - dFrom) * 
                          ((*Function)(dFrom) + (*Function)(dTo)));
  } 
  else {
    i = 1;
    for (j = 0; j < nSteps - 2; j++) 
      i = i << 1;

    dDelta = (dTo - dFrom) / (double) i;

    x = dFrom + 0.5 * dDelta;
    dSum = 0;
    while (x < dTo) {
      dSum = dSum + (*Function)(x);
      x = x + dDelta;
    }

    return (dStoredArea = 0.5 * (dStoredArea + dDelta * dSum));

  }

} /* Trapezes */


/* End */
