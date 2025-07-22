/* delays.c

   Originally written by Frederic Bois
   
   Copyright (c) 2015-2017 Free Software Foundation, Inc.

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

   Contains routines for handling delay differential equations (essentially
   initialization of memory arrays, storage and retrieval.   
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#include "lexerr.h"
#include "simmonte.h"
#include "yourcode.h"


/* ----------------------------------------------------------------------------
   Constants
*/

/* Delay storage size */
#define MAX_DELAY 1000


/* -----------------------------------------------------------------------------
   externs: delay management stuff
*/
extern double vrgModelVars[]; /* States and Outputs */


/* WARNING! ===== GLOBAL VARIABLES ! */

/* -----------------------------------------------------------------------------
   globals to this file
*/
static double dInitialTime;
int    iCurrentTime;
double *rgdTime = NULL;
long   *rgiVars = NULL;
double **pdVar  = NULL;


/* -----------------------------------------------------------------------------
   CalcDelay

   Returns the value of a state or output variable at a given (past) time.
   Past values are stored in ... The step back in time is given "delay", the
   current time is dTime. A loop is used.
*/

double CalcDelay (HVAR hvar, double dTime, double delay)
{
  int i;
  int sentinel; /* stopping flag for circular array */
  double dTmp, oldTime;

  /* printf("\n\n in CalcDelay, time = %g", dTime); */

  /* if the handle has not been seen, create the storage for that variable.
     We should check that it's a state or output variable... */
  if (!rgiVars[hvar]) {
    /* printf("\n => creating past array for variable %ld...\n", hvar); */
    pdVar[hvar] = InitdVector(MAX_DELAY);
    pdVar[hvar][0] = vrgModelVars[hvar];
    rgiVars[hvar] = 1; /* mark as initialized */
  }

  /* negative or null delays are forbidden */
  /* that should be handled by mod, or stay dynamic? */
  if (delay <= 0) {
    printf ("\nError: negative or null delays aren't allowed - Exiting.\n");
    exit(0);
    /* for null delays the variable's current value could be returned... */
  }
  else { /* delay strictly positive, OK */

    /* printf("\n back time: %e - %e = %e\n", dTime, delay, dTime - delay); */

    oldTime = dTime - delay;
    if (oldTime <= dInitialTime) {
      /* printf("\n default var %ld at val %g\n", hvar, pdVar[hvar][0]); */
      return (pdVar[hvar][0]);
    } 
    else { /* past time sought is after initial time */

      /* find the index of time (dTime - delay). 
         The current time is stored in rgdTime[iCurrentTime].
         We could save on searching time for multiple calls at the same time */
      i = iCurrentTime - 1;
      if (i < 0)
        i = MAX_DELAY - 1;
      sentinel = 0;
      while (rgdTime[i] > oldTime) {
        i = i - 1;
        if (i < 0)
          i = MAX_DELAY - 1;
        sentinel = sentinel + 1;
        /* printf("\n sentinel: %d\n", sentinel); */
        if (sentinel > MAX_DELAY - 1) {
          printf ("Error: size MAX_DELAY of rgdTime array = "
                  "%ld too small.\n", (long) MAX_DELAY);
          exit(0);
        }
      } /* end while */
 
      /* compute delayed term */
      if (i == (iCurrentTime - 1)) { 
        /* an integration step larger than delay is attempted; it's a special 
           case: interpolate between i-1 and i, rather than between i and 
           iCurrentTime (which points to an empty cell or outdated ones).
           Note that deSolve forbids that... */
        dTmp = pdVar[hvar][i-1] + 
               ((pdVar[hvar][i] - pdVar[hvar][i-1]) * 
                (oldTime - rgdTime[i-1]) / (rgdTime[i] - rgdTime[i-1]));
      }
      else { 
        /* time step < delay, OK, interpolate between i and i+1 */
        dTmp = pdVar[hvar][i] + 
               ((pdVar[hvar][i+1] - pdVar[hvar][i]) * 
                (oldTime - rgdTime[i]) / (rgdTime[i+1] - rgdTime[i]));
      }  

      /* printf("\n computed var %ld at val %g\n", hvar, dTmp); */
    }
  
    return dTmp;

  } /* else */

} /* CalcDelay */


/* ---------------------------------------------------------------------------
   InitDelays
   
   routine called at start if delay differential equations are used.
   Initializes storage and internal bookkeeping variables.
*/
void InitDelays (double dTime)
{
  int i;

  /* printf("\n\n in InitDelays"); */

  /* initialize the time storage */
  if (!rgdTime) {
    rgdTime = InitdVector(MAX_DELAY);
    iCurrentTime = -1;
    dInitialTime = dTime;
    /* printf("\n => past time array created."); */
  }
      
  /* initialize a table of the delayed variables
     note that the storage vectors pdVar[x] will be init in CalcDelay
     upon first call to that function for a given x (state or output)
     That is messy and should be handled by the model generator */
  if (!rgiVars) {
    rgiVars = InitlVector(GetNModelVars());
    pdVar   = InitpdVector(GetNModelVars());
    for (i = 0; i < GetNModelVars(); i++)
      rgiVars[i] = 0;
  }

} /* InitDelays */


/* ---------------------------------------------------------------------------
   StoreDelayed
   
   routine called after each successful step of the integrator. 
   It stores current time, states and outputs values for later retrieval. 
*/
void StoreDelayed (double t)
{
  int i;

  /* printf("\n\n in StoreDelayed, time %g\n", t); */

  iCurrentTime++;
  if (iCurrentTime == MAX_DELAY) /* loop */
    iCurrentTime = 0;

  rgdTime[iCurrentTime] = t;

  for (i = 0; i < GetNModelVars(); i++)
    if (rgiVars[i]) {
      pdVar[i][iCurrentTime] = vrgModelVars[i];
      /* printf(" stored var %d, val %g\n", i, vrgModelVars[i]); */
    }

} /* StoreDelayed */


/* End */
