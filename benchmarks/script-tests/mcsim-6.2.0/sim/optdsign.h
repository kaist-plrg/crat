/* optdesign.h

   Originally written by Frederic Bois
   
   Copyright (c) 1997-2017 Free Software Foundation, Inc.

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

   Header file for optdesign.c
*/

#ifndef OPTDESIGN_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions
*/

#include <float.h>  /* Floating point limits */

#include "sim.h"


/* ----------------------------------------------------------------------------
   Prototypes
*/

void CloseOptFiles (PANALYSIS panal);

void Compute_cost (long nData, int *piData_mask, double *dCost);

void Do_Importance_Ratios (double **pdY, PMCVAR *pLikes, long nSims, 
                           long nPreds, long nDesignPts, int *piDesign_mask,
                           int nDesignPt_tried, double *pdIR);

void DoOptimalDesign (PANALYSIS panal);

double DoVariance (long nDim, double *pdImpR, double **pdX,
                   long istart, long ifinish);

int Estimate_y (PANALYSIS panal, double *pdTheta, double *pdPred);

void Importance_Resample (long nMod, long *pIndex0, long *pIndex1, 
                          long *plDrawn, double *pdLL, double dSumL);

void InitOptArrays (PANALYSIS panal, int **piDesign_mask, 
                    long *pnDesignPts, double ***pdY, long *pnPreds, 
                    long *pnStartDecisionPts, double **pdVariance,
                    double **pdIR, long nSims);

void OpenOptFiles (PANALYSIS panal);

void ReadAndSimulate (PANALYSIS panal, long nParms, 
                      double **pdY, long nPred, PMCVAR *pLikes, long nSims);

void SetupLikes (PANALYSIS panal, long nDesignPts, PMCVAR **pdLikes);

void WriteOptimOut (PANALYSIS panal, long iter, long nDesignPts, int criterion,
                    double *pdVariance, int *piData_mask, long iCrit, 
                    double dCrit, double dCost);

void WriteOutHeader (PANALYSIS panal, int criterion);

#define OPTDESIGN_H_DEFINED
#endif  /* OPTDESIGN_H_DEFINED */

/* End */

