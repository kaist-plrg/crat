/* simo.h

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

   Header file for simo.c
*/

#ifndef SIMO_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions
*/

#include "yourcode.h"
#include "sim.h"


/* ----------------------------------------------------------------------------
   Prototypes
*/

void CloseMCFiles (PANALYSIS);
void SaveOutputs (PEXPERIMENT, PDOUBLE);
void NextOutputTime (PEXPERIMENT, PDOUBLE, PINT);
void OpenMCFiles (PANALYSIS panal);
void WriteMCHeader (PFILE, PANALYSIS);
void WriteMCOutput (PANALYSIS, PMCPREDOUT);
void WriteNormalOutput (PANALYSIS, PEXPERIMENT);
int  WriteOneMod (PVOID, PVOID);

#define SIMO_H_DEFINED
#endif  /* SIMO_H_DEFINED */

/* End */

