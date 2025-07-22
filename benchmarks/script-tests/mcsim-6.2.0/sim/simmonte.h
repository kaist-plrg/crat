/* simmonte.h

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

   Header file for simmonte.c
*/

/* ----------------------------------------------------------------------------
   Prototypes */

void CalcMCParms (PMONTECARLO pMC, double rgParms[], long iStart);
int  CalculateOneMCParm (PMCVAR pMCVar);
double GetParm (PMCVAR pMCVar, int iIndex);
BOOL GetSPMods (PANALYSIS panal, double rgdOptionalParms[]);
BOOL InitSetPoints (PMONTECARLO pMC);
BOOL ReadSetPoints (PMONTECARLO pMC, double rgParms[]);
void SetParents (PMONTECARLO pMC, long iStart);
void SetParms (long cParms, HVAR *rghvar, double *rgdParm);
void SetParmsLog (long cParms, HVAR *rghvar, double *rgdParm);
void SetParmsExp (long cParms, HVAR *rghvar, double *rgdParm);

/* End */

