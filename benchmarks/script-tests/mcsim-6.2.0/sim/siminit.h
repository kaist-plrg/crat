/* siminit.h

   Originally written by Don Maszle

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

   Header file for simulation
*/

#ifndef SIMINIT_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions
*/

#include "sim.h"


/* ----------------------------------------------------------------------------
   Prototypes
*/

void CreateOutputSchedule (POUTSPEC pos);
BOOL FindNewPoint (POUTSPEC pos, PINT piPoint);
void GetModelInfo (PMODELINFO pmi);
void InitAnalysis (PANALYSIS panal);
void InitExperiment (PEXPERIMENT pexp, PMODELINFO pmodelinfo);
void InitIntegratorSpec (PINTSPEC pis);
void InitMonteCarlo (PMONTECARLO pmc);
void InitGibbs (PGIBBSDATA pgd);
int  InitOneOutVar (PVOID pData, PVOID pInfo);
int  InitOneDataVar (PVOID pData, PVOID pInfo);
BOOL InitOutputs (PEXPERIMENT pexp, PINT piOut, PDOUBLE pdTout);
void InitOutputSpec (POUTSPEC pos);
BOOL PrepareOutSpec (PEXPERIMENT pexp);
BOOL PrintOutSpec (PEXPERIMENT pexp);

#define SIMINIT_H_DEFINED
#endif  /* SIMINIT_H_DEFINED */

/* End */

