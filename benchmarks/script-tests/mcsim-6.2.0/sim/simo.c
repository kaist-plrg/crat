/* simo.c

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

   Output routines for the simulation
*/

#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "lexerr.h"
#include "simo.h"
#include "modelu.h"

static char vszDefOutFilename[] = "sim.out";
static char vszDefMCOutFilename[] = "simmc.out";


/* ----------------------------------------------------------------------------
   SaveOutputs

   Also saves states
*/

void SaveOutputs (PEXPERIMENT pexp, PDOUBLE pdTout)
{
  #define SO_EPSILON (1e-100) /* Smaller values are zeroed  */

  static     PDOUBLE rgdInterpStates, rgdInterpDeriv;
  int        i, j, index;
  PMODELINFO pmod = pexp->pmodelinfo;
  POUTSPEC   pos = &pexp->os;
  extern     IFN vrgInputs[]; /* Input Function records */

  if (!(rgdInterpStates) || !(rgdInterpDeriv))
    if ( !(rgdInterpStates = InitdVector (GetNModelVars ())) ||
         !(rgdInterpDeriv  = InitdVector (GetNModelVars ())))
      ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "SaveOutputs", NULL);

  memcpy (rgdInterpStates, pmod->pdModelVars,
          pmod->nModelVars*sizeof(double));

  /* Update inputs and outputs defined only in Dynamics */
  CalcDeriv(rgdInterpStates, rgdInterpDeriv, pdTout);

  /* Update output scaling */
  CalcOutputs (rgdInterpStates, rgdInterpDeriv, pdTout);

  for (i = 0; i < pos->nOutputs; i++) {

    /* Save interpolated value if there are still times to output
       for this variable, and if this time is scheduled */

    if (pos->piCurrentOut[i] < pos->pcOutputTimes[i]
        && *pdTout == pos->prgdOutputTimes[i][pos->piCurrentOut[i]]) {
      double dTmp = 0;

      if (IsModelVar(pos->phvar_out[i]))  /* Use interp'd model value */
        dTmp = rgdInterpStates[ ModelIndex(pos->phvar_out[i])];

      else { /* Use current parm/input value */
        index = HINDEX(pos->phvar_out[i]);
        if (IsInput(pos->phvar_out[i]) &&
            (vrgInputs[index].iType == IFN_SPIKES)) {

          j = vrgInputs[index].iDoseCur;

          if ((vrgInputs[index].rgT0s[j] == pexp->dTime) &&
              (j < vrgInputs[index].nDoses))
            dTmp = vrgInputs[index].rgMags[j];
          else
            dTmp = 0;

        }
        else
          dTmp = GetVarValue (pos->phvar_out[i]);
      }

      if (fabs(dTmp) < SO_EPSILON) /* Avoid silly little numbers  */
        dTmp = 0.0;

      pos->prgdOutputVals[i][pos->piCurrentOut[i]++] = dTmp;

    } /* if */
  } /* for */

} /* SaveOutputs */


/* -----------------------------------------------------------------------------
   NextOutputTime

   Returns in pdTout,the next time, pdTout, at which an variable is
   to be output.
*/

void NextOutputTime (PEXPERIMENT pexp, PDOUBLE pdTout, PINT piOut)
{
  if (pexp->dTime < pexp->dTfinal) {
    if (++*piOut < pexp->os.cDistinctTimes) {
      *pdTout = pexp->os.rgdDistinctTimes[*piOut];
    }
    else {
      *pdTout = pexp->dTfinal;
    }
  }

} /* NextOutputTime */


/* -----------------------------------------------------------------------------
   WriteOneMod

   writes one parameter modification from the list.   Inputs are *not*
   written.
*/

int WriteOneMod (PVOID pData, PVOID pInfo)
{
  PMCVAR pmcvar = (PMCVAR) pData;
  PFILE pfile = (PFILE) pInfo;

  if (!IsInput (pmcvar->hvar))
    fprintf(pfile, "%g\t", pmcvar->dVal);

  return 0;

} /* WriteOneMod */


/* -----------------------------------------------------------------------------
   WriteMCHeader

   Write a tabulated text header with the run number, the list of parameters
   and outputs.
*/

void WriteMCHeader (PFILE pfileOut, PANALYSIS panal)
{
  long i, j, k;
  PMONTECARLO pmc = &panal->mc;
  OUTSPEC *pos;

  fprintf (pfileOut, "Iter");

  for (i = 0; i < pmc->nParms; i++)
   fprintf (pfileOut, "\t%s", GetVarName(pmc->rgpMCVar[i]->hvar));

  /* print the outputs as they come with experiment and time code */
  for (i = 0; i < panal->expGlobal.iExp; i++) {
    pos = &panal->rgpExps[i]->os;
    for (j = 0; j < pos->nOutputs; j++) {
      for (k = 0; k < pos->pcOutputTimes[j]; k++)
         fprintf (pfileOut, "\t%s_%ld.%ld", pos->pszOutputNames[j], i+1, k+1);
    } /* for j */
  } /* for i */

  fprintf (pfileOut, "\n");

  fflush (pfileOut);

} /* WriteMCHeader */


/* -----------------------------------------------------------------------------
   OpenMCFiles

   For each processor (if more than one), open the Monte Carlo output
   file to be written to by WriteMCOutput()

   Report fatal error (will lead to exit) if the file cannot be opened.
*/
void OpenMCFiles (PANALYSIS panal)
{
  PMONTECARLO pmc = &panal->mc;

  /* Use command line spec if given */
  if (panal->bCommandLineSpec) {
    free (pmc->szMCOutfilename);
    panal->bAllocatedFileName = FALSE;
    pmc->szMCOutfilename = panal->szOutfilename;
  }
  else
    if (!(pmc->szMCOutfilename)) /* Default if none given */
      pmc->szMCOutfilename = vszDefMCOutFilename;

  /* prefix the filename with the rank of the process if more than one
     process is used */
  if (panal->size > 1) {
    char* with_rank = malloc(sizeof(char)*(6+strlen(pmc->szMCOutfilename)));
    sprintf(with_rank, "%04d_%s", panal->rank, pmc->szMCOutfilename);
    pmc->szMCOutfilename = with_rank;
  }

  if (!pmc->pfileMCOut
      && !(pmc->pfileMCOut = fopen (pmc->szMCOutfilename, "w"))) {
    ReportError (NULL, RE_FATAL | RE_CANNOTOPEN, pmc->szMCOutfilename,
                 "OpenMCFiles()");
  }

  WriteMCHeader (pmc->pfileMCOut, panal);

} /* OpenMCFiles */


/* -----------------------------------------------------------------------------
   CloseMCFiles

   Closes output files associated with Monte Carlo and set points runs
*/

void CloseMCFiles (PANALYSIS panal)
{
  fclose (panal->mc.pfileMCOut);
  printf ("\nWrote results to \"%s\"\n", panal->mc.szMCOutfilename);

} /* CloseMCFiles */


/* -----------------------------------------------------------------------------
   WriteMCOutput

   Output the parameters for this run and the results of the
   simulation (passed through TransformPred).
*/

void WriteMCOutput (PANALYSIS panal, PMCPREDOUT pmcpredout)
{
  PFILE pfileMC;
  PMONTECARLO pmc = &panal->mc;

  pfileMC = pmc->pfileMCOut;

  fprintf (pfileMC, "%ld\t", panal->mc.lRun);

  /* Include parameter values for that run */
  WriteArray (pfileMC, panal->mc.nParms, panal->mc.rgdParms);
  fprintf (pfileMC, "\t");

  /* write the flattened and eventually transformed predictions */
  WriteArray (pfileMC, pmcpredout->nbrdy, pmcpredout->pred);
  fprintf (pfileMC, "\n");

  fflush (pfileMC);

} /* WriteMCOutput */


/* -----------------------------------------------------------------------------
   WriteNormalOutput

   Write the results in the output file. This procedure is
   called only from time to time in order to save storage space
*/
void WriteNormalOutput (PANALYSIS panal, PEXPERIMENT pexp)
{
  long     i, j;
  PFILE    pfile;
  POUTSPEC pos;

  if (!panal) return;

  pos = &pexp->os;

  if (!panal->szOutfilename)
    panal->szOutfilename = vszDefOutFilename;

  /* prefix the filename with the rank of the process if more than one
     process is used */
  if (panal->size > 1) {
    char* with_rank = malloc(sizeof(char)*(5+strlen(panal->szOutfilename)));
    sprintf(with_rank,"%4d%s",panal->rank,panal->szOutfilename);
    panal->szOutfilename = with_rank;
  }

  if (!(panal->pfileOut))
    if (!(panal->pfileOut = fopen (panal->szOutfilename, "w")))
      ReportError (NULL, RE_CANNOTOPEN | RE_FATAL, panal->szOutfilename, NULL);

  pfile = panal->pfileOut;
  fprintf (pfile, "Results of Simulation %d\n\n", pexp->iExp);

  /* Vertical output:  Formatted  Time1    Out_Var1  Out_Var2 ... */

  fprintf (pfile, "Time");

  for (i = 0; i < pos->nOutputs; i++)
    fprintf (pfile, "\t%s", pos->pszOutputNames[i]);
  fprintf (pfile, "\n");

  for (j = 0; j < pos->nOutputs; j++)
    pos->piCurrentOut[j] = 0;

  for (i = 0; i < pos->cDistinctTimes; i++) {
    fprintf (pfile, "%g", pos->rgdDistinctTimes[i]);
    for (j = 0; j < pos->nOutputs; j++) {

      if (pos->piCurrentOut[j] < pos->pcOutputTimes[j]
          && pos->rgdDistinctTimes[i]
          == pos->prgdOutputTimes[j][pos->piCurrentOut[j]])
        fprintf (pfile, "\t%g",
                 pos->prgdOutputVals[j][pos->piCurrentOut[j]++]);

      else
        fprintf (pfile, "\t");

    } /* for */

    fprintf (pfile, "\n");

  } /* for */

  fprintf (pfile, "\n\n");

} /* WriteNormalOutput */
