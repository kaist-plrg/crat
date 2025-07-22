/* simmonte.c

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

   Handles functions related to Monte Carlo analysis.

*/

#include <assert.h>
#include <ctype.h>
#include <float.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#include "sim.h"
#include "lex.h"
#include "lexerr.h"
#include "strutil.h"
#include "simmonte.h"


/* ----------------------------------------------------------------------------
   SetParms

   sets the parameters in the rghvar array to the values in the rgdParm
   array.
*/
void SetParms (long cParms, HVAR *rghvar, double *rgdParm)
{
  long i;

  for (i = 0; i < cParms; i++)
    SetVar (rghvar[i], rgdParm[i]);

} /* SetParms */


/* ----------------------------------------------------------------------------
   SetParmsLog

   sets the parameters in the rghvar array to the log-transformed
   values in the rgdParm array.
*/
void SetParmsLog (long cParms, HVAR *rghvar, double *rgdParm)
{
  long i;

  for (i = 0; i < cParms; i++)
    SetVar (rghvar[i], log(rgdParm[i]));

} /* SetParmsLog */


/* ----------------------------------------------------------------------------
   SetParmsExp

   sets the parameters in the rghvar array to the exp-transformed
   values in the rgdParm array.
*/
void SetParmsExp (long cParms, HVAR *rghvar, double *rgdParm)
{
  long i;

  for (i = 0; i < cParms; i++)
    SetVar (rghvar[i], exp(rgdParm[i]));

} /* SetParmsExp */


/* ----------------------------------------------------------------------------
   CalculateOneMCParm

   Callback function for CalculateMCParms. Assign the dVal member of 
   a MCVAR structure by sampling from the distribution specified by its iType
*/
int CalculateOneMCParm (PMCVAR pMCVar)
{
  double dParm1, dParm2, dMin, dMax;

  /* Check pMCVar for dependencies */
  dParm1 = *(pMCVar->pdParm[0]);
  dParm2 = *(pMCVar->pdParm[1]);
  dMin   = *(pMCVar->pdParm[2]);
  dMax   = *(pMCVar->pdParm[3]);

  /* Set variable randomly according to selected distribution */
  switch (pMCVar->iType) {

    default:
    case MCV_UNIFORM:
      pMCVar->dVal = UniformRandom(dParm1, dParm2);
      break;

    case MCV_LOGUNIFORM:
      pMCVar->dVal = LogUniformRandom(dParm1, dParm2);
      break;

    case MCV_BETA:
      pMCVar->dVal = BetaRandom(dParm1, dParm2, dMin, dMax);
      break;

    case MCV_HALFNORMAL:
      pMCVar->dVal = fabs(NormalRandom(dParm1, dParm2));
      break;

    case MCV_NORMAL:
      pMCVar->dVal = NormalRandom(dParm1, dParm2);
      break;

    case MCV_NORMALCV:
      pMCVar->dVal = NormalRandom(dParm1, fabs(dParm1 * dParm2));
      break;

    case MCV_NORMALV:
      pMCVar->dVal = NormalRandom(dParm1, sqrt(dParm2));
      break;

    case MCV_TRUNCNORMAL:
      pMCVar->dVal = TruncNormalRandom(dParm1, dParm2, dMin, dMax);
      break;

    case MCV_TRUNCNORMALCV:
      pMCVar->dVal = TruncNormalRandom(dParm1, fabs(dParm1 * dParm2),
                                       dMin, dMax);
      break;

    case MCV_TRUNCNORMALV:
      pMCVar->dVal = TruncNormalRandom(dParm1, sqrt(dParm2), dMin, dMax);
      break;

    case MCV_LOGNORMAL:
      pMCVar->dVal = LogNormalRandom(dParm1, dParm2);
      break;

    case MCV_TRUNCLOGNORMAL:
      pMCVar->dVal = TruncLogNormalRandom(dParm1, dParm2, dMin, dMax);
      break;

    case MCV_LOGNORMALV:
      pMCVar->dVal = LogNormalRandom(dParm1, exp(sqrt(dParm2)));
      break;

    case MCV_TRUNCLOGNORMALV:
      pMCVar->dVal = TruncLogNormalRandom(dParm1, exp(sqrt(dParm2)),
                                          dMin, dMax);
      break;

    case MCV_CHI2:
      pMCVar->dVal = Chi2Random(dParm1);
      break;

    case MCV_BINOMIAL:
      pMCVar->dVal = BinomialRandom(dParm1, (long) dParm2);
      break;

    case MCV_PIECEWISE:
      pMCVar->dVal = PiecewiseRandom(dMin, dParm1, dParm2, dMax);
      break;

    case MCV_EXPONENTIAL:
      pMCVar->dVal = ExpRandom(dParm1);
      break;

    case MCV_GGAMMA:
      pMCVar->dVal = GGammaRandom(dParm1, dParm2);
      break;

    case MCV_INVGGAMMA:
      pMCVar->dVal = InvGGammaRandom(dParm1, dParm2);
      break;

    case MCV_TRUNCINVGGAMMA:
      pMCVar->dVal = TruncInvGGammaRandom(dParm1, dParm2, dMin, dMax);
      break;

    case MCV_POISSON:
      pMCVar->dVal = PoissonRandom(dParm1);
      break;

    case MCV_BINOMIALBETA: /* dMin is in fact beta */
      pMCVar->dVal = BinomialBetaRandom(dParm1, dParm2, dMin);
      break;

    case MCV_GENLOGNORMAL: /* dMin is in fact stdevlognorm */
      pMCVar->dVal = GenLogNormalRandom(dParm1, dParm2, dMin);
      break;

    case MCV_STUDENTT: /* dMin is in fact the scale parameter */
      pMCVar->dVal = StudentTRandom(dParm1, dParm2, dMin);
      break;

    case MCV_CAUCHY:
      pMCVar->dVal = CauchyRandom(dParm1);
      break;

    case MCV_HALFCAUCHY:
      pMCVar->dVal = fabs(CauchyRandom(dParm1));
      break;

    case MCV_USERLL: /* not allowed for straight Monte Carlo simulations */
      ReportError (NULL, RE_BADCONTEXT | RE_FATAL, "UserSpecifiedLL", NULL);
      break;

    case MCV_NEGATIVEBINOM:
      pMCVar->dVal = NegativeBinomialRandom(dParm1, dParm2);
      break;

  } /* switch */

  return 0;

} /* CalculateOneMCParm */


/* ----------------------------------------------------------------------------
   CalcMCParms

   calculates random parameters for a Monte Carlo variation.

   This routines uses arrays for the MC vars and distributions.
   It replaces the obsolete CalculateMCParms which used lists.

   The calculated parms are stored in the rgParms[] array.  If this
   array is NULL, the parms are stored in the pMC->rgParms[] array.

   The calculation starts at index iStart.
*/
void CalcMCParms (PMONTECARLO pMC, double rgParms[], long iStart)
{
  long i;

  if (!rgParms)
    rgParms = pMC->rgdParms; /* Put them in the usual place */

  for (i = iStart; i < pMC->nParms; i++) {
    CalculateOneMCParm (pMC->rgpMCVar[i]);
    rgParms[i] = pMC->rgpMCVar[i]->dVal;
  } /* for */

} /* CalcMCParms */


/* ----------------------------------------------------------------------------
   InitSetPoints

   Openn and reads the header of the SetPoint file containing the
   parameters to be tried.

   Returns the file pointer if everything is ok.
*/
BOOL InitSetPoints (PMONTECARLO pMC)
{
  register char c;
  PFILE pfile;

  if (!(pfile = fopen(pMC->szSetPointsFilename, "r")))
    ReportError (NULL, RE_CANNOTOPEN | RE_FATAL,
                 pMC->szSetPointsFilename, NULL);

  pMC->pfileSetPoints = pfile;

  /* Throw away the first line.  This allows a MC output file to be used
     directly as a setpoints file. */
  do { c = getc(pMC->pfileSetPoints); } while (c != '\n');

  if (feof(pMC->pfileSetPoints))
    ReportError (NULL, RE_INSUF_POINTS | RE_FATAL,
                 pMC->szSetPointsFilename, NULL);

  return (!pfile);

} /* InitSetPoints */


/* ----------------------------------------------------------------------------
   ReadSetPoints

   Reads set points from a file for this run.

   Returns non-zero if a full set of points is read, 0 otherwise.
*/
BOOL ReadSetPoints (PMONTECARLO pMC, double rgParms[])
{
  BOOL bReturn = FALSE; /* Initially, flag no points read */
  register char c;
  long i;

  if (!rgParms)
    rgParms = pMC->rgdParms; /* Put data in the usual place */

  /* Throw away dummy field */
  do { 
    c = getc(pMC->pfileSetPoints);
    if (feof(pMC->pfileSetPoints))
      goto Exit_ReadSetPoints;
  } while ((c != '\t') && (c != ' '));

  /* Increment across set point parms list */
  for (i = 0; i < pMC->nSetParms; i++) {

    /* Try to read one data point */
    if (feof(pMC->pfileSetPoints) ||
        (fscanf(pMC->pfileSetPoints, "%lg", &pMC->rgpMCVar[i]->dVal) == EOF)) {

      if (pMC->nRuns) /* More points expected */
        ReportError (NULL, RE_INSUF_POINTS | RE_FATAL,
                     pMC->szSetPointsFilename, NULL);

      /* If !nRuns, flag that EOF reached without issuing error */
      goto Exit_ReadSetPoints;

    } /* if */

    rgParms[i] = pMC->rgpMCVar[i]->dVal; /* Copy value to user array */
  } /* for */

  bReturn = TRUE; /* Flag that all parms were read */

  /* Throw away remainder of line. This allows a MC output file to be used
     directly as a setpoints file. */
  do {
    c = getc(pMC->pfileSetPoints);
  } while ((c != '\n') && !feof(pMC->pfileSetPoints));

Exit_ReadSetPoints:
  ;
  return (bReturn);

} /* ReadSetPoints */


/* ----------------------------------------------------------------------------
   GetSPMods

   Reads a new set of modifications from the set points input file.

   Returns TRUE if got modifications OK.

   FALSE is only returned when the number of runs (nRuns) is set to
   zero. In this case the simulation continues until end of file is
   reached, and returns FALSE to flag the eof condition.
*/
BOOL GetSPMods (PANALYSIS panal, double rgParms[])
{
  BOOL bOK;

  bOK = ReadSetPoints (&panal->mc, rgParms);
  /* eventually override by Distrib specs */
  CalcMCParms (&panal->mc, rgParms, panal->mc.nSetParms);
  return bOK;

} /* GetSPMods */


/* ----------------------------------------------------------------------------
   SetParents

   FB 21/07/97: For each Monte Carlo variable, if there are parents referenced
   in hParm for some of the distributional parameters, pdParms are set to point
   to the parent's dVals. If there is no parent, pdParms already point to their
   own dParms and nothing is done.

   The calculation starts at index iStart.
*/
void SetParents (PMONTECARLO pMC, long iStart)
{
  long i, j, k;
  PMCVAR pMCVar1, pMCVar2;
  BOOL bFound;

  for (i = iStart; i < pMC->nParms; i++) { /* for each pMCVar considered */
    pMCVar1 = pMC->rgpMCVar[i];
    for (j = 0; j < 4; j++) { /* for each of its distrib. param */
      if (pMCVar1->iParmType[j] == MCVP_PARM) { 
        /* if there is a parent, find it, but parents must appear before 
           current pMCVar */
        bFound = FALSE;
        for (k = 0; k < i; k++) { 
          pMCVar2 = pMC->rgpMCVar[k];
          if (pMCVar1->hParm[j] == pMCVar2->hvar) {
            /* Point to the parent dVal */
            pMCVar1->pdParm[j] = &(pMCVar2->dVal);
            bFound = TRUE;
          }
        }
        if (!bFound) { /* oops, parent not found, error */
          printf ("\n"
                  "Error: parents must be declared before childrens when\n"
                  "       creating sampling dependencies - Exiting.\n\n");
          exit(0);
        }
      }
    }
  }

} /* SetParents */


/* End */
