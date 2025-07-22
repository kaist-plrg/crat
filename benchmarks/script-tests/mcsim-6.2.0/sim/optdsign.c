/* optdesign.c

   Originally written by Frederic Bois
   16 August 1997

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
*/

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "optdsign.h"
#include "hungtype.h"
#include "lexerr.h"
#include "matutil.h"
#include "mh.h"
#include "simmonte.h"
#include "yourcode.h"

enum {Shannon, Var_Reduction}; /* optimization criterion */


/* ----------------------------------------------------------------------------
   InitOptArrays

   Count Data statements and initialize some arrays.
*/

void InitOptArrays (PANALYSIS panal, int **piDesign_mask, 
                    long *pnDesignPts, double ***pdY, long *pnPreds, 
                    long *pnStartDecisionPts, double **pdVariance,
                    double **pdIR, long nSims)
{
  BOOL bFound;
  int i, j, k;
  OUTSPEC *pos;

  /* Count the total number of predictions requested and the data points
     specified (a prediction+data pair defines a design points */
  *pnPreds = *pnDesignPts = 0;
  for (i = 0; i < panal->expGlobal.iExp; i++) {
    pos = &panal->rgpExps[i]->os;
    bFound = FALSE;
    for (j = 0; j < pos->nOutputs; j++) {
      for (k = 0; k < pos->pcOutputTimes[j]; k++) {

        /* if a Data statement exists that's a design point */
        if (pos->prgdDataVals) {
          (*pnDesignPts)++;
          bFound = TRUE;
        }
        (*pnPreds)++;
      } /* for k */
    } /* for j */
    if (bFound)
      /* Simulation i contained a data statement, decision points start at
         least in the next Simulation */
      *pnStartDecisionPts = *pnPreds;
  } /* for i */

  if (*pnDesignPts == 0) { /* no design points ? */
    printf("Error: you must provide Data Statements ");
    printf("for at least one Simulation to define design points - Exiting.\n");
    exit(0);
  }

  if (*pnPreds == *pnDesignPts) { /* no pure predictions ? */
    printf ("Error: you must provide at least one Simulation ");
    printf ("without Data Statements for utility computations - Exiting.\n");
    exit(0);
  }

  /* allocate the piDesign_mask array: one per design point */
  /* allocate the pdVariance array: one per design point */
  /* allocate the pdIR array: one per iteration */
  /* allocate the pdY matrix */
  if (( !(*piDesign_mask = InitiVector (*pnDesignPts))) ||
      ( !(*pdVariance    = InitdVector (*pnDesignPts))) ||
      ( !(*pdIR          = InitdVector (nSims)))        ||
      ( !(*pdY           = InitdMatrix (nSims, *pnPreds))))
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "InitOptArrays", NULL);

} /* InitOptArrays */


/* ----------------------------------------------------------------------------
   OpenOptFiles

   Opens the output file.
*/

void OpenOptFiles (PANALYSIS panal)
{
  /* take care of the output file first */

  /* use command line spec if given */
  if (panal->bCommandLineSpec)
    panal->gd.szGout = panal->szOutfilename;

  /* use default name if none given */
  else if (!(panal->gd.szGout))
    panal->gd.szGout = "simopt.default.out";

  if (!(panal->gd.pfileOut)
      && !(panal->gd.pfileOut = fopen (panal->gd.szGout, "w")))
    ReportError (NULL, RE_FATAL | RE_CANNOTOPEN,
                 panal->gd.szGout, "[in OpenOptFiles()]");

} /* OpenOptFiles */


/* ----------------------------------------------------------------------------
   WriteOutHeader

   Prints a tabulated header at the top of the output file.
*/

void WriteOutHeader (PANALYSIS panal, int criterion)
{
  int i, j, k;
  OUTSPEC *pos;
  
  fprintf (panal->gd.pfileOut, "iter\t");

  for (i = 0; i < panal->expGlobal.iExp; i++) {
    pos = &panal->rgpExps[i]->os;
    for (j = 0; j < pos->nOutputs; j++) {
      for (k = 0; k < pos->pcOutputTimes[j]; k++) {
        /* if a Data statement exists... */
        if (pos->prgdDataVals) 
          fprintf (panal->gd.pfileOut, "T%g\t", pos->prgdOutputTimes[j][k]);
      }
    }
  }

  fprintf (panal->gd.pfileOut, "Chosen\t");

  if (criterion == Var_Reduction)
    fprintf (panal->gd.pfileOut, "Variance\tSD\tUtility\n");

  fflush (panal->gd.pfileOut);
  
} /* WriteOutHeader */


/* ----------------------------------------------------------------------------
   SetupLikes

   Scan each Monte Carlo variable to find likelihoods and set
   their pdParms to data and output arrays. pdVal and dVal are not set.
*/

void SetupLikes (PANALYSIS panal, long nPreds, PMCVAR **pLikes)
{
  BOOL bFound, bLikeFound;
  long i, j, k, m, n;
  long nPts = 0;
  OUTSPEC *pos;
  PMCVAR pMCVar;
  PMONTECARLO pMC = &panal->mc;

  /* Create an array of pointers to MCVars, one for each design point */
  if (!(*pLikes = (PMCVAR*) malloc (nPreds * sizeof(PMCVAR))))
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "SetupLikes", NULL);

  /* Scan all design points, find the likelihood associated with each one */
  for (i = 0; i < panal->expGlobal.iExp; i++) {

    pos = &panal->rgpExps[i]->os;

    for (j = 0; j < pos->nOutputs; j++) {

      for (k = 0; k < pos->pcOutputTimes[j]; k++) {

        /* Allocate space for an MCVar likelihood specification */
        if (!((*pLikes)[nPts] = (PMCVAR) malloc (sizeof(MCVAR))))
          ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "SetupLikes", NULL);

        /* If a Data statement exists that's a design point */
        if (pos->prgdDataVals) {

          /* Find the likelihood statement associated with the current 
             output variable */
          bLikeFound = FALSE;
          m = pMC->nSetParms;
          while (!bLikeFound) {
            bLikeFound = (pMC->rgpMCVar[m]->hvar == pos->phvar_out[j]);
            if (!bLikeFound) m++;
          }

          if (bLikeFound) { /* Set pointers of MCVar */

            pMCVar = pMC->rgpMCVar[m];

            /* You can use parameters defined in the read-in list in 
               Likelihoods. You cannot yet add Distrib statements (that
               would require more accounting, checking and sampling */
            /* Fetch pointers to the parents in read-in list */
            SetParents (pMC, 0);

            /* Set the data and predictions pdParms of pMCVar */
            for (m = 0; m < 4; m++) {

              if (pMCVar->iParmType[m] == MCVP_PRED) { /* it's an output */
                /* Set pdParm[m] to an output, first find it */
                bFound = FALSE;
                n = 0;
                while ((n < pos->nOutputs) && (!bFound)) {
                  bFound = (pMCVar->hParm[m] == pos->phvar_out[n]);
                  if (!bFound) n++;
                }
                if (bFound) {
                  pMCVar->pdParm[m] = &(pos->prgdOutputVals[n][k]);
                }
                else { /* no corresponding Print statement found: error */
                  printf ("Error: missing Print statement for parameter "
                          "number %ld of %s distribution - Exiting.\n\n",
                          j, pMCVar->pszName);
                  exit (0);
                }
              } /* end if MCVP_PRED */

              else if (pMCVar->iParmType[m] == MCVP_DATA) { /* it's data */
                /* Set pdParm[m] to a data array, first find it */
                bFound = FALSE;
                n = 0;
                while ((n < pos->nData) && (!bFound)) {
                  bFound = (pMCVar->hParm[m] == pos->phvar_dat[n]);
                  if (!bFound) n++;
                }
                if (bFound) {
                  pMCVar->pdParm[m] = &(pos->prgdDataVals[n][k]);
                }
                else { /* no Data found: error */
                  printf ("Error: no Data for %s in Simulation %ld "
                          "- Exiting.\n\n", pMCVar->pszName, i);
                  exit (0);
                }
              } /* end if MCVP_DATA */
            } /* for m */
          } /* end if bLikeFound */
          else { 
            /* No likelihood statement found for the current output: error */
            printf ("Error: missing Likelihood for %s - Exiting.\n\n",
                    pos->pszOutputNames[j]);
            exit (0);
          }

          /* Copy the MCVar created */
          memcpy ((*pLikes)[nPts], pMCVar, sizeof (MCVAR));

        } /* if data */

        else { /* no data: give a null likelihood spec */
          (*pLikes)[nPts] = NULL;
        }

        /* Increment likelihood array index */
        nPts = nPts + 1;

      } /* for k */
    } /* for j */
  } /* for i */
  
} /* SetupLikes */


/* ----------------------------------------------------------------------------
   Estimate_y

   Calculates y[] for the given conditions by running the model.
   y[] is then flattened in the pdY array.

   Note that pdY may not be completely initialized by this routine if it
   is passed uninitialized.
*/

int Estimate_y (PANALYSIS panal, double *pdTheta, double *pdY)
{
  int cNPred = 0;
  int i, j, k;
  OUTSPEC *pos;

  /* Run the PBPK model for each experiment assigned to the subject specified
   */
  for (i = 0; i < panal->expGlobal.iExp; i++) {
    InitModel ();

    /* global modifications */
    ModifyParms (panal->expGlobal.plistParmMods);

    /* set params to pdTheta values */
    SetParms (panal->mc.nSetParms, panal->mc.rghvar, pdTheta);

    /* set the Mods for this exp */
    ModifyParms (panal->rgpExps[i]->plistParmMods);

    if (!DoOneExperiment (panal->rgpExps[i])) {
      /* Error */
      printf ("Warning: Can't estimate y with parameters:\n");
      WriteArray (stdout, panal->mc.nSetParms, pdTheta);
      fputc('\n', stdout);
      return 0;
    } /* if */

    /* link pdY to outputs */
    pos = &panal->rgpExps[i]->os;
    for (j = 0; j < pos->nOutputs; j++) {
      for (k = 0; k < pos->pcOutputTimes[j]; k++) {
        pdY[cNPred] = pos->prgdOutputVals[j][k];
        cNPred++;
      } /* for k */
    } /* for j */

  } /* for i */

  return 1;

} /* Estimate_y */


/* ----------------------------------------------------------------------------
 ReadAndSimulate

   Initialize arrays and reads a parameter sample, obtained for example 
   from a previous MCMC sample. Run the structural model, simulate data 
   and output the loglikelihood of each design point (Outputs having 
   associated Data statements) in pdY. For non-design points of output
   pdY contains just the output value (not its log-likelihood). 

*/

void ReadAndSimulate (PANALYSIS panal, long nSetParms, 
                      double **pdY, long nPreds, PMCVAR *pLikes, long nSims)
{
  register char c;
  long lDummy, iter = 0;
  long j;
  PDOUBLE pdTheta = NULL;
  PDOUBLE pdTheta_0 = NULL;
  PDOUBLE pdData = NULL;
  PDOUBLE pdData_old = NULL;
  FILE *pfileRestart = panal->gd.pfileRestart;

  /* Temporary space allocation for model parameters and simulated data */
  if ( !(pdTheta    = InitdVector (nSetParms)) ||
       !(pdTheta_0  = InitdVector (nSetParms)) ||
       !(pdData_old = InitdVector (nPreds))    ||
       !(pdData     = InitdVector (nPreds)))
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadAndSimulate", NULL);

  /* open the restart file */
  if (!(pfileRestart)
      && !(pfileRestart = fopen (panal->gd.szGrestart, "r")))
    ReportError (NULL, RE_FATAL | RE_CANNOTOPEN,
                 panal->gd.szGrestart, "[in ReadAndSimulate()]");

  /* Skip the first line. This allows a MC output file to be used
     directly as a restart file. */
  do { c = getc(pfileRestart); } while (c != '\n');

  /* Keep reading lines as long as iter < nSims and we have not reached 
     the end of the file.
     Also keep incrementing the global iteration counter, iter:  */
  while ((iter < nSims) && 
         ( !(feof(pfileRestart) ||
           (fscanf(pfileRestart, "%ld", &lDummy) == EOF)))) {

    /* Read model parameters in pdTheta */
    for (j = 0; j < nSetParms; j++) {
      if (!fscanf (pfileRestart, "%lg", &(pdTheta[j])))
        ReportError (NULL, RE_READERROR | RE_FATAL, panal->gd.szGrestart, NULL);
      else
        panal->mc.rgpMCVar[j]->dVal = pdTheta[j];
    }

    /* Throw away remainder of line */
    do { c = getc(pfileRestart); } while (c != '\n');

    /* For all experiments, compute the predictions for the parameter vector */
    Estimate_y (panal, pdTheta, pdY[iter]);

    if (iter == 0) { /* first simulation, just make predictions */
      for (j = 0; j < nSetParms; j++) {
        pdTheta_0[j] = pdTheta[j]; /* save parameter vector for reuse at end */
      }
      for (j = 0; j < nPreds; j++) {
        if (pLikes[j] != NULL) {
          CalculateOneMCParm (pLikes[j]);
          pdData_old[j] = pLikes[j]->dVal;
        }
      }
    }
    else { /* other iterations */
      /* Simulate data */
      for (j = 0; j < nPreds; j++) {
        if (pLikes[j] != NULL) {
          CalculateOneMCParm (pLikes[j]);
          pdData[j] = pLikes[j]->dVal;
        }
      }

      /* Compute the data likelihood with the previous iteration data vector */
      for (j = 0; j < nPreds; j++)
        if (pLikes[j] != NULL) {
          pLikes[j]->dVal = pdData_old[j];
          pdY[iter][j] = LnDensity (pLikes[j], panal);

          /* Update old data vector */
          pdData_old[j] = pdData[j];
        }

      if (iter == nSims - 1) { /* last iteration: process the first one also */
        /* Recompute the predictions to be up to date for all outputs because
           likelihood parameters may point to panal->os internal variables */
        Estimate_y (panal, pdTheta_0, pdY[0]);
        for (j = 0; j < nPreds; j++)
          if (pLikes[j] != NULL) {
            pLikes[j]->dVal = pdData_old[j];
            pdY[0][j] = LnDensity (pLikes[j], panal);
          }        
      }
    } /* end else iter */

    /* Increment iter */
    iter++;

  } /* end while */

  if (iter < nSims) { 
    printf ("\nError: The number of lines in file %s is less than\n",
	    panal->gd.szGrestart);
    printf ("       the number of lines to read (%ld) - Exiting\n", nSims);
    exit (0);
  }

  fclose (pfileRestart);

} /* ReadAndSimulate */


/* ----------------------------------------------------------------------------
   Do_Importance_Ratios

   Compute the expected likelihood of nDesignPts and forms the 
   importance ratios by scaling them to their sum.
   The prediction array MUST contain the log-likelihood of individual data
   points.

   Uses piDesign_mask to select the design points to consider.
*/

void Do_Importance_Ratios (double **pdY, PMCVAR *pLikes, long nSims, 
                           long nPreds, long nDesignPts, int *piDesign_mask,
                           int nDesignPt_tried, double *pdIR)
{
  long i, j, k;
  double dSumL = 0;
  BOOL bOn;

  for (k = 0; k < nSims; k++) {
    pdIR[k] = 0;
    j = 0;
    for (i = 0; i < nPreds; i++) {
      if (pLikes[i] != NULL) {
        if (j == nDesignPt_tried) /* invert the current mask */
          bOn = ! piDesign_mask[j];
        else /* straigth mask */
          bOn = piDesign_mask[j];

        /* Form the log-likelihood for all design experiments */
        if (bOn)
          pdIR[k] = pdIR[k] + pdY[k][i];

        j++;
      } /* if pLikes */
    } /* for i */

    pdIR[k] = exp(pdIR[k]);
    dSumL = dSumL + pdIR[k];

  } /* for k */

  for (k = 0; k < nSims; k++)
    pdIR[k] = pdIR[k] / dSumL;
  
} /* Do_Importance_Ratios */


/* ----------------------------------------------------------------------------
   DoVariance

   Compute the total variance (after log transformation) of elements of a
   matrix, considering importance ratios.
   Only part of the matrix (from istart to ifinish) is used.
*/

double DoVariance (long nDim, double *pdIR, double **pdX,
                   long istart, long ifinish)
{
  long i, j;
  register double ave, ss, dTmp;

  ss = 0;

  /* for all predictions (X between istart and ifinish) */
  for (i = istart; i < ifinish; i++) {
    ave = 0;
    /* Compute the average pdX */
    for (j = 0; j < nDim; j++) {
      ave = ave + pdIR[j] * log(pdX[j][i]);
    }
  
    /* Compute the SS over pdX */
    for (j = 0; j < nDim; j++) {
      dTmp = log(pdX[j][i]) - ave;
      ss = ss + pdIR[j] * dTmp * dTmp;
    }
  }

  return (ss / (double) (ifinish - istart));

} /* DoVariance */


/* ----------------------------------------------------------------------------
   Compute_utility

   Utilities can be computed here: totally arbitrary and unused for now
*/

void Compute_utility (long nDesignPts, int *piDesign_mask, double *dUtility)
{
  int nPts = 0;
  int i;
 
  for (i = 0; i < nDesignPts; i++)
    if (piDesign_mask[i]) nPts++;

  /* Let say that cost is 2 monetary units per design point */
  *dUtility = -2 * nPts;
  
} /* Compute_utility */


/* ----------------------------------------------------------------------------
   WriteOptimOut

   writes to the output file, print only the significant portions.
*/

void WriteOptimOut (PANALYSIS panal, long iter, long nDesignPts, 
                    int criterion, double *pdVariance, int *piDesign_mask, 
                    long iCrit, double dCrit, double dUtility)
{
  long i;
 
  fprintf (panal->gd.pfileOut, "%ld\t", iter);

  if (iCrit < nDesignPts) { /* if iCrit is usefully defined */
    for (i = 0; i < nDesignPts; i++) {
      if ((&panal->mc)->style == forward) {
        if ((i == iCrit) || !(piDesign_mask[i]))
          fprintf (panal->gd.pfileOut, "%g\t", pdVariance[i]);
        else 
          fprintf (panal->gd.pfileOut, "0\t");
      }
      else { /* backward style, the mask needs to be inverted */
        if (!(piDesign_mask[i]))
          fprintf (panal->gd.pfileOut, "0\t");
        else 
          fprintf (panal->gd.pfileOut, "%g\t", pdVariance[i]);
      }
    } /* end for */

    fprintf (panal->gd.pfileOut, "%ld\t", iCrit+1);
  }
  else
    for (i = 0; i <= nDesignPts; i++)
      fprintf (panal->gd.pfileOut, "0\t");

  if (criterion == Var_Reduction)
    fprintf (panal->gd.pfileOut, "%g\t%g\t%g\n", dCrit, sqrt(dCrit), dUtility);

  fflush (panal->gd.pfileOut);
  
} /* WriteOptimOut */


/* ----------------------------------------------------------------------------
   Importance_Resample

   Does just that, updating pIndex1, picking from pIndex0, pdL is destroyed.
*/

void Importance_Resample (long nSims, long *pIndex0, long *pIndex1, 
                          long *plDrawn, double *pdL, double dSumL)
{
  long i, j;

  /* do the weights */
  for (i = 0; i < nSims; i++)
    pdL[i] = pdL[i] / dSumL;

  j = 0;
  do {    
    /* randomly pick a vector among the nSims available. No risk of 
       overflow because Randoms() is between 0 and 1 */
    i = (long) floor (Randoms() * nSims);

    if (Randoms() < pdL[i]) { /* accept */
      pIndex1[j] = pIndex0[i];
      j++;
      plDrawn[pIndex0[i]]++;
    }
  }
  while (j < nSims);

} /* Importance_Resample */


/* ----------------------------------------------------------------------------
   CloseOptFiles

   Closes output files associated with the design optimizer.
   The restart file has already been closed by the ReadAndSimulate

*/

void CloseOptFiles (PANALYSIS panal)
{
  if (panal->gd.pfileOut) {
    fclose (panal->gd.pfileOut);
    printf ("\nWrote results to \"%s\"\n", panal->gd.szGout);
  }

} /* CloseOptFiles */


/* ----------------------------------------------------------------------------
   DoOptimalDesign

   Core routine of the experimental design optimizer
*/

void DoOptimalDesign (PANALYSIS panal)
{
  PGIBBSDATA  pgd = &panal->gd;
  PMONTECARLO pmc = &panal->mc;

  long i, j;
  long iBest = 0;
  long dim;
  long nDesignPts;            /* # of data points in a data set */
  long nPreds;                /* # of predicted values */
  long nSims = pmc->nRuns;    /* # of model parametrizations tested */
  int  *piDesign_mask;        /* current mask for likelihood computation */
  long nDesignPt_tried;       /* index of the currently tried time point */
  long nStartDecisionPts;     /* index to the start of decision points */
  int  criterion;             /* either Shannon's or variance reduction */
  double dBest = 0;           /* best criterion */
  double dUtility = 0;        /* total utility of a design */
  double *pdIR;               /* array of importance ratios (dim: nSims) */
  double *pdVariance;         /* pred. variances. associated to a design pt */
  double **pdY;               /* predictions, then likelihoods */
  PMCVAR *pLikes;             /* array of likelihood specifications */

  /* criterion can be Var_Reduction or Shannon */
  criterion = Var_Reduction;
  if (criterion != Var_Reduction) {
    printf ("Oooops, Shannon not implemented - exiting\n");
    exit (0);
  }

  /* announce the work to be done */
  printf ("\nDoing analysis - Optimal Design %s %s - %d experiment%c\n",
          (pmc->style == forward ? "forward" : "backward"),
          (criterion == Var_Reduction ? "variance reduction" : "Shannon"),
          panal->expGlobal.iExp, (panal->expGlobal.iExp > 1 ? 's' : ' '));

  /* open restart and output files */
  OpenOptFiles (panal);

  /* Initialize the design-points and predictions arrays */
  InitOptArrays (panal, &piDesign_mask, &nDesignPts, &pdY, &nPreds,
                 &nStartDecisionPts, &pdVariance, &pdIR, nSims);

  /* Associate predictions to MCVars likelihood records */
  SetupLikes (panal, nPreds, &pLikes);

  /* Read in the parameter samples and simulate predictions */
  if (!(pgd->szGrestart)) {
    /* error: we must have a starting prior sample */
    printf ("Error: there must be a parameter sample file - Exiting\n");
    exit (0);
  }
  else {
    /* read the starting values in the order they are printed and
       close the file when finished */
    ReadAndSimulate (panal, pmc->nSetParms, pdY, nPreds, pLikes, nSims);
  }

  /* Write the header line of the output file */
  WriteOutHeader (panal, criterion);

  /* Turn experimental points on (backward trials) or off (forward trials) */
  for (i = 0; i < nDesignPts; i++) 
    piDesign_mask[i] = !(pmc->style == forward);

  /* Start the computation */
  if (criterion == Var_Reduction) {

    if (pmc->style == backward) { /* all points included */

      /* Set nDesignPt_tried at a value beyond the array bounds so that it is
         not found and does not interfere in the next call to Likelihood */
      nDesignPt_tried = nDesignPts + 1;

      /* Compute the importance ratios of each parameter set */
      Do_Importance_Ratios (pdY, pLikes, nSims, nPreds, nDesignPts, 
                            piDesign_mask, nDesignPt_tried, pdIR);

      /* Compute the total variance of the pure predictions (decision 
         points) */
      dBest = DoVariance (nSims, pdIR, pdY, nStartDecisionPts, nPreds);
    }
    else { /* forward style is simple */
      for (j = 0; j < nSims; j++)
        pdIR[j] = 1 / (double) nSims;

      dBest = DoVariance (nSims, pdIR, pdY, nStartDecisionPts, nPreds);
    }
  }

  iBest = nDesignPts + 1; /* out of bounds, undefined */

  /* Compute_utility (nDesignPts, piDesign_mask, &dUtility); */

  /* output the first line ... */
  WriteOptimOut (panal, 0, nDesignPts, criterion, pdVariance, piDesign_mask,
                 iBest, dBest, dUtility);

  /* Start the loop over designs points, only some of the previously 
     computed points will be used */
  dim = ((pmc->style == backward) ? nDesignPts - 1 : nDesignPts);
  for (i = 0; i < dim; i++) {

    /* Reset the criterion for "best" */
    if (criterion == Var_Reduction)
      dBest = DBL_MAX;

    for (j = 0; j < nDesignPts; j++) {

      /* Try this point, if it is not already accepted or deleted.
         (style == backward) is 1 in case of backward style and 0 in
         case of forward style. So if we go backward we skip the point
         if it is already "off" */
      if (piDesign_mask[j] == (pmc->style == backward)) { 

        nDesignPt_tried = j; /* try add or remove point j */

        pdVariance[j] = 0; /* reset */

        /* Check whether this point is the best of the j tried so far */
        if (criterion == Var_Reduction) {

          /* Do an importance reweighting to be able to get the variance
             reductions */
          Do_Importance_Ratios (pdY, pLikes, nSims, nPreds, nDesignPts, 
                                piDesign_mask, nDesignPt_tried, pdIR);

          /* Compute the total variance of the decision points */
          pdVariance[j] = DoVariance (nSims, pdIR, pdY, nStartDecisionPts, 
                                      nPreds);

        } /* if */

        /* If forward we want to keep the point giving the best reduction, 
           i.e. the lowest variance; if backward we want to remove the point 
           giving the lowest variance augmentation, which is also the lowest
           variance */
        if (dBest > pdVariance[j]) {
          dBest = pdVariance[j];
          iBest = j;
        }

      } /* if piDesign_mask */

    } /* for j design point */
    
    /* set the best point, by inverting it definitely */
    piDesign_mask[iBest] = !piDesign_mask[iBest];

    /* Compute_utility (nDesignPts, piDesign_mask, &dUtility); */

    /* output ... */
    WriteOptimOut (panal, i+1, nDesignPts, criterion,  pdVariance, 
                   piDesign_mask, iBest, dBest, dUtility);

    printf ("%ld points design done\n", i+1);
     
  } /* for i */

  /* if style is backward we would still like to know what a zero-point design
     would do */
  if (pmc->style == backward) {

    /* Importance ratios are reinitialized with the natural weights */
    for (j = 0; j < nSims; j++) pdIR[j] = 1 / (double) nSims;

    if (criterion == Var_Reduction) {
      dBest = DoVariance (nSims, pdIR, pdY, nStartDecisionPts, nPreds);
      j = 0;
      while (piDesign_mask[j] == 0)
        j++;
      iBest = j;
    }

    /* Output, with null utility */
    WriteOptimOut (panal, nDesignPts, nDesignPts, criterion, pdVariance, 
                   piDesign_mask, iBest, dBest, 0);

    printf ("%ld points design done\n", nDesignPts);
  }

  free (piDesign_mask);

  CloseOptFiles (panal);

} /* DoOptimalDesign */

