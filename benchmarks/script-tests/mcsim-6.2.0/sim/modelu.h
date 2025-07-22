/* modelu.h

   Written by Don Maszle
   7 October 1991

   Copyright (c) 1991-2017 Free Software Foundation, Inc.

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

   Model utilities include file.

   Utility prototypes and structures used by the generated model
   file.  The model typedefs need to be defined here so that
   'modelu.c' can use them.
*/

#ifndef MODELU_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions  */

#include "modiface.h"
#include "delays.h"


/* ----------------------------------------------------------------------------
   Constants  */

#define ID_NULL     0x00000
#define ID_STATE    0x10000
#define ID_INPUT    0x20000
#define ID_OUTPUT   0x30000
#define ID_PARM     0x40000  /* Global parameter */
#define ID_INLINE   0xA0000  /* Inline statement */

#define HV_TYPE_MASK   0xF0000 /* Handle to variable is a DWORD */
#define HV_INDEX_MASK  0x0FFFF /* == 0xtiiii, type and index */


/* ----------------------------------------------------------------------------
   Typedefs  */

/* Global Variable Map */

typedef struct tagVM {
    PSTR szName;     /* Name of the variable */
    PVOID pVar;      /* Ptr to C variable */
    HVAR hvar;       /* Handle to variable: ID_TYPE | index */
} VMMAPSTRCT, *PVMMAPSTRCT; /* Variable Map element */


/* ----------------------------------------------------------------------------
   Macros  */

#define TYPE(pvm)    ((pvm) ? (pvm)->hvar & HV_TYPE_MASK : ID_NULL)
#define INDEX(pvm)   ((pvm) ? (pvm)->hvar & HV_INDEX_MASK: ID_NULL)

#define HTYPE(hvar)  ((hvar) & HV_TYPE_MASK)
#define HINDEX(hvar) ((int) ((hvar) & HV_INDEX_MASK))


/* ----------------------------------------------------------------------------
   Prototypes  */

void FixupDependentInputs (void);

void        GetStartPeriods (PDOUBLE pdTime);
void        GetStateHandles (HVAR *phvar);
PVMMAPSTRCT GetVarPtr (PVMMAPSTRCT pvm, PSTR szName);
int         GetVarType (HVAR hvar);

void PostUpdateSpikes (PDOUBLE pdTime);

void UpdateDefaultInput (PIFN pifn, PDOUBLE pdTnext, PDOUBLE pdTime);
void UpdateNDoses (PIFN pifn, PDOUBLE pdTnext, PDOUBLE pdTime);
BOOL UpdateSpikes (PIFN pifn, PDOUBLE pdTnext, PDOUBLE pdTime);

#define MODELU_H_DEFINED
#endif

/* End */

