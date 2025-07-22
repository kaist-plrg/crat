/* lexfn.h

   Written by Don Maszle
   15 October 1991

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

   Header file for input definitions.
*/

#ifndef LEXFN_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions  */

#include "lex.h"

/* ----------------------------------------------------------------------------
   Constants  */

/* Input Function constants */

#define IFN_NULL     0
#define IFN_CONSTANT 1
#define IFN_PERDOSE  2
#define IFN_PEREXP   3
#define IFN_NDOSES   4
#define IFN_SPIKES   5
#define IFN_EVENTS   6
#define IFN_PERTRANS 7

/* ----------------------------------------------------------------------------
   Typedefs  */

/* IFN:  Predefined input functions

   The parameters of interest vary depending on the type of the function.
   I know, this should really be a union, but they are kind of messy and
   this needs to be redone more intelligently anyway.

   At any point in time, dVal is the current value (this is calculated
   for a given point in time by CalcInputs().  For pulsed inputs, bOn
   flags whether or not the pulse is active.  Dependencies of IFN parms on
   on model parameters are respected via the handle fields

   Here is what the parms mean:

     IFN_CONSTANT  dMag is the constant

   -- Periodic functions:  Period = dTper, magnitude = dMag, start time = dT0

     IFN_PERDOSE  Periodic dose lasting dTexp
     IFN_PEREXP   Periodic exponential with decay constant dDecay
     IFN_PERTRANS Periodic transit (analytical multicompartment) Savic model
                  Savic et al., J Pharmacokinet Pharmacodyn. 2007, 34:711.

   -- Multiple pulse functions:

     IFN_NDOSES   nDoses of rgMags[] starting at rgT0s[]
     IFN_SPIKES   nDoses spikes of rgMags[] at time rgT0s[]
     IFN_EVENTS   nDoses discontinuities in a state variable
*/

typedef struct tagIFN {
  /* Bookkeeping */

  int    iType;            /* One of the IFN_ types */
  BOOL   bOn;              /* TRUE if exposure is On */
  double dTStartPeriod;    /* Start of current period */
  double dVal;             /* Current value: CalcInputs updates */

  /* Periodic functions */

  double dMag;             /* Magnitude of input */
  double dTper;            /* Duration of Period */
  double dT0;              /* Starting time of exposure */
  double dTexp;            /* Exposure duration */

  /* For exponential inputs, the exponential decay rate
     Exposure lasts for N_TAU_EXPOSE Tau periods. (tau=1/Decay)
     After this, input is considered to be neglible. */
  double dDecay;

  /* For Savic's transit model inputs 
     dMag * ((dDecay * t)^n) * exp(-dDecay * t) / n!
     with n! = SQRT2PI * (n^(n+0.5)) *exp(-n)
     So we need n, the number of virtual transit compartments (as a double) */
  double dNcpt; 

  /* Dependencies for the periodic parms */

  HANDLE hMag;             /* Handle to magnitude */
  HANDLE hTper;            /* Handle to period */
  HANDLE hT0;              /* Handle to starting time */
  HANDLE hTexp;            /* Handle to exposure time */
  HANDLE hDecay;           /* Handle to exponential decay rate */
  HANDLE hNcpt;            /* Handle to the number of virtual compartments */

  /* Multiple dose inputs */

  int nDoses;              /* Number of doses of Spikes */
  int iDoseCur;            /* Current Dose */

  /* For value input */
  PDOUBLE rgT0s;           /* Array of start times */
  PDOUBLE rgMags;          /* Array of magnitudes */

  /* For variable input */
  HANDLE *rghT0s;          /* Handles to start times */
  HANDLE *rghMags;         /* Handles to magnitudes */

  /* For events */
  HANDLE target_state;
  PINT   rgOper;           /* Array of operation types */

} IFN, *PIFN; /* struct tagIFN */


/* ----------------------------------------------------------------------------
   Prototypes */

BOOL DefDepParm (PSTR szLex, PDOUBLE pdValue, HANDLE *phvar);

int  GetFnType (PSTR szName);
BOOL GetInputArgs (PINPUTBUF pibIn, PIFN pifn, int n);
BOOL GetInputFn (PINPUTBUF pibIn, PSTR sz, PIFN pifn);
BOOL GetNDoses (PINPUTBUF pibIn, PSTR szLex, PIFN pifn);
BOOL GetNNumbers (PINPUTBUF pibIn, PSTR szLex, int nNumbers, PDOUBLE rgd);
BOOL GetSpikes (PINPUTBUF pibIn, PSTR szLex, PIFN pifn);
BOOL GetEvents (PINPUTBUF pibIn, PSTR szLex, PIFN pifn);

void InitIFN (PIFN pifn);

#define LEXFN_H_DEFINED
#endif

/* End */

