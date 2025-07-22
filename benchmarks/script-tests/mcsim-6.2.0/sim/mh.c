/* mh.c

   Written by Frederic Bois

   Copyright (c) 1996-2018 Free Software Foundation, Inc.

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

   This file calls at least one GNU Scientific Library function, if the
   symbol HAVE_LIBGSL is defined in config.h (or in the makefile).
   Otherwise, the corresponding features are disabled (and the program
   exits with an error message).
*/

#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <time.h>
#include <sys/time.h>

#ifdef USEMPI
#include <mpi.h>
#endif

#include "mh.h"
#include "lsodes.h"
#include "lexerr.h"
#include "simmonte.h"

#ifdef HAVE_LIBGSL
#include <gsl/gsl_cdf.h>
#endif


/* Function -------------------------------------------------------------------
   CalculateMeanAndVariance

   Calculate mean and variance of each component of array x, incrementally.
   Index parameter n must be received as 0 at the start of a sequence.
*/
#ifdef NDEF
/* Frederic's version */
void CalculateMeanAndVariance (long *n, double x, double *mean, double *var)
{
  static double K   = 0.0;
  static double Ex  = 0.0;
  static double Ex2 = 0.0;

  double dTmp;

  /* shifted data algorithm */
  if (*n == 0) {
    K = x;
    Ex  = 0.0;
    Ex2 = 0.0;
  }
  else {
    dTmp = x - K;
    Ex  += dTmp;
    Ex2 += dTmp * dTmp;
  }

  (*n)++;

  dTmp = Ex / *n;
  *mean = K + dTmp;

  *var = (Ex2 - Ex * dTmp) / (*n - 1);

#ifdef NDEF
  // or Welford algorithm:

  // for a new value newValue, compute the new count, new mean, the new M2.
  // mean accumulates the mean of the entire dataset
  // M2 aggregates the squared distance from the mean
  // count aggregates the number of samples seen so far
def update(existingAggregate, newValue):
    (count, mean, M2) = existingAggregate
    count = count + 1
    delta = newValue - mean
    mean = mean + delta / count
    delta2 = newValue - mean
    M2 = M2 + delta * delta2

    return (count, mean, M2)

# retrieve the mean, variance and sample variance from an aggregate
def finalize(existingAggregate):
    (count, mean, M2) = existingAggregate
    (mean, variance, sampleVariance) = (mean, M2/count, M2/(count - 1))
    if count < 2:
      return float('nan')
    else:
      return (mean, variance, sampleVariance)
#endif

} /* CalculateMeanAndVariance */
#endif

 /* Cathie's version
 * Variance is calculated using a method described by Knuth
 * m_k = m_k-1 + (x_k - m_k-1)/k
 * v_k = v_k-1 + (x_k - m_k-1)(x_k - m_k)
 * sigma^2 = v_k / (k-1)
 * This portion of the code only calculates the itermediate
 * values - before the variance is sent along the final
 * calculation should be executed (CollectConvInfo)
 */
void CalculateMeanAndVariance (long n, double x, double *xi_bari,
                               double *si_2i) {

  if (n == 1) {
    *xi_bari = x; // mean
    *si_2i = 0;
    return;
  }

  double mTmp = *xi_bari;
  *xi_bari = *xi_bari + (x - *xi_bari)/n;
  *si_2i = *si_2i + (x - mTmp) * (x - *xi_bari);

} /* End CalculateMeanAndVariance */


int checkConvergence (int nOut, int variableCount, int p_count,
                      double **meansForAll, double **varsForAll, double *Rhat)
{
  int vi, pi;
  int converged = 0;
  double varsofvars, meansofmeans, varsofmeans, meansofvars;

  // first calculate the means x_bar = Mean(nChains, xi_bar);
  // calculate the mean of the variances W = Mean (nChains, si_2);
  for (vi = 0; vi < variableCount; vi++) {
    meansofmeans = 0.0;
    for (pi = 0; pi < p_count; pi++) {
      // use the function we already wrote for consistency
      CalculateMeanAndVariance((pi+1),meansForAll[pi][vi],&meansofmeans,
                               &varsofmeans);
      CalculateMeanAndVariance((pi+1),varsForAll[pi][vi],&meansofvars,
                               &varsofvars);
    }

    if ((meansofvars == 0) && (varsofmeans == 0)) {
      *Rhat = 1;
      converged++;
    } else {
      // calculate Vhat and Rhat together (to save memory)
      // s_2 = ((nOut - 1) * W + B) / nOut; // intra-chain variance part 1
      double s2 = ((nOut-1) * meansofvars + varsofmeans) / nOut;
      //  Vhat = s_2 + B / (nOut * dChains); // part 2
      double Vhat = s2 + varsofmeans /(nOut * p_count);
      //  Rhat = Vhat / W; // convergence criterion
      *Rhat = Vhat / meansofvars;
      if (*Rhat < 1.05) // we should parameterize this 1.05 criterion
        converged++;
    }
  }

  return converged;

} /* end checkConvergence */


void CollectConvInfo (PLEVEL plevel, char **args)
{
  double **mean_dest = (double **) args[0];
  double **var_dest  = (double **) args[1];
  long *n = (long *) args[2];
  long  i;
  PMCVAR pMCVar;

  /* For all MC vars at this level */
  for (i = 0; i < plevel->nMCVars; i++) {
    pMCVar = plevel->rgpMCVars[i];
    **mean_dest = pMCVar->dVal_mean;
    /* Keep in mind that the running variance calculation is an
       intermediate value and we want to communicate the variance here
       That means that we need to do the final step before sending
       sigma^2 = v_k / (k-1) */
    **var_dest = pMCVar->dVal_var/(*n-1);
    (*mean_dest)++;
    (*var_dest)++;
  }
} /* end CollectConvInfo */


/* Function -------------------------------------------------------------------
   AnnounceMarkov

   Print out the type of simulations to be performed.
*/
void AnnounceMarkov (int size, int nSimTypeFlag, long nIter)
{

  switch (nSimTypeFlag) {

  case 0:
    printf("\nDoing %ld Metropolis within Gibbs simulation", nIter);
    printf((nIter != 1 ? "s" : ""));
    if (size > 1)
      printf(" on each of %d processors\n", size);
    else
      printf("\n");
    break;

  case 1:
    printf("\nPrinting data and predictions for the last line of the "
           "restart file\n");
    break;

  case 2:
    printf("\nDoing %ld Metropolis-Hastings simulation", nIter);
    printf((nIter != 1 ? "s" : ""));
    if (size > 1)
      printf(" on each of %d processors\n", size);
    else
      printf("\n");
    break;

  case 3:
    printf("\nDoing %ld Metropolis within Gibbs posterior "
           "tempered simulation", nIter);
    printf((nIter != 1 ? "s\n" : "\n"));
    break;

  case 4:
    printf("\nDoing %ld Metropolis within Gibbs likelihood "
           "tempered simulation", nIter);
    printf((nIter != 1 ? "s\n" : "\n"));
    break;

  case 5:
    printf("\nDoing Stochastic optimization\n");
    break;
  }

} /* AnnounceMarkov */


/* Function -------------------------------------------------------------------
   CalculateTotals

   Find total prior, likelihood for all MC vars
   Called from TraverseLevels
*/
void CalculateTotals (PLEVEL plevel, char **args)
{
  PANALYSIS panal = (PANALYSIS)args[0];
  double *pdLnPrior = (double*)args[1];

  long n;

  for (n = 0; n < plevel->nMCVars; n++) {
    *pdLnPrior += LnDensity(plevel->rgpMCVars[n], panal);
  }

} /* CalculateTotals */


/* Function -------------------------------------------------------------------
   CheckForFixed

   It is possible for an MC var to be fixed by a subsequent `=' statement in
   the level just below the level at which it is declared in the input file;
   This routine marks such vars as "Fixed" and set their dVal to the
   prescribed number.
   Called from TraverseLevels
*/

void CheckForFixed (PLEVEL plevel, char **args)
{
  long    n, m;
  PMCVAR  pMCVar;
  PVARMOD pFVar;

  for (n = 0; n < plevel->nMCVars; n++) {
    pMCVar = plevel->rgpMCVars[n];
    for (m = 0; m < plevel->nFixedVars; m++) {
      pFVar = plevel->rgpFixedVars[m];
      if (pMCVar->hvar == pFVar->hvar) {
        pMCVar->bIsFixed = TRUE;
        if (IsInput (pFVar->hvar)) {
          printf("Error: a sampled parameter cannot be assigned an input\n");
          exit(0);
        }
        else
          pMCVar->dVal = pFVar->uvar.dVal;
      }
    }
  }

} /* CheckForFixed */


/* Function -------------------------------------------------------------------
   CheckPrintStatements

   Tries to check that 'Print' and 'Data' statements are consistent.
   Note that experiments must have outputs; this is checked in
   PrepareOutSpec in siminit.c.
   Called from TraverseLevels
*/

void CheckPrintStatements (PLEVEL plevel, char **args)
{
  PANALYSIS panal = (PANALYSIS) args[0];
  POUTSPEC  pos;
  /* PMCVAR    pMCVar;
  BOOL      bFound, bOK; */
  long      i, j /* , k, l, dCount, dIndex */;

  if (plevel->pexpt == NULL)
    return;

  pos = &(plevel->pexpt->os);

  /* check that the same var does not appear in 2 or more print statements */
  for (j = 0; j < pos->nOutputs; j++)
    for (i = j+1; i < pos->nOutputs; i++)
      if (pos->phvar_out[j] == pos->phvar_out[i])
        ReportRunTimeError (panal, RE_DUPVARINEXPRT | RE_FATAL,
                            pos->pszOutputNames[i], "Print");

  /* check that the same var does not appear in 2 or more data statements */
  for (j = 0; j < pos->nData; j++)
    for (i = j+1; i < pos->nData; i++)
      if (pos->phvar_dat[j] == pos->phvar_dat[i])
        ReportRunTimeError (panal, RE_DUPVARINEXPRT | RE_FATAL,
                            pos->pszDataNames[i], "Data");

  /* check that print and matching data lists are of equal length */
  for (j = 0; j < pos->nOutputs; j++)
    for (i = 0; i < pos->nData; i++)
      if ((pos->phvar_out[j] == pos->phvar_dat[i]) &&
          (pos->pcOutputTimes[j] != pos->pcData[i])) {
        printf("\nError: unequal times in Print and Data statements for %s\n"
               "Exiting.\n\n", pos->pszOutputNames[j]);
        exit(0);
      }

} /* CheckPrintStatements */


/* Function -------------------------------------------------------------------
   CheckAllTransitions

   Check whether all perk transitions rates are high enough.
*/
BOOL CheckAllTransitions (PGIBBSDATA pgd)
{
  BOOL   bOK;
  double AcceptRate;
  int    i;

  i = pgd->startT;
  bOK = TRUE;
  while ((i <= pgd->endT - 1) && bOK) {
    if (pgd->rglTransAttempts[i] < 10) {
      bOK = FALSE;
      break;
    }
    else
      AcceptRate = pgd->rglTransAccepts[i] /
                   ((double) pgd->rglTransAttempts[i]);

    bOK = (AcceptRate > 0.15);
    i++;
  }

  return bOK;

} /* end CheckAllTransitions */


/* Function -------------------------------------------------------------------
   CheckTransitions

   Check whether the first perk transition rate is between reasonable bounds
   and that enough attemps have been made
   Return -1 if too low or too few attempts, 0 if OK, +1 if too high
*/
int CheckTransitions (PGIBBSDATA pgd)
{
  double AcceptRate;
  int    i;

  i = pgd->startT;
  if (pgd->rglTransAttempts[i] < 10)
    return (-1);   /* too low */
  else
    AcceptRate = pgd->rglTransAccepts[i] /
                 ((double) pgd->rglTransAttempts[i]);

  if (AcceptRate < 0.30) {
    return (-1);   /* too low */
  }
  else {
    if (AcceptRate < 1) {
      return (0);  /* OK */
    }
    else {
      return (+1); /* too high */
    }
  }

} /* end CheckTransitions */


/* Function -------------------------------------------------------------------
   CloneLikes

   Called from TraverseLevels
   First copy the likelihoods in the current plistLikes in the rgpLikes
   down to the next lower levels. Then eventually override them by definitions
   in rgpLikes at this level.
*/
void CloneLikes (PLEVEL plevel, char **args)
{
  long nLikes;
  long i, j, k;
  PLEVEL pLower;
  PMCVAR pClone;
  PMCVAR pMCVar;
  BOOL bFound;

  for (i = 0; i < plevel->iInstances; i++) {
    pLower = plevel->pLevels[i];
    /* The number of likelihoods is overestimated by this, but that will do
       for now: */
    pLower->nLikes = nLikes = plevel->nLikes + ListLength(plevel->plistLikes);

    if (pLower->nLikes != 0) {
      if (!(pLower->rgpLikes =
             (PMCVAR*) malloc (pLower->nLikes * sizeof(PMCVAR))))
        ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "CloneLikes", NULL);
    }
  }

  /* Copy the likelihoods in the plistLikes at this level to the
     rgpLikes of the lower levels */
  nLikes = 0;
  ForAllList3 (plevel->plistLikes, &CloneLikesL, plevel, &nLikes, NULL);

  /* Note: nLikes now equals the number of likelihoods in the current
     plistLikes */

  /* Copy the likelihoods in rgpLikes at this level down to the lower
     levels, if they are not already defined */
  for (i = 0; i < plevel->iInstances; i++) {
    pLower = plevel->pLevels[i];

    for (j = 0; j < plevel->nLikes; j++) {
      pMCVar = plevel->rgpLikes[j];

      /* if that likelihood is already in pLower->rgpLikes forget it */
      bFound = FALSE;
      k = 0;
      while ((k < nLikes) && (!bFound)) {
        bFound = (pMCVar->hvar == pLower->rgpLikes[k]->hvar);
        if (!bFound) k++;
      }
      if (!bFound) {
        if (!(pClone = (PMCVAR) malloc (sizeof (MCVAR))))
          ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "CloneLikes", NULL);

        memcpy (pClone, pMCVar, sizeof (MCVAR));
        pLower->rgpLikes[nLikes+j] = pClone;
      }
    }
  }

} /* CloneLikes */


/* Function -------------------------------------------------------------------
   CloneLikesL

   Clone and copy the members of a plistLikes from a level to the rgpLikes
   arrays of the levels below
   Called from ForAllList3
*/
void CloneLikesL (PVOID pData, PVOID pUser1, PVOID pUser2, PVOID pUser3)
{
  PMCVAR pMCVar = (PMCVAR) pData;
  PLEVEL plevel = (PLEVEL) pUser1;
  long   *pnLikes = (long *) pUser2;
  long   i;
  PLEVEL pLower;
  PMCVAR pClone;

  ++pMCVar->iDepth;
  for (i = 0; i < plevel->iInstances; i++) {
    pLower = plevel->pLevels[i];
    if (!(pClone = (PMCVAR) malloc (sizeof (MCVAR))))
      ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "CloneLikeL", NULL);
    memcpy (pClone, pMCVar, sizeof (MCVAR));
    pLower->rgpLikes[*pnLikes] = pClone;
  }
  ++(*pnLikes);

} /* CloneLikesL */


/* Function -------------------------------------------------------------------
   CloneMCVars

   Called from TraverseLevels
   For all MC vars in list at given level, add to arrays of all instances of
   next (lower) level; the instances are created by the cloning routine
   CloneMCVarsL
*/
void CloneMCVars (PLEVEL plevel, char **args)
{
  long nMCVars = ListLength(plevel->plistMCVars);
  long n;
  PLEVEL pLower;

  for (n = 0; n < plevel->iInstances; n++) {
    pLower = plevel->pLevels[n];
    pLower->nMCVars = nMCVars;
    if (nMCVars != 0) {
      if (!(pLower->rgpMCVars = (PMCVAR*) malloc (nMCVars * sizeof(PMCVAR))))
        ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "CloneMCVars", NULL);
    }
  }

  nMCVars = 0;
  ForAllList3 (plevel->plistMCVars, &CloneMCVarsL, plevel, &nMCVars, NULL);

} /* CloneMCVars */


/* Function -------------------------------------------------------------------
   CloneMCVarsL

   Clone and copy the members of a plistMCVars
   Called from ForAllList3
*/
void CloneMCVarsL (PVOID pData, PVOID pUser1, PVOID pUser2, PVOID pUser3)
{
  PMCVAR pMCVar = (PMCVAR) pData;
  PLEVEL plevel = (PLEVEL) pUser1;
  long   *pnMCVars = (long *) pUser2;
  long   i;
  PLEVEL pLower;
  PMCVAR pClone;

  ++pMCVar->iDepth;
  for (i = 0; i < plevel->iInstances; i++) {
    pLower = plevel->pLevels[i];
    if (!(pClone = (PMCVAR) malloc(sizeof (MCVAR))))
      ReportError(NULL, RE_OUTOFMEM | RE_FATAL, "CloneMCVarsL", NULL);
    memcpy (pClone, pMCVar, sizeof (MCVAR));
    pClone->plistDependents = InitList();
    pLower->rgpMCVars[*pnMCVars] = pClone;
  }
  ++(*pnMCVars);

} /* CloneMCVarsL */


/* Function -------------------------------------------------------------------
   CloseMarkovFiles

   Closes output files associated with the Markov sampler.
   The restart file has already been closed by the ReadRestart

*/
void CloseMarkovFiles (PGIBBSDATA pgd)
{
  /* If tempered MCMC, close perk scale recording file */
  if ((pgd->nSimTypeFlag == 3) || (pgd->nSimTypeFlag == 4)) {
    char szFileName[MAX_FILENAMESIZE+6];
    sprintf(szFileName, "%s%s", pgd->szGout, ".perks");
    fclose(pgd->pfilePerks);
    printf("\nWrote perks to \"%s\"\n", szFileName);
  }

  if (pgd->pfileOut) {
    fclose(pgd->pfileOut);
    printf("Wrote MCMC sample to \"%s\"\n", pgd->szGout);
  }

  printf("\n");

} /* CloseMarkovFiles */


/* Function -------------------------------------------------------------------
   ConvertLists

   Converts lists to arrays
   Called from TraverseLevels
*/
void ConvertLists(PLEVEL plevel, char **args)
{
  PANALYSIS panal = (PANALYSIS)args[0];
  long m, n;
  PMCVAR pMCVar;

  if (plevel->pexpt == NULL)
    ListToPVArray (panal, plevel->plistVars, &plevel->nFixedVars,
                   &plevel->rgpFixedVars);
  else
    ListToPVArray (panal, plevel->pexpt->plistParmMods, &plevel->nFixedVars,
                   &plevel->rgpFixedVars);

  for (n = 0; n < plevel->nMCVars; n++) {
    pMCVar = plevel->rgpMCVars[n];
    ListToPMCArray (panal, pMCVar->plistDependents,
                    &pMCVar->nDependents, &pMCVar->rgpDependents);

    /* if there are no dependent, experiments are most likely to be dependent
       on that parameter (complete checking would require  verifying the
       model file - FB 2 May 1998 */
    if (pMCVar->nDependents == 0)
      pMCVar->bExptIsDep = TRUE;
    else {
      /* if there are dependent, experiments may be dependent on that
         parameter if they are not shielded by an intermediate instance of
         the same parameter */
      m = 0;
      while ((m < pMCVar->nDependents) &&
             !(pMCVar->bExptIsDep = (strcmp(pMCVar->rgpDependents[m]->pszName,
                                            pMCVar->pszName) ? TRUE : FALSE)))
        m++;
    }
  }

} /* ConvertLists */


/* Function -------------------------------------------------------------------
   DoMarkov

   Core routine of the MCMC sampler
*/
void DoMarkov (PANALYSIS panal)
{
#ifdef USEMPI
  //double     startMarkov = time(NULL);
#endif
  PGIBBSDATA pgd = &panal->gd;
  PLEVEL     pLevel0 = panal->pLevels[0];
  long       nThetas, nUpdateAt, nTotal;
  long       i, iter = 0;
  long       nIter = pgd->nMaxIter;  /* scheduled iterations of the sampler */
  double     *pdMCVarVals  = NULL;   /* read in values of thetas */
  double     *pdSum        = NULL;   /* column sums of read thetas */
  double     **prgdSumProd = NULL;   /* column sums of thetas cross-products */
  double     dTmp, dLnPrior = 0, dLnData = 0;

  if (panal->rank == 0)
    AnnounceMarkov(panal->size, pgd->nSimTypeFlag, nIter);

  OpenMarkovFiles(panal);

  ReadDataFile(panal); /* (if needed) */

  /* MC variables must be placed in arrays at the next lower level */
  TraverseLevels(pLevel0, CloneMCVars, NULL);

  /* likelihoods must percolate down to the experiment level */
  TraverseLevels(pLevel0, CloneLikes, NULL);

  /* find the parents and dependents of the MC vars */
  TraverseLevels(pLevel0, FindMCParents,    panal, NULL);
  TraverseLevels(pLevel0, FindMCDependents, panal, NULL);

  /* find the parents of the parameters in likelihood statements */
  TraverseLevels(pLevel0, FindLikeParents, panal, NULL);

  /* convert the rest of the lists to arrays */
  TraverseLevels(pLevel0, ConvertLists, panal, NULL);

  /* check for MC vars that have been fixed */
  TraverseLevels(pLevel0, CheckForFixed, NULL);

  /* check variables in statements of type
     'Distrib(<var1>, <distrib>, Prediction, <var2>)'
     for identical 'Print' statements */
  TraverseLevels(pLevel0, CheckPrintStatements, panal, NULL);

  /* if required, print out the structure for debugging */
  if ((panal->rank == 0) && (panal->bDependents)) {
    printf("Hierarchical structure:\n\n");
    TraverseLevels(pLevel0, PrintDeps, NULL);
    printf("\nDone.\n\n");
    return;
  }

  /* change the MC vars hvar pointers from pointing to model parameters to
     pointing to the parents' dVal */
  TraverseLevels(pLevel0, SetPointers, panal, NULL);

  /* get the initial values of the MC vars by reading the restart file if
     one is given */
  if (pgd->szGrestart) {

    /* count the variables to read */
    nThetas = 0;
    TraverseLevels(pLevel0, GetNumberOfMCVars, &nThetas);

    /* read the starting values in the order they are printed and
       close the file when finished */
    if (((pgd->nSimTypeFlag == 3) || (pgd->nSimTypeFlag == 4)) &&
        (pgd->nPerks != 0)) {
      /* if the user has required tempering and specified a perk scale,
         then read also the pseudo-priors */
      ReadRestartTemper(pgd->pfileRestart, nThetas, pgd->nPerks,
                        &pdMCVarVals, &pdSum, &prgdSumProd, &iter,
                        &pgd->indexT, pgd->rgdlnPi);
    }
    else
      ReadRestart(pgd->pfileRestart, nThetas, &pdMCVarVals, &pdSum,
                  &prgdSumProd, &iter);

    /* set the dVals of the variables to the values read in */
    nThetas = 0; /* variables will be recounted */
    if (!TraverseLevels1 (pLevel0, SetMCVars, pdMCVarVals, &nThetas, NULL)) {
      printf("\nError: some read-in parameters are out of bounds - "
             "Exiting\n\n");
      exit(0);
    }

    /* If nSimTypeFlag is 1 print just the predictions and data and exit */
    if (pgd->nSimTypeFlag == 1) {
      if (panal->rank == 0) { /* do it on only one processor */
        PrintAllExpts(pLevel0, panal, pgd->pfileOut);
        CloseMarkovFiles (pgd);
      }
      return;
    }

    /* If component jumps, tempered or stochastic optimization: read eventually
       the kernel file */
    if ((pgd->nSimTypeFlag == 0) || (pgd->nSimTypeFlag >= 3)) {

      char szKernelFile[MAX_FILENAMESIZE+12];
      /* prefix the filename with the rank of the process if more than one
         process is used, postfix it with .kernel in any case */
      if (panal->size > 1) {
        sprintf(szKernelFile, "%04d_%s%s", panal->rank, panal->gd.szGrestart,
                ".kernel");
      }
      else {
        sprintf(szKernelFile, "%s%s", panal->gd.szGrestart, ".kernel");
      }

      FILE *pfile = fopen(szKernelFile, "r");

      if (pfile) {
        /* Read kernel sizes from file */
        printf("Reading kernel file %s\n", szKernelFile);
        TraverseLevels(pLevel0, ReadKernel, pfile, NULL);
      }
      else {
        /* Set the jumping kernel's SD automatically */
        TraverseLevels(pLevel0, SetKernel, 1, pdMCVarVals, NULL);

        /* Free the now useless array */
        free(pdMCVarVals);
      }
    }
    else { /* vector jumping, initialize prior */
      TraverseLevels(pLevel0, CalculateTotals, panal, &dLnPrior, NULL);
    }

    /* Initialize the predictions arrays by running all experiments */
    if (!RunAllExpts (panal, &dLnData)) {
      /* error, cannot compute */
      printf("\nError: cannot compute at the starting point - Exiting\n\n");
      exit(0);
    }

    /* Save the data likelihoods */
    TraverseLevels1 (pLevel0, SaveLikelihoods, NULL);

    /* If tempered MCMC and no user-specified perks, set the perk
       scale and the pseudo-priors automatically */
    InitPerks(panal);

    /* Now that we have the MC vars etc. right, write the output file header */
    WriteHeader(panal);

  } /* end if restart file */

  else { /* no restart file, init by sampling */

    /* If nSimTypeFlag is 1 print an error message and exit */
    if (pgd->nSimTypeFlag == 1) {
      printf("\nError: a restart file must be given to print data and"
             "         predictions - Exiting.\n\n");
      exit(0);
    }

    /* Set the jumping kernel's SD */
    TraverseLevels(pLevel0, SetKernel, 2, pdMCVarVals, NULL);

    /* initialize the thetas by sampling from the prior */
    TraverseLevels(pLevel0, InitMCVars, NULL);

#ifdef USEMPI
    /* count the variables if we monitor their convergence */
    nThetas = 0;
    TraverseLevels(pLevel0, GetNumberOfMCVars, &nThetas);
#endif

    /* Calculate the total prior */
    TraverseLevels(pLevel0, CalculateTotals, panal, &dLnPrior, NULL);

    /* Initialize the predictions arrays by running all experiments */
    if (!RunAllExpts (panal, &dLnData)) {
      /* Error, cannot compute */
      printf("\nError: cannot compute at the starting point - Exiting\n\n");
      exit(0);
    }

    /* Save the data likelihoods */
    TraverseLevels1(pLevel0, SaveLikelihoods, NULL);

    /* If tempered MCMC and no user-specified perks, set the perk
       scale and the pseudo-priors automatically */
    InitPerks(panal);

    /* Now that we have the MC vars etc. right, write the output file header */
    WriteHeader(panal);

    /* Write the current thetas to the output file */
    fprintf(pgd->pfileOut, "0\t");
    TraverseLevels(pLevel0, WriteMCVars, pgd->pfileOut, NULL);

    /* Output indexT and pseudo-priors if needed */
    if ((pgd->nSimTypeFlag == 3) || (pgd->nSimTypeFlag == 4)) {
      fprintf(pgd->pfileOut, "%d\t", pgd->indexT);
      for (i = 0; i < pgd->nPerks; i++)
        fprintf(pgd->pfileOut, "%e\t", pgd->rgdlnPi[i]);
    }

    /* Output prior and likelihood */
    fprintf(pgd->pfileOut, "%e\t%e\t%e\n", dLnPrior, dLnData,
            dLnPrior + dLnData);
    fflush(pgd->pfileOut);

  } /* end no restart file */

  /* Initializations are finished, let's do the iterations */
  nTotal = UPDATE_BASE;
  nUpdateAt = iter + nTotal; /* kernel will be updated at that iteration */

#ifdef USEMPI
  double **meansForAll;
  double **varsForAll;
  double *means;
  double *vars;
  double Rhat;
  if (panal->bPrintConvergence) {
    /* when running in parallel we need to pack the running mean and
       convergence values for each variable into an array that will be
       sent to the root rank; this code allocates the space for those values;
       we also have to prep the 0 rank processor to calculate the variance
       of variances and mean of means for convergence */
    if (panal->rank == 0) {
      meansForAll = (double**) malloc(sizeof(double*) * panal->size);
      varsForAll  = (double**) malloc(sizeof(double*) * panal->size);
      int sender;
      for (sender = 0; sender < panal->size; sender++){
        meansForAll[sender] = (double*) malloc(sizeof(double) * nThetas);
        varsForAll[sender]  = (double*) malloc(sizeof(double) * nThetas);
      }
      means = meansForAll[0];
      vars  = varsForAll[0];
    } else {
      means = (double*) malloc(sizeof(double) * nThetas);
      vars  = (double*) malloc(sizeof(double) * nThetas);
    }
  }
  //fprintf(stderr, "starting %ld iterations\n", nIter);
  //double startIter = time(NULL);
  //fprintf(stderr, "Time to warm up %lf\n", startIter - startMarkov);

  MPI_Barrier(MPI_COMM_WORLD);
#endif

  while (iter < nIter) {

#ifdef USEMPI
    if (panal->bPrintConvergence) { /* convergence check */
      if ((iter+1) % pgd->nPrintFreq == 0) {
        double *tempMeans, *tempVars;
        tempMeans = means;
        tempVars  = vars;
        /* all processors must collect their info into an array */
        TraverseLevels(pLevel0, CollectConvInfo, &tempMeans, &tempVars, &iter);
        /* the root process needs to collect everyone's data and process it */
        if (panal->rank == 0 ) {
          int sender = 0;
          MPI_Status status;
          for (sender = 1; sender < panal->size; sender++) {
            MPI_Recv(meansForAll[sender], nThetas, MPI_DOUBLE, sender,
                     0, MPI_COMM_WORLD, &status);
          }
          int converged = checkConvergence(iter+1, nThetas, panal->size,
                                           meansForAll, varsForAll, &Rhat);
          if (iter > 10) /* avoid pathologies at start */
            fprintf(stderr, "Iteration %ld,\tRhat %g,\tconverged %d\n",
                   iter + 1, Rhat, converged);
          if (iter == nIter - 1)
            printf("\n");
          /* everyone else just sends their data */
        } else {
          MPI_Send(means, nThetas, MPI_DOUBLE, 0, 0, MPI_COMM_WORLD);
        }
      }
    }
#endif

    /* Output to screen, if required as a command-line option */
    if (panal->bOutputIter && ((iter+1) % panal->nOutputFreq == 0)) {
      if (panal->size > 1)
        printf("Processor %d, Iteration %ld\n", panal->rank, iter + 1);
      else
        printf("Iteration %ld\n", iter + 1);
      if (iter == nIter - 1)
        printf("\n");
    }

    /* Start output to file, eventually */
    if (((iter + 1) % pgd->nPrintFreq == 0) &&
        (iter >= pgd->nMaxIter - pgd->nPrintIter))
      fprintf(pgd->pfileOut, "%ld\t", iter + 1);

    /* take this path for 0 and 5 */
    if ((pgd->nSimTypeFlag == 0) || (pgd->nSimTypeFlag == 5)) {
      /* component by component jumps or stochastic optimization */

      TraverseLevels(pLevel0, SampleThetas, panal, pgd, &iter, &nUpdateAt,
                     &nTotal, NULL);

      /* Output log-densities, eventually */
      if (((iter + 1) % pgd->nPrintFreq == 0) &&
          (iter >= pgd->nMaxIter - pgd->nPrintIter)) {
        dLnPrior = 0.0;
        TraverseLevels(pLevel0, CalculateTotals, panal, &dLnPrior, NULL);
        dLnData = 0.0;
        TraverseLevels1(pLevel0, SumAllExpts, &dLnData, NULL);
        fprintf(pgd->pfileOut, "%e\t%e\t%e\n", dLnPrior, dLnData,
                dLnPrior + dLnData);
        fflush (pgd->pfileOut);
      }
    }
    else {
      /* this is the path for 3 and 4 */
      if ((pgd->nSimTypeFlag == 3) || (pgd->nSimTypeFlag == 4)) {
        /* tempered */
        TraverseLevels(pLevel0, SampleThetasTempered, panal, pgd, &iter,
                       &nUpdateAt,  &nTotal, &pgd->indexT, NULL);

        dLnPrior = 0.0;
        TraverseLevels(pLevel0, CalculateTotals, panal, &dLnPrior, NULL);
        dLnData = 0.0;
        TraverseLevels1(pLevel0, SumAllExpts, &dLnData, NULL);

        /* Robbins-Monro updating of the pseudo prior */
        dTmp = pgd->dCZero / (iter + pgd->dNZero);
        for (i = 0; i < pgd->nPerks; i++) {
          if (i == pgd->indexT)
            pgd->rgdlnPi[i] -= dTmp;
          else
            pgd->rgdlnPi[i] += dTmp / pgd->nPerks;
        }

        /* Output indexT and log-densities, eventually */
        if (((iter + 1) % pgd->nPrintFreq == 0) &&
            (iter >= pgd->nMaxIter - pgd->nPrintIter)) {
          fprintf(pgd->pfileOut, "%d\t", pgd->indexT);
          for (i = 0; i < pgd->nPerks; i++)
            fprintf(pgd->pfileOut, "%e\t", pgd->rgdlnPi[i]);
          fprintf(pgd->pfileOut, "%e\t%e\t%e\n", dLnPrior, dLnData,
                  dLnPrior + dLnData);
          fflush(pgd->pfileOut);
        }

        /* update population count of current temperature */
        pgd->rglCount[pgd->indexT] = pgd->rglCount[pgd->indexT]+1;

        /* test the temperature and change indexT if necessary */
        pgd->indexT = SampleTemperature2 (pgd, dLnPrior, dLnData);

      } /* end tempered */
      else { /* vector jumps (option 2) */

        SampleThetaVector(pLevel0, panal, nThetas, pdMCVarVals, pdSum,
                          prgdSumProd, iter, nUpdateAt, nTotal, &dLnPrior,
                          &dLnData);

        /* Output, eventually */
        if (((iter + 1) % pgd->nPrintFreq == 0) &&
            (iter >= pgd->nMaxIter - pgd->nPrintIter)) {

          for (i = 0; i < nThetas; i++)
            fprintf(pgd->pfileOut, "%5g\t", pdMCVarVals[i]);

          fprintf(pgd->pfileOut, "%e\t%e\t%e\n", dLnPrior, dLnData,
                   dLnPrior + dLnData);
          fflush(pgd->pfileOut);
        }
      }
    }

    /* Adjust the update time, eventually */
    if (iter == nUpdateAt) {
      nTotal = (nTotal * 3) / 2;
      nUpdateAt = iter + nTotal;
    }

    /* Increment the iteration counter */
    iter = iter + 1;

  } /* while iter */

#ifdef USEMPI
  //double stopIter = time(NULL);
  //fprintf(stderr,"Time Iterating: %lf\n", (stopIter-startIter));
#endif

  /* If tempered MCMC: print perk scale */
  if ((pgd->nSimTypeFlag == 3) || (pgd->nSimTypeFlag == 4)) {
    /* PrintTemperatureDiagnostics (stdout, pgd); */
    PrintTemperatureDiagnostics(pgd->pfilePerks, pgd);
  }

  /* If component jumps, tempered or stochastic optimization: save kernel */
  if ((pgd->nSimTypeFlag == 0) || (pgd->nSimTypeFlag >= 3)) {

    char szKernelFile[MAX_FILENAMESIZE+7];
    sprintf(szKernelFile, "%s%s", panal->gd.szGout, ".kernel");

    FILE *pfile = fopen (szKernelFile, "w");
    if (!pfile) {
      printf("Cannot create kernel saving file '%s'\n", panal->gd.szGdata);
      exit(0);
    }

    /* Write out the jumping kernel SDs */
    TraverseLevels (pLevel0, WriteKernel, pfile, NULL);

    fprintf(pfile, "\n");
    fclose(pfile);
    printf("Wrote kernel SDs to \"%s\"\n", szKernelFile);
  }

  CloseMarkovFiles(pgd);

#ifdef USEMPI
  //double stopMarkov = time(NULL);
  //fprintf(stderr,"Time Markov: %lf\n", (stopMarkov - startMarkov));
#endif

} /* DoMarkov */


/* Function -------------------------------------------------------------------
   EqualSlopes

   Check whether three points in a row are aligned (within a tolerance range).
   Return TRUE if they do, FALSE otherwise.
   The relative difference is tested in fact.
*/
BOOL EqualSlopes (PDOUBLE x, PDOUBLE y, int i)
{
  #define SL_EPSILON 0.01
  double s1, s2;

  s1 = (y[i+1] - y[i]) / (x[i+1] - x[i]);

  s2 = (y[i+2] - y[i]) / (x[i+2] - x[i]);

  return (fabs(s2/s1 - 1) < SL_EPSILON);

} /* EqualSlopes */


/* Function -------------------------------------------------------------------
   Extrapolate

   Extrapolate linearly to dTargetX from a pair of
   (perk, pseudo-prior) points.
*/
double Extrapolate (PGIBBSDATA pgd, double dTargetX, int i1, int i2)
{
  return (pgd->rgdlnPi[i1] -
          (pgd->rgdPerks[i1] - dTargetX) *
          (pgd->rgdlnPi [i2] - pgd->rgdlnPi [i1]) /
          (pgd->rgdPerks[i2] - pgd->rgdPerks[i1]));

} /* Extrapolate */


/* Function -------------------------------------------------------------------
   FindLikeParents

   Called from TraverseLevels
   Find the parents of the MC vars corresponding to likelihood at this level
   by looking at sampled parameters at this and all previous levels. Note that
   dependencies are not checked among likelihoods.
*/
void FindLikeParents (PLEVEL plevel, char **args)
{
  PANALYSIS panal = (PANALYSIS)args[0];
  long      k, l, m, n;
  PLEVEL    pPrevLev;
  PMCVAR    pMCVar1, pMCVar2;
  BOOL      bFound;

  /* Set the current level array as we pass through */
  panal->pCurrentLevel[plevel->iDepth] = plevel;

  for (k = 0; k < plevel->nLikes; k++) {
    pMCVar1 = plevel->rgpLikes[k]; /* current likelihood */

    for (l = 0; l < 4; l++) {
      if (pMCVar1->iParmType[l] == MCVP_PARM) {
        /* if there is a parent, find it, but parents must appear before
           current pMCVar */
        bFound = FALSE;

        /* First, this level */
        for (m = 0; m < plevel->nMCVars; m++) { /* for all previous distrib */
          pMCVar2 = plevel->rgpMCVars[m];
          if (pMCVar1->hParm[l] == pMCVar2->hvar) {
            pMCVar1->pMCVParent[l] = pMCVar2;
            bFound = TRUE;
          }
        } /* for m */

        /* Now, all previous levels */
        if (!bFound) {
          for (n = plevel->iDepth - 1; n >= 0; n--) {
            pPrevLev = panal->pCurrentLevel[n];

            for (m = 0; m < pPrevLev->nMCVars; m++) {
              pMCVar2 = pPrevLev->rgpMCVars[m];
              if (pMCVar1->hParm[l] == pMCVar2->hvar) {
                pMCVar1->pMCVParent[l] = pMCVar2;
                bFound = TRUE;
              }
            } /* for m */
          } /* for n */
        }

        if (!bFound) { /* oops, parent not found, error */
          printf("\n"
                 "Error: parent in position %ld of %s must be\n"
                 "       declared before it when creating\n"
                 "       sampling dependencies - Exiting.\n\n",
                 l, pMCVar1->pszName);
          exit(0);
        }

      }
    } /* for l */
  } /* for k */

} /* FindLikeParents */


/* Function -------------------------------------------------------------------
   FindMCDependents

   Called from TraverseLevels
   Find the direct dependents of the MC vars at this level by looking at this
   and all lower levels
*/
void FindMCDependents (PLEVEL plevel, char **args)
{
  long   i, j;
  PMCVAR pMCVar;

  for (i = 0; i < plevel->nMCVars; i++) {
    pMCVar = plevel->rgpMCVars[i];
    for (j = 0; j < 4; j++)
      if ((pMCVar->pMCVParent[j] != NULL) &&
          (pMCVar->pMCVParent[j]->hvar == pMCVar->hParm[j]))
        QueueListItem(pMCVar->pMCVParent[j]->plistDependents, pMCVar);
  }

} /*FindMCDependents */


/* Function -------------------------------------------------------------------
   FindMCParents

   Called from TraverseLevels
   Find the parents of the MC vars at this level by looking at this and
   all previous levels.
*/
void FindMCParents (PLEVEL plevel, char **args)
{
  PANALYSIS panal = (PANALYSIS)args[0];
  long      k, l, m, n;
  PLEVEL    pPrevLev;
  PMCVAR    pMCVar1, pMCVar2;
  BOOL      bFound;

  /* Set the current level array as we pass through */
  panal->pCurrentLevel[plevel->iDepth] = plevel;

  for (k = 0; k < plevel->nMCVars; k++) {
    pMCVar1 = plevel->rgpMCVars[k]; /* current distrib */

    for (l = 0; l < 4; l++) {
      if (pMCVar1->iParmType[l] == MCVP_PARM) {
        /* if there is a parent, find it, but parents must appear before
           current pMCVar */
        bFound = FALSE;

        /* First, this level */
        for (m = 0; m < k; m++) { /* for all previous distrib */
          pMCVar2 = plevel->rgpMCVars[m];
          if (pMCVar1->hParm[l] == pMCVar2->hvar) {
            pMCVar1->pMCVParent[l] = pMCVar2;
            bFound = TRUE;
          }
        } /* for m */

        /* Now, all previous levels */
        if (!bFound) {
          for (n = plevel->iDepth - 1; n >= 0; n--) {
            pPrevLev = panal->pCurrentLevel[n];

            for (m = 0; m < pPrevLev->nMCVars; m++) {
              pMCVar2 = pPrevLev->rgpMCVars[m];
              if (pMCVar1->hParm[l] == pMCVar2->hvar) {
                pMCVar1->pMCVParent[l] = pMCVar2;
                bFound = TRUE;
              }
            } /* for m */
          } /* for n */
        }

        if (!bFound) { /* oops, parent not found, error */
          printf("\n"
                 "Error: parent in position %ld of %s must be\n"
                 "       declared before it when creating\n"
                 "       sampling dependencies - Exiting.\n\n",
                 l, pMCVar1->pszName);
          exit(0);
        }

      }
    } /* for l */
  } /* for k */

} /* FindMCParents */


/* Function -------------------------------------------------------------------
   GetNumberOfMCVars

   Find the total number of MC vars
   Called from TraverseLevels
*/
void GetNumberOfMCVars (PLEVEL plevel, char **args)
{
  long *pnThetas = (long *) args[0];

  *pnThetas += plevel->nMCVars;

} /* GetNumberOfMCVars */


/* Function -------------------------------------------------------------------
   InitMCVars

   Sample initial values of thetas if not fixed
   Called from TraverseLevels
*/
void InitMCVars(PLEVEL plevel, char **args)
{
  long n;

  for (n = 0; n < plevel->nMCVars; n++)
    if ( !(plevel->rgpMCVars[n]->bIsFixed))
      CalculateOneMCParm (plevel->rgpMCVars[n]);

} /* InitMCVars */


/* Function -------------------------------------------------------------------
   ListToPMCArray

   Convert list to array
*/
void ListToPMCArray (PANALYSIS panal, PLIST plist,
                     long *pnMCVars, PMCVAR **rgpMCVars)
{
  if ((*pnMCVars = ListLength(plist)) == 0)
    return;

  if (!(*rgpMCVars = (PMCVAR*) malloc (*pnMCVars * sizeof(PMCVAR))))
    ReportRunTimeError (panal, RE_OUTOFMEM | RE_FATAL, "ListToPMCArray");

  *pnMCVars = 0;
  ForAllList3 (plist, &ListToPMCArrayL, pnMCVars, *rgpMCVars, NULL);

} /*ListToPMCArray */


/* Function -------------------------------------------------------------------
   ListToPMCArrayL
*/
void ListToPMCArrayL (PVOID pData, PVOID pUser1, PVOID pUser2, PVOID pUser3)
{
  PMCVAR pMCVar = (PMCVAR)pData;
  long *pnMCVars = (long*)pUser1;
  PMCVAR *rgpMCVars = (PMCVAR*)pUser2;

  rgpMCVars[(*pnMCVars)++] = pMCVar;

} /* ListToPMCArrayL */


/* Function -------------------------------------------------------------------
   ListToPVArray
*/
void ListToPVArray (PANALYSIS panal, PLIST plist,
                    long *pnFixedVars, PVARMOD **rgpFixedVars)
{
  if ((*pnFixedVars = ListLength (plist)) == 0)
    return;

  if (!(*rgpFixedVars = (PVARMOD*) malloc (*pnFixedVars * sizeof(PVARMOD))))
    ReportRunTimeError (panal, RE_OUTOFMEM | RE_FATAL, "ListToPVArray");

  *pnFixedVars = 0;
  ForAllList3 (plist, &ListToPVArrayL, pnFixedVars, *rgpFixedVars, NULL);

} /*ListToPVArray */


/* Function -------------------------------------------------------------------
   ListToPVArrayL
*/
void ListToPVArrayL (PVOID pData, PVOID pUser1, PVOID pUser2, PVOID pUser3)
{
  PVARMOD pVar = (PVARMOD)pData;
  long    *pnFixedVars = (long*)pUser1;
  PVARMOD *rgpFixedVars = (PVARMOD*)pUser2;

  rgpFixedVars[(*pnFixedVars)++] = pVar;

} /* ListToPVArrayL */


/* Function -------------------------------------------------------------------
   LnDensity

   Returns the log of the (exact) density of variate under its distribution.
*/
#define LN2PI      1.837877066409345339082
#define LNSQRT2PI  0.918938533204672669541
#define LNINVERPI -1.144729885849400163877
#define LN2OVERPI -0.4515827052894548221396

double LnDensity (PMCVAR pMCVar, PANALYSIS panal)
{
  double dTmp = 0, density;
  double dTmp2, dTmp3, dTmp4;
  double dParm1 = *(pMCVar->pdParm[0]);
  double dParm2 = *(pMCVar->pdParm[1]);
  double dMin   = *(pMCVar->pdParm[2]);
  double dMax   = *(pMCVar->pdParm[3]);
  double dTheta = pMCVar->dVal;
  char str[10];

  /* This should take care of all dTheta checking */
  if (pMCVar->iType == MCV_BINOMIALBETA) {
    if (dTheta < 0) {
      printf("Error: variate out of bounds in LnDensity\n");
      exit(0);
    }
  }
  else if (pMCVar->iType == MCV_GENLOGNORMAL ||
           pMCVar->iType == MCV_STUDENTT) {
    if (dParm1 < 0) {
      printf("Error: parameter %g out of bounds in LnDensity\n",dParm1);
      exit(0);
    }
  }
  else {
    if (dTheta > dMax || dTheta < dMin) {
      return (NULL_SUPPORT);
    }
  }

  switch (pMCVar->iType) {

    case MCV_UNIFORM:
      if (dTheta > dParm2 || dTheta < dParm1)
        return (NULL_SUPPORT);
      if (dParm2 <= dParm1)
        ReportRunTimeError (panal, RE_BADUNIFORMDIST | RE_FATAL,
                            pMCVar->pszName, "LnDensity");
      return -log(dParm2 - dParm1);

    case MCV_LOGUNIFORM:
      if (dTheta > dParm2 || dTheta < dParm1)
        return (NULL_SUPPORT);
      if (dParm2 <= dParm1)
        ReportRunTimeError (panal, RE_BADUNIFORMDIST | RE_FATAL,
                            pMCVar->pszName, "LnDensity");
      return -log(dTheta * (dParm2 - dParm1));

    case MCV_NORMALV:  dParm2 = sqrt (dParm2);
      return lnDFNormal (dTheta, dParm1, dParm2);

    case MCV_NORMALCV: dParm2 = fabs(dParm1 * dParm2);
      return lnDFNormal (dTheta, dParm1, dParm2);

    case MCV_NORMAL:
    case MCV_HALFNORMAL:
      return lnDFNormal (dTheta, dParm1, dParm2);

    case MCV_LOGNORMALV: dParm2 = exp(sqrt(dParm2)); /* fall thru */
    case MCV_LOGNORMAL:
      if (dParm1 <= 0.0) {
        sprintf(str, "%5.2e", dParm1);
        ReportRunTimeError(panal, RE_BADLOGNORMALMEAN | RE_FATAL,
                           pMCVar->pszName, str, "LnDensity");
      }
      return (lnDFNormal(log(dTheta),log(dParm1),log(dParm2)) - log(dTheta));

    case MCV_TRUNCNORMALV: dParm2 = sqrt (dParm2);
      return lnDFNormal (dTheta, dParm1, dParm2) -
             log(CDFNormal ((dMax - dParm1) / dParm2) -
                 CDFNormal ((dMin - dParm1) / dParm2));

    case MCV_TRUNCNORMALCV: dParm2 = fabs(dParm1 * dParm2);
      return lnDFNormal (dTheta, dParm1, dParm2) -
             log(CDFNormal ((dMax - dParm1) / dParm2) -
                 CDFNormal ((dMin - dParm1) / dParm2));

    case MCV_TRUNCNORMAL:
      if (dParm2 <= 0.0) {
        sprintf(str, "%5.2e", dParm2);
        ReportRunTimeError(panal, RE_BADNORMALSD | RE_FATAL,
                           pMCVar->pszName, str, "LnDensity");
      }
      return lnDFNormal (dTheta, dParm1, dParm2) -
             log(CDFNormal ((dMax - dParm1) / dParm2) -
                 CDFNormal ((dMin - dParm1) / dParm2));

    case MCV_TRUNCLOGNORMALV: dParm2 = exp(sqrt(dParm2)); /* fall thru */
    case MCV_TRUNCLOGNORMAL:
      if (dParm1 <= 0.0 ) {
        sprintf(str, "%5.2e", dParm1);
        ReportRunTimeError(panal, RE_BADLOGNORMALMEAN | RE_FATAL,
                           pMCVar->pszName, str, "LnDensity");
      }
      if (dParm2 <= 1.0 ) {
        sprintf(str, "%5.2e", dParm2);
        ReportRunTimeError(panal, RE_BADLOGNORMALSD | RE_FATAL,
                           pMCVar->pszName, str, "LnDensity");
      }
      dTmp = log(dParm2);
      return lnDFNormal (log(dTheta), log(dParm1), dTmp) - log(dTheta) -
             log(CDFNormal (log(dMax / dParm1) / dTmp) -
                 CDFNormal (log(dMin / dParm1) / dTmp));

    case MCV_BETA:
      return lnDFBeta (dTheta, dParm1, dParm2, dMin, dMax);

    case MCV_CHI2:
      dTmp = 0.5 * dParm1;
      return (dTmp - 1) * log(dTheta) - 0.5 * dTheta +
             dTmp * (-6.9314718056E-01) - lnGamma (dTmp);

    case MCV_BINOMIAL:
      if ((dParm1 < 0) || (dParm1 > 1)) {
        printf("Error: bad p for binomial variate in LnDensity\n");
        exit (0);
      }
      if (dTheta > dParm2) {
        return (NULL_SUPPORT);
      }
      /* log binomial coefficient n! / (x!(n-x)!) */
      dTmp = lnGamma (dParm2 + 1) - lnGamma (dTheta + 1) -
             lnGamma (dParm2 - dTheta + 1);

      if (dParm1 == 0) { /* revised FB 18/07/97 */
        if (dTheta != 0)
          return (NULL_SUPPORT); /* should be -INF */
      }
      else
        dTmp = dTmp + dTheta * log(dParm1);

      if (dParm1 == 1) { /* revised FB 18/07/97 */
        if ((dParm2 - dTheta) == 0)
          return dTmp; /* because log(0^0) is 0 */
        else
          return (NULL_SUPPORT); /* should be -INF */
      }
      else
        return dTmp + (dParm2 - dTheta) * log(1 - dParm1);

    case MCV_NEGATIVEBINOM:
      /* dParm1: r, number of runs before success, 
         dParm2: p, probability of success */
      if ((dParm2 < 0) || (dParm2 > 1)) {
        printf("Error: bad p for negative binomial variate in LnDensity\n");
        exit (0);
      }
      /* log binomial coefficient ((x + r - 1)! / (x!(r - 1)!) */
      dTmp = lnGamma (dTheta + dParm1) - lnGamma (dTheta + 1) -
             lnGamma (dParm1);

      if ((dParm2 == 0) && (dTheta != 0))
        return (NULL_SUPPORT); /* should be -INF */
      else
        dTmp = dTmp + dTheta * log(dParm2);

      if (dParm2 == 1) {
        if (dParm1 == 0)
          return dTmp; /* because log(0^0) is 0 */
        else
          return (NULL_SUPPORT); /* should be -INF */
      }
      else
        return (dTmp + dParm1 * log(1 - dParm2));

    case MCV_PIECEWISE:
      density = 2 / (dMax + dParm2 - dParm1 - dMin);

      if (dTheta <= dParm1)
        return log(density * (dTheta - dMin) / (dParm1 - dMin));

      else
        if (dTheta <= dParm2)
          return log(density);
        else
          return log(density * (dMax - dTheta) / (dMax - dParm2));

    case MCV_EXPONENTIAL:
      if (dParm1 <= 0) {
        printf ("Error: negative or null inverse scale (%g) for exponential "
                "variate in LnDensity\n", dParm1);
        exit (0);
      }
      return -dTheta * dParm1 + log(dParm1);

    case MCV_GGAMMA:
      if (dParm2 <= 0) {
        printf ("Error: bad inv. scale for gamma variate in LnDensity\n");
        exit (0);
      }
      return (dParm1 - 1) * log(dTheta) - dParm2 * dTheta +
             dParm1 * log(dParm2) - lnGamma (dParm1);

    case MCV_TRUNCINVGGAMMA:
#ifdef HAVE_LIBGSL
      dTmp = gsl_cdf_gamma_P (1/dMax, dParm1, dParm2) -
             gsl_cdf_gamma_P (1/dMin, dParm1, dParm2); /* fall thru */
#else
      printf("Error: Truncated inverse gamma density cannot be evaluated\n");
      printf("       if the GNU Scientific Library is not installed\n");
      exit(0);
#endif
    case MCV_INVGGAMMA:
      if (dParm2 <= 0) {
        printf("Error: bad scale for inv. gamma variate in LnDensity\n");
        exit(0);
      }
      return (-dParm1 - 1) * log(dTheta) - dParm2 / dTheta +
             dParm1 * log(dParm2) - lnGamma (dParm1) - dTmp;

    case MCV_POISSON:
      if (dParm1 <= 0) {
        printf("Error: bad rate for Poisson variate in LnDensity\n");
        exit(0);
      }
      return dTheta * log(dParm1) - dParm1 - lnGamma (dTheta + 1);

    case MCV_BINOMIALBETA:
      if (dParm1 < 0) {
        printf("Error: bad expectation for BinomialBeta variate "
               "in LnDensity\n");
        exit(0);
      }
      if (dParm2 <= 0) {
        printf("Error: bad alpha for BinomialBeta variate in LnDensity\n");
        exit(0);
      }
      if (dMin <= 0) {
        printf("Error: bad beta for BinomialBeta variate in LnDensity\n");
        exit(0);
      }
      dTmp = floor (0.5 + dParm1 + dParm1 * dMin / dParm2); /* this is N */
      if (dTheta > dTmp)
        return (NULL_SUPPORT);
      else {
        if ((dParm2 == 0.5) && (dMin == 0.5))
          dTmp = lnGamma (0.5 + dTheta) +
                 lnGamma (0.5 + dTmp - dTheta) -
                 lnGamma (dTheta + 1) - lnGamma (dTmp - dTheta + 1);
        else
          dTmp = lnGamma (dParm2 + dMin) + lnGamma (dTmp + 1) +
                 lnGamma (dParm2 + dTheta) +
                 lnGamma (dMin + dTmp - dTheta) -
                 lnGamma (dTheta + 1) - lnGamma (dTmp - dTheta + 1) -
                 lnGamma (dParm2) - lnGamma(dMin) -
                 lnGamma (dParm2 + dMin + dTmp);
        return dTmp;
      }

    case MCV_GENLOGNORMAL: /* Generalized LogNormal */
      if (dParm1 < 0) {
        printf("Error: bad expectation for GenLogNormal variate "
               "in LnDensity\n");
        exit(0);
      }
      /* This is relative stdev of lognormal part -- dMin is sigma for the
         lognormal part */
      dTmp = sqrt(exp(pow(dMin,2)) * (exp(pow(dMin,2)) - 1));
      /* This is the transformation parameter lambda */
      dTmp2 = pow(dParm2/dTmp,2);
      /* This is the transformed mean */
      dTmp3 = log(dParm1 + sqrt(pow(dParm1,2) + dTmp2));
      /* This is the transformed theta -- use Taylor series for
         negative dTheta when lambda/dTheta^2 < 0.01 */
      if ((dTheta < 0) && (dTmp2 < (0.01*dTheta*dTheta)))
        dTmp4 = log(dTmp2/(-2*dTheta)*(1+0.25*dTmp2/pow(dTheta,2)));
      else
        dTmp4 = log(dTheta + sqrt(pow(dTheta,2) + dTmp2));
      /* Normal log-density minus log-jacobian */
      return lnDFNormal( dTmp4, dTmp3, dTmp ) - 0.5*log(pow(dTmp4,2) + dTmp2);

    case MCV_STUDENTT: /* Student t, 
                          dParm1 is dof, dParm2 is m, dMin is sigma */
      if (dParm1 <= 0) {
        printf("Error: bad dof for Student-T variate"
               "in LnDensity\n");
        exit(0);
      }
      dTmp = (dParm1 + 1)/ 2;
      return (lnGamma(dTmp) - lnGamma(dParm1 / 2)
              -0.5 * log(dParm1 * PI *dMin *dMin)
              -dTmp * log(1 + pow((dTheta - dParm2) / dMin, 2) / dParm1));

    case MCV_CAUCHY:
      return (LNINVERPI - log(dParm1 + dTheta * dTheta / dParm1));

    case MCV_HALFCAUCHY: /* dParm1 is scale */
      return (LN2OVERPI - log(dParm1 + dTheta * dTheta / dParm1));

    case MCV_USERLL:     /* dParm1 is the model computed log-likelihood */
      return (dParm1);

    default:
      ReportRunTimeError(panal, RE_UNKNOWNDIST | RE_FATAL, "LnDensity");

  } /* switch */

  /* Not reached */
  return 0.0 ;

} /* LnDensity */


/* Function -------------------------------------------------------------------
   LnLike

   returns the log-likelihood of the dependents given the parameter specified
   by pMCVar
*/
double LnLike (PMCVAR pMCVar, PANALYSIS panal)
{
  long n;
  double dDensity, dLnLike = 0.0;

  for (n = 0; n < pMCVar->nDependents; n++) {
    dDensity = LnDensity(pMCVar->rgpDependents[n], panal);
    if (dDensity == NULL_SUPPORT)
      return NULL_SUPPORT;
    else
      dLnLike += dDensity;
  }
  return dLnLike;

} /* LnLike */


/* Function -------------------------------------------------------------------
   LnLikeData

   Likelihood of the data for one experiment
*/
double LnLikeData (PLEVEL plevel, PANALYSIS panal)
{
  PMCVAR pMCVar;
  long   i, j, k;
  double dLnLike = 0.0;
  double dTmp;
  BOOL   bMissData, bMissOutp;
  static PDOUBLE pdBase[4];

  /* For all the likelihoods seen at the experiment level */
  for (i = 0; i < plevel->nLikes; i++) {
    pMCVar = plevel->rgpLikes[i];

    for (k = 0; k < 4; k++)
      pdBase[k] = pMCVar->pdParm[k]; /* store the base pointers of pdParm */

    /* Set dVal of pMCVar to the current data value */
    for (j = 0; j < pMCVar->lCount; j++) { /* for each data value */

      pMCVar->dVal = pMCVar->pdVal[j]; /* point to jth data value */

      /* If the main data is coded as missing do nothing, and otherwise: */
      if (pMCVar->dVal != INPUT_MISSING_VALUE) {
        /* Set the pdParms of pMCVar to point to data or output values */
        bMissData = FALSE; /* initialize */
        bMissOutp = FALSE; /* initialize */
        for (k = 0; k < 4; k++) {
          if (pMCVar->iParmType[k] == MCVP_PRED) {
            /* Advance in the prediction array */
            pMCVar->pdParm[k] = pdBase[k] + j;
            bMissOutp = bMissOutp + (*(pMCVar->pdParm[k]) == MISSING_VALUE);
          }
          else if (pMCVar->iParmType[k] == MCVP_DATA) {
            /* Advance in the data array */
            pMCVar->pdParm[k] = pdBase[k] + j;
            bMissData = bMissData +
                        (*(pMCVar->pdParm[k]) == INPUT_MISSING_VALUE);
          }
        } /* end for k */

        /* If no missing data among the k parameters */
        if (bMissData == FALSE) {
          /* If no missing model output among the k parameters */
          if (bMissOutp == FALSE) {
            dTmp = LnDensity (pMCVar, panal);
            if (dTmp == NULL_SUPPORT) {
              /* Reset the pdParms to the beginning of arrays  FB 9/6/99 */
              for (k = 0; k < 4; k++)
                pMCVar->pdParm[k] = pdBase[k];
              return (NULL_SUPPORT);
            }
            else
              dLnLike = dLnLike + dTmp;
          }
          else /* missing output, cannot compute */
            ReportRunTimeError (panal, RE_BADMODEL | RE_FATAL, "LnLikeData");
        } /* end if bMissData */

      } /* end if ! INPUT_MISSING_VALUE */
    } /* end for j */

    /* Reset the pdParms to the beginning of the output or data arrays */
    for (k = 0; k < 4; k++)
      pMCVar->pdParm[k] = pdBase[k];

  } /* end for i */

  return (dLnLike);

} /* LnLikeData */


/* Function -------------------------------------------------------------------
   MaxMCVar

   Get the upper bound of an MCVar (random variable).
*/
double MaxMCVar (PMCVAR pMCVar)
{
  /* if the parameter has a discrete distribution, round it - FB 12/06/97 */
  if (pMCVar->iType == MCV_BINOMIAL || pMCVar->iType == MCV_POISSON ) {
   return( *(pMCVar->pdParm[3]));
  }
  else { /* FB fixed the uniform case - FB 30/06/97 */
    if (pMCVar->iType == MCV_UNIFORM || pMCVar->iType == MCV_LOGUNIFORM ) {
      return( *(pMCVar->pdParm[1]));
    }
    else {
      return( *(pMCVar->pdParm[3]));
    }
  }

} /* MaxMCVar*/


/* Function -------------------------------------------------------------------
   MinMCVar

   Get the lower bound of an MCVar (a random variable)
*/
double MinMCVar (PMCVAR pMCVar)
{
  /* if the parameter has a discrete distribution, round it - FB 12/06/97 */
  if (pMCVar->iType == MCV_BINOMIAL || pMCVar->iType == MCV_POISSON) {
   return(*(pMCVar->pdParm[2]));
  }
  else { /* FB fixed the uniform case - FB 30/06/97 */
    if (pMCVar->iType == MCV_UNIFORM || pMCVar->iType == MCV_LOGUNIFORM) {
      return( *(pMCVar->pdParm[0]));
    }
    else {
      return( *(pMCVar->pdParm[2]));
    }
  }

} /* MinMCVar */


/* Function -------------------------------------------------------------------
   RunTemperingBlock

   Run a block of MCMC simulations to adjust the perk scale and pseudo-priors
*/
void RunTemperingBlock (PANALYSIS panal, long lRunLength, PLONG iter)
{
  PGIBBSDATA pgd = &panal->gd;
  PLEVEL     pLevel0 = panal->pLevels[0];
  double     dTmp, dLnPrior = 0, dLnData = 0;
  long       i, j;
  long       nUpdateAt, nTotal;

  for (i = 0; i < lRunLength; i++) { /* run block */

    nTotal = UPDATE_BASE;
    nUpdateAt = *iter + nTotal;

    TraverseLevels (pLevel0, SampleThetasTempered, panal, pgd, &i,
                    &nUpdateAt, &nTotal, &pgd->indexT, NULL);

    dLnPrior = 0.0;
    TraverseLevels (pLevel0, CalculateTotals, panal, &dLnPrior, NULL);
    dLnData = 0.0;
    TraverseLevels1 (pLevel0, SumAllExpts, &dLnData, NULL);

    /* special? Robbins-Munro updating of the pseudo-priors */
    dTmp = pgd->dCZero / (double) (i + pgd->dNZero);
    /*if (pgd->indexT == pgd->startT) {
      pgd->rgdlnPi[pgd->startT] -= dTmp;
      for (j = pgd->startT + 1; j <= pgd->endT; j++)
        pgd->rgdlnPi[j] += dTmp / (double) pgd->nPerks;
    }
    else {
      pgd->rgdlnPi[pgd->startT] += dTmp;
      for (j = pgd->startT + 1; j <= pgd->endT; j++)
        pgd->rgdlnPi[j] -= dTmp / (double) pgd->nPerks;
    }
    */

    for (j = pgd->startT; j <= pgd->endT; j++) {
      if (j == pgd->indexT)
        pgd->rgdlnPi[j] -= dTmp;
      else
        pgd->rgdlnPi[j] += dTmp / (double) pgd->nPerks;
    }

    /* update population count of current temperature */
    pgd->rglCount[pgd->indexT] = pgd->rglCount[pgd->indexT]+1;

    /* test the temperature and change indexT if necessary */
    pgd->indexT = SampleTemperature2 (pgd, dLnPrior, dLnData);

    /* Adjust the update time eventually */
    if (i == nUpdateAt) {
      nTotal = (nTotal * 3) / 2;
      nUpdateAt = i + nTotal;
    }

    /* Increment the total iteration counter */
    (*iter)++;

  } /* end for */

} /* RunTemperingBlock */


/* Function -------------------------------------------------------------------
   NextDown

   Return from a table the element just inferior to the argument
*/
double NextDown (double Perk)
{
  int i;
  static double PTable[21] = {0,    1E-6, 1E-5, 1E-4,  1E-3,   1E-2,
                              0.1,  0.2,  0.3,  0.5,   0.6,    0.7, 0.8, 0.9,
                              0.95, 0.97, 0.99, 0.999, 0.9999, 0.99999, 1};

  i = 0;
  while (Perk > PTable[i]) {
    i++;
  }

  return (i == 0 ? PTable[i] : PTable[i-1]);

} /* NextDown */


/* Function -------------------------------------------------------------------
   InitPerks

   Adjusts automatically the perk schedule and pseudo-priors
   for tempered MCMC, if the user has not specified values.
*/
void InitPerks (PANALYSIS panal)
{
  PGIBBSDATA pgd = &panal->gd;
  long       i, j, k, iter = 0;
  double     dTmp;
  int        bTrans;
  BOOL       bHappy, bTooManyTrials;

  /* if we are doing tempered MCMC */
  if ((pgd->nSimTypeFlag == 3) || (pgd->nSimTypeFlag == 4)) {

    /* allocate transition counts array */
    if (!(pgd->rglTransAttempts = InitlVector (NTEMP)) ||
        !(pgd->rglTransAccepts  = InitlVector (NTEMP)))
      ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "InitPerks", NULL);

    /* initialize them at zero */
    for (i = 0; i < NTEMP; i++)
      pgd->rglTransAttempts[i] = pgd->rglTransAccepts[i] = 0;

    /* if perks not set by the user */
    if (pgd->nPerks == 0) {

      /* Screen message */
      printf ("Setting perks (inverse temperatures).\n");

      pgd->nPerks = NTEMP;

      /* allocate perk, pseudo-prior and population count arrays */
      if (!(pgd->rgdPerks = InitdVector (NTEMP)) ||
          !(pgd->rgdlnPi  = InitdVector (NTEMP)) ||
          !(pgd->rglCount = InitlVector (NTEMP)))
        ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "InitPerks", NULL);

      for (i = 0; i < NTEMP; i++) {
        pgd->rgdPerks[i] = pgd->rgdlnPi[i] = pgd->rglCount[i] = 0;
      }

      /* start at perk 1 and just below it */
      pgd->endT   = NTEMP - 1;
      pgd->startT = NTEMP - 2;
      pgd->indexT = pgd->startT;

      double dEPSILON = 0.99;
      double dUP = 2.0;
      pgd->rgdPerks[pgd->startT] = dEPSILON;
      pgd->rgdPerks[pgd->endT]   = 1.00;

      long nOldPrintIter = pgd->nPrintIter;
      pgd->nPrintIter = -pgd->nMaxPerkSetIter;
      int lRunLength = 100;
      double dBoundary = 0.0;

      do { /* loop over running a block, checking results, and adjusting */

        pgd->indexT = pgd->startT;

        /* run a batch of tempered MCMC simulations */
        RunTemperingBlock (panal, lRunLength, &iter);

        /* post-run diagnostic printing */
        PrintTemperatureDiagnostics (stdout, pgd);
        PrintTemperatureDiagnostics (pgd->pfilePerks, pgd);

        /* check block */
        bTrans = CheckTransitions (pgd);
        if (pgd->rgdPerks[pgd->startT] == dBoundary) {
          /* boundary perk reached */
          bHappy = (bTrans > -1); /* transition OK or too high */
        }
        else { /* boundary perk not reached */
          bHappy = FALSE;
        }
        bTooManyTrials = (iter > pgd->nMaxPerkSetIter);

        /* if unhappy: adjust */
        if (!bHappy) {

          if (bTrans == -1) { /* acceptance rate too low */

            printf ("acceptance rate 1<->2 too low, stepping back up\n");

            /* go back up half way to the point above */
            dTmp = (pgd->rgdPerks[pgd->startT] +
                    pgd->rgdPerks[pgd->startT+1]) / dUP;

            /* adjust pseudo-prior with an average of old and extrapolated */
            pgd->rgdlnPi[pgd->startT] = (pgd->rgdlnPi[pgd->startT] +
              Extrapolate (pgd, dTmp, pgd->startT, pgd->startT+1)) / 2;

            pgd->rgdPerks[pgd->startT] = dTmp;

          } /* end too low */

          if (bTrans == +1) { /* acceptance rate too high */

            printf ("acceptance rate 1<->2 too high, moving down\n");

            dTmp = NextDown(pgd->rgdPerks[pgd->startT]);

            /* adjust pseudo-prior with an average of old and extrapolated */
            pgd->rgdlnPi[pgd->startT] = (pgd->rgdlnPi[pgd->startT] +
              Extrapolate (pgd, dTmp, pgd->startT, pgd->startT+1)) / 2;

            pgd->rgdPerks[pgd->startT] = dTmp;

          } /* end too high */

          if (bTrans == 0) {
            /* acceptance rate ok, but target perk not reached
               check if we have space to add a point below */
            if (pgd->startT > 0) { /* we do have space */

              printf ("acceptance rate 1<->2 ok, adding a new point\n");

              /* if all transitions rates are OK the pseudo-priors should be
                 about right and we might be able to take some intermediate
                 points off the scale */
              if (CheckAllTransitions(pgd)) {
                /* remove recursively center point if 3 points are aligned */
                int i = pgd->endT;
                int j = i - 2;
                while (j >= pgd->startT) {
                  if (EqualSlopes(pgd->rgdPerks, pgd->rgdlnPi, j)) {
                    /* remove point i - 1 from the scale, repacking
                       the scale that implies copying perks,
                       pseudo-prior, and counts from positions start
                       to (i - 2) to positions (start + 1) to (i - 1)
                       and setting start to (start + 1).  start has
                       moved up, so j does not need to be updated */
                    for (k = j; k >= pgd->startT; k--) {
                      pgd->rgdPerks[k+1] = pgd->rgdPerks[k];
                      pgd->rgdlnPi [k+1] = pgd->rgdlnPi [k];
                      pgd->rglCount[k+1] = pgd->rglCount[k];
                    }
                    pgd->startT++;
                    if (pgd->indexT <= j)
                      pgd->indexT++;
                    lRunLength = lRunLength - 100;
                    printf("Scale has been reduced.\n");
                  }
                  else { /* slopes not equal, move i and j down */
                    i--;
                    j--;
                  }
                }
              }

              pgd->startT = pgd->startT - 1;
              pgd->indexT = pgd->startT;

              /* give perk to the new point */
              pgd->rgdPerks[pgd->startT] =
                NextDown(pgd->rgdPerks[pgd->startT+1]);

              /* pseudo-prior of new point is an average of above and
                 extrapolated */
              pgd->rgdlnPi[pgd->startT] = (pgd->rgdlnPi[pgd->startT+1] +
                Extrapolate (pgd, pgd->rgdPerks[pgd->startT],
                             pgd->startT+1, pgd->startT+2)) / 2;

              lRunLength = lRunLength + 100;
            }
            else /* no more space in scale, stop */
              bTooManyTrials = TRUE;

          } /* end OK */

          /* re-initialize counts at zero */
          for (i = pgd->startT; i <= pgd->endT; i++) {
            pgd->rglCount[i] = 0;
            pgd->rglTransAttempts[i] = pgd->rglTransAccepts[i] = 0;
          }

        } /* end if !bHappy */

      } while ((!bHappy) && (!bTooManyTrials));

      if (pgd->rgdPerks[pgd->startT] == dBoundary)
        printf ("Perk %lg reached in %ld iterations.\n", dBoundary, iter);
      else
        printf ("Perk %lg not reached in %ld iterations...\n", dBoundary, iter);

      /* restore */
      pgd->nPrintIter = nOldPrintIter;

      /* prepare the actual runs */
      int iCount = pgd->endT - pgd->startT + 1;
      if (iCount != NTEMP) { /* shift array contents to start at 0 */
        pgd->nPerks = iCount;
        pgd->indexT = pgd->indexT - pgd->startT;
        for (i = 0; i < iCount; i++) {
          j = pgd->startT + i;
          pgd->rgdPerks[i] = pgd->rgdPerks[j];
          pgd->rgdlnPi[i]  = pgd->rgdlnPi[j];
          pgd->rglCount[i] = 0;
        }
        pgd->startT = 0;
        pgd->endT   = iCount - 1;
      }

      printf ("Done with InitPerks - Continuing.\n\n");

    } /* if perks not set by user */
  } /* if tempered */

} /* InitPerks */


/* Function -------------------------------------------------------------------
   OpenMarkovFiles

   Opens the output file and the restart file.
*/
void OpenMarkovFiles (PANALYSIS panal)
{
  PGIBBSDATA pgd = &panal->gd;
  char* with_rank;

  /* if we are debugging the structure, do nothing here */
  if (panal->bDependents) return;

  /* Take care of the output file first */

  /* Use command line spec if given */
  if (panal->bCommandLineSpec) {
    free (pgd->szGout);
    panal->bAllocatedFileName = FALSE;
    pgd->szGout = panal->szOutfilename;
  }

  /* Default if none given */
  else if (!(pgd->szGout))
    pgd->szGout = "MCMC.default.out";

  if (panal->size > 1) {
    with_rank = malloc(sizeof(char)*(6+strlen(pgd->szGout)));
    sprintf(with_rank,"%04d_%s",panal->rank,pgd->szGout);
    pgd->szGout = with_rank;
  }

  /* Eventually open the restart file before crushing the output file */
  if (pgd->szGrestart)
    if (!(pgd->pfileRestart)
      && !(pgd->pfileRestart = fopen (pgd->szGrestart, "r")))
      ReportRunTimeError(panal, RE_FATAL | RE_CANNOTOPEN,
                         pgd->szGrestart, "OpenMarkovFiles");

  if (!(pgd->pfileOut)
      && !(pgd->pfileOut = fopen (pgd->szGout, "w")))
    ReportRunTimeError(panal, RE_FATAL | RE_CANNOTOPEN,
                       pgd->szGout, "OpenMarkovFiles");

  /* If tempered MCMC, open perk scale recording file */
  if ((pgd->nSimTypeFlag == 3) || (pgd->nSimTypeFlag == 4)) {
    char szFileName[MAX_FILENAMESIZE+6];
    sprintf(szFileName, "%s%s", pgd->szGout, ".perks");
    if (!(pgd->pfilePerks)
        && !(pgd->pfilePerks = fopen (szFileName, "w")))
      ReportRunTimeError(panal, RE_FATAL | RE_CANNOTOPEN,
                         szFileName, "OpenMarkovFiles");
  }

} /* OpenMarkovFiles */


/* Function -------------------------------------------------------------------
   PrintAllExpts

   Run and print all experiments at level below `plevel'
*/
void PrintAllExpts (PLEVEL plevel, PANALYSIS panal, PFILE pOutFile)
{
  long n;

  for (n = 0; n < plevel->iInstances; n++)
    TraverseLevels1 (plevel->pLevels[n], PrintExpt, panal, pOutFile, NULL);

} /* PrintAllExpts */


/* Function -------------------------------------------------------------------
   PrintDeps

   Called from TraverseLevels

   For debugging, print the variables, parents, and dependencies
*/
void PrintDeps (PLEVEL plevel, char **args)
{
  long n, m;
  PMCVAR pMCVar;

  printf("Depth %d; Instance %d\n", plevel->iDepth, plevel->iSequence);

  for (n = 0; n < plevel->nMCVars; n++) {
    pMCVar = plevel->rgpMCVars[n];

    printf ("Variable %s (%d) [%" PRIxPTR "]\n",
            pMCVar->pszName, pMCVar->iDepth, (intptr_t) pMCVar);

    for (m = 0; m < 4; m++)
      if (pMCVar->pMCVParent[m] != NULL)
        printf ("  Parent %ld: %s (%d) [%" PRIxPTR "]\n", m,
                pMCVar->pMCVParent[m]->pszName, pMCVar->pMCVParent[m]->iDepth,
                (intptr_t) pMCVar->pMCVParent[m]);

    for (m = 0; m < pMCVar->nDependents; m++)
      printf ("  Dependent: %s (%d) [%" PRIxPTR "]\n",
              pMCVar->rgpDependents[m]->pszName,
              pMCVar->rgpDependents[m]->iDepth,
              (intptr_t) pMCVar->rgpDependents[m]);

    if (pMCVar->bExptIsDep)
      printf("  This variable influences experiments directly\n");
  }

} /* PrintDeps */


/* Function -------------------------------------------------------------------
   PrintExpt

   Run all experiments and print the level code, experiment number, time, data,
   and predictions to the output file.

   Called from TraverseLevels1
*/
int PrintExpt (PLEVEL plevel, char **args)
{
  PANALYSIS   panal = (PANALYSIS)args[0];
  PFILE       pOutFile = (PFILE)args[1];
  long        k, l, m, n;
  PEXPERIMENT pExpt = plevel->pexpt;
  POUTSPEC    pos;
  static long printed_head = 0;

  if (!printed_head) {
    fprintf (pOutFile,
             "Level\tSimulation\tOutput_Var\tTime\tData\tPrediction\n");
    printed_head = 1;
  }

  /* Set level sequence */
  panal->pCurrentLevel[plevel->iDepth] = plevel;

  panal->iInstance[plevel->iDepth] = plevel->iSequence;

  if (pExpt != NULL) {
    InitModel ();

    /* Set the model vars that have been sampled in this experiment and
       above levels */
    for (n = 0; n <= plevel->iDepth; n++) {
      SetModelVars (panal->pCurrentLevel[n]);
      SetFixedVars (panal->pCurrentLevel[n]);
    }

    if (!DoOneExperiment (pExpt)) {
      /* Error */
      printf ("Warning: DoOneExperiment failed\n");
      return 0;
    }
    else {
      pos = &pExpt->os;
      for (m = 0; m < pos->nOutputs; m++) {
        /* find the corresponding data index in case Print and Data are not
           in the same order */
        for (k = 0; k < pos->nData; k++)
          if (!strcmp(pos->pszDataNames[k], pos->pszOutputNames[m]))
            break;

        for (l = 0; l < pos->pcOutputTimes[m]; l++) {
          for (n = 1; n < plevel->iDepth; n++)
            fprintf (pOutFile, "%d_", panal->iInstance[n]);
          fprintf (pOutFile, "%d\t", panal->iInstance[plevel->iDepth]);

          if (k != pos->nData) /* Data found */
            fprintf (pOutFile, "%d\t%s\t%g\t%g\t%g\n", pExpt->iExp,
                     pos->pszOutputNames[m], pos->prgdOutputTimes[m][l],
                     pos->prgdDataVals[k][l], pos->prgdOutputVals[m][l]);
          else /* data not found, empty field */
            fprintf (pOutFile, "%d\t%s\t%g\t\t%g\n", pExpt->iExp,
                     pos->pszOutputNames[m], pos->prgdOutputTimes[m][l],
                     pos->prgdOutputVals[m][l]);
        } /* end for l */
        fprintf (pOutFile, "\n");

      } /* end for m */
      fprintf (pOutFile, "\n");

    } /* end else */

  } /* end if pExpt */

  return (1);

} /* PrintExpt*/


/* Function -------------------------------------------------------------------
   PrintTemperatureDiagnostics

   For debugging and information, print the state of the tempering algorithm
*/
void PrintTemperatureDiagnostics (PFILE fOut, PGIBBSDATA pgd)
{
  register int i;

  fprintf (fOut, "\nPerks:");
  for (i = pgd->startT; i <= pgd->endT; i++) {
    fprintf (fOut, "\t%g", pgd->rgdPerks[i]);
  }
  fprintf (fOut, "\nCounts:");
  for (i = pgd->startT; i <= pgd->endT; i++) {
    fprintf (fOut, "\t%ld", pgd->rglCount[i]);
  }
  fprintf (fOut, "\nLnPi(i):");
  for (i = pgd->startT; i <= pgd->endT; i++) {
    fprintf (fOut, "\t%g", pgd->rgdlnPi[i]);
  }
  fprintf (fOut, "\nTried Jumps:\t");
  for (i = pgd->startT; i <= pgd->endT - 1; i++) {
    fprintf (fOut, "\t%ld", pgd->rglTransAttempts[i]);
  }
  fprintf (fOut, "\nAccepted Jumps:\t");
  for (i = pgd->startT; i <= pgd->endT - 1; i++) {
    fprintf (fOut, "\t%ld", pgd->rglTransAccepts[i]);
  }
  fprintf(fOut, "\n\n");
  fflush(fOut);

#ifdef ndef
  /* This can be computed by the user if s/he wishes */
  if (eGeyer == Geyer) {
    /* Geyer's proposed adjustment of pseudo-priors */
    BOOL bZeroes;
    for (i = pgd->startT; i <= pgd->endT; i++) { /* check for zero counts */
      bZeroes = (pgd->rglCount[i] == 0);
      if (bZeroes) break; /* a zero count found, stop */
    }
    if (bZeroes) {
      fprintf (fOut, "Adjusted LnPi(i) not computable, zero-counts found.\n");
    }
    else {
      fprintf (fOut, "Adjusted LnPi(i):    ");
      for (i = pgd->startT; i <= pgd->endT; i++) {
        pgd->rgdlnPi[i] = pgd->rgdlnPi[i] - log(pgd->rglCount[i]);
        fprintf (fOut, "% 10.6lE", pgd->rgdlnPi[i]);
      }
      fprintf (fOut, "\n");
    }
  }
#endif

} /* PrintTemperatureDiagnostics */


/* Function -------------------------------------------------------------------
   ReadData

   Reads data for an experiment. There need to be one data item
   per output specified, and in the order given by the Print or PrintStep
   statements.
   Upgraded by FB 27/03/99 to deal with improved data handling.
   Called from TraverseLevels.
*/
void ReadData (PLEVEL plevel, char **args)
{
  FILE *pfileData = (FILE *) args[0];
  POUTSPEC pos;
  int cDat, i, j;

  if (plevel->pexpt == NULL)
    return;

  pos = &(plevel->pexpt->os);

  /* here the number of data must equal the number of outputs */
  cDat = pos->nOutputs;
  pos->prgdDataVals = InitpdVector (cDat);
  pos->pcData       = InitiVector (cDat);  /* FB 5/11/99 */
  pos->pszDataNames = (PSTR *) malloc (cDat * sizeof(PSTR));
  pos->phvar_dat    = (HVAR *) malloc (cDat * sizeof(HVAR));

  if (pos->prgdDataVals == NULL || pos->phvar_dat == NULL ||
      pos->pszDataNames == NULL || pos->pcData    == NULL)
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadData()", NULL);
  else {
    pos->nData = cDat;  /* FB 27/03/99: Set count of data */

    /* scan all data values for data file */
    for (i = 0; i < cDat; i++) {
      /* allocate space for pos->prgdDataVals[i] */
      if ( !(pos->prgdDataVals[i] = InitdVector (pos->pcOutputTimes[i])))
        ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadData()", NULL);;

      /* for each requested output time */
      for (j = 0; j < pos->pcOutputTimes[i]; j++)
        if (fscanf(pfileData, "%lg", &(pos->prgdDataVals[i][j])) == EOF) {
          printf ("Error: incorrect length for data file - Exiting\n");
          exit(0);
        }
      pos->pcData[i] = j; /* FB 5/11/99 */
      pos->phvar_dat[i] = pos->phvar_out[i];
      pos->pszDataNames[i] = pos->pszOutputNames[i];

    } /* for i */

  } /* else */

} /* ReadData */


/* Function -------------------------------------------------------------------
   ReadDataFile

   Reads in data from a specified data file, if specified.
*/
void ReadDataFile (PANALYSIS panal)
{
  if (panal->gd.szGdata) {

    char c;

    FILE *pfile = fopen (panal->gd.szGdata, "r");

    if (!pfile) {
      printf ("Cannot open data file '%s'\n", panal->gd.szGdata);
      exit (0);
    }

    /* skip the first line */
    do { c = getc(pfile); } while (c != '\n');

    TraverseLevels (panal->pLevels[0], ReadData, pfile, NULL);

    fclose (pfile);
  }

} /* ReadDataFile */


/* Function -------------------------------------------------------------------
   ReadRestart

   initialize the parameters by reading them in the restart file.
*/
void ReadRestart (FILE *pfileRestart, long nThetas,
                  PDOUBLE *pdTheta, PDOUBLE *pdSum, PDOUBLE **prgdSumProd,
                  long *pIter)
{
  register char c;
  register long i, j;

  if (*pdTheta == NULL)
    if ( !(*pdTheta = InitdVector(nThetas)) )
      ReportRunTimeError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadRestart");

  if (*pdSum == NULL)
    if ( !(*pdSum = InitdVector(nThetas)) )
      ReportRunTimeError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadRestart");

  if (*prgdSumProd == NULL)
    if ( !(*prgdSumProd = InitdMatrix (nThetas, nThetas)) )
      ReportRunTimeError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadRestart");

  *pIter = -1;

  for (i = 0; i < nThetas; i++) {
    (*pdSum)[i] = 0.0;
    for (j = 0; j < nThetas; j++)
      (*prgdSumProd)[i][j] = 0.0;
  }

  /* skip the first line. This allows a MC output file to be used
     directly as a restart file. */
  do { c = getc(pfileRestart); } while (c != '\n');

  /* as long as we have not reached the end of the file we keep reading lines
     and overwriting the thetas, they keep only their last value.
     We throw away first field, and we keep incrementing the global iteration
     counter iter:
   */
  while (!(feof (pfileRestart) ||
          (fscanf (pfileRestart, "%*s") == EOF))) {
    for (i = 0; i < nThetas; i++) {
      if ( fscanf(pfileRestart, "%lg", &((*pdTheta)[i])) == EOF ) {
        printf ("Error: incorrect length for restart file - Exiting\n");
        exit(0);
      }
      else { /* reading ok */
        /* update pdSum, the column sum of pdTheta */
        (*pdSum)[i] += (*pdTheta)[i];
      }
    }

    /* Throw away remainder of line. This allows a MC output file to be used
       directly as a restart file. */
    do { c = getc(pfileRestart); } while (c != '\n');

    /* update prgdSumProd */
    for (i = 0; i < nThetas; i++)
      for (j = 0; j < nThetas; j++)
        (*prgdSumProd)[i][j] += (*pdTheta)[i] * (*pdTheta)[j];


    /* increment pIter */
    *pIter = *pIter + 1;

  } /* end while */

  /* note that the theta returned is the last parameter set read */

  fclose (pfileRestart);

} /* ReadRestart */


/* Function -------------------------------------------------------------------
   ReadRestartTemper

   initialize parameters and temperature stuff (pseudoprior, indexT) by reading
   them in the restart file.
*/
void ReadRestartTemper (FILE *pfileRestart, long nThetas, int nPerks,
                        PDOUBLE *pdTheta, PDOUBLE *pdSum, PDOUBLE **prgdSumProd,
                        long *pIter, int *pindexT, double *pdlnPi)
{
  register char c;
  register long i, j;

  if (*pdTheta == NULL)
    if ( !(*pdTheta = InitdVector(nThetas)) )
      ReportRunTimeError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadRestart");

  if (*pdSum == NULL)
    if ( !(*pdSum = InitdVector(nThetas)) )
      ReportRunTimeError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadRestart");

  if (*prgdSumProd == NULL)
    if ( !(*prgdSumProd = InitdMatrix (nThetas, nThetas)) )
      ReportRunTimeError (NULL, RE_OUTOFMEM | RE_FATAL, "ReadRestart");

  *pIter = -1;

  for (i = 0; i < nThetas; i++) {
    (*pdSum)[i] = 0.0;
    for (j = 0; j < nThetas; j++)
      (*prgdSumProd)[i][j] = 0.0;
  }

  /* skip the first line. This allows a MC output file to be used
     directly as a restart file. */
  do { c = getc(pfileRestart); } while (c != '\n');

  /* as long as we have not reached the end of the file we keep reading lines
     and overwriting the thetas, they keep only their last value.
     We throw away first field, and we keep incrementing the global iteration
     counter iter: */
  while (!(feof (pfileRestart) ||
           (fscanf (pfileRestart, "%*s") == EOF))) {
    for (i = 0; i < nThetas; i++) {
      if ( fscanf(pfileRestart, "%lg", &((*pdTheta)[i])) == EOF ) {
        printf ("Error: incorrect length for restart file - Exiting\n");
        exit(0);
      }
      else { /* reading ok */
        /* update pdSum, the column sum of pdTheta */
        (*pdSum)[i] += (*pdTheta)[i];
      }
    }

    if (fscanf(pfileRestart,"%d", pindexT) == EOF) {
      printf ("Error: incorrect length for restart file - Exiting\n");
      exit(0);
    }

    for (i = 0; i < nPerks; i++) {
      if (fscanf(pfileRestart,"%lg", &(pdlnPi[i])) == EOF) {
        printf ("Error: incorrect length for restart file - Exiting\n");
        exit(0);
      }
    }

    /* Throw away remainder of line. This allows a MC output file to be used
       directly as a restart file. */
    do { c = getc(pfileRestart); } while (c != '\n');

    /* update prgdSumProd */
    for (i = 0; i < nThetas; i++)
      for (j = 0; j < nThetas; j++)
        (*prgdSumProd)[i][j] += (*pdTheta)[i] * (*pdTheta)[j];

    /* increment pIter */
    *pIter = *pIter + 1;

  } /* end while */

  /* note that the theta returned is the last parameter set read */

  fclose (pfileRestart);

} /* ReadRestartTemper */


/* Function -------------------------------------------------------------------
   RestoreLikelihoods

   Called from TraverseLevels1
*/
int RestoreLikelihoods (PLEVEL plevel, char **args)
{
  PEXPERIMENT pExpt = plevel->pexpt;

  if (pExpt != NULL) {
    pExpt->dLnLike = pExpt->dLnLikeSave;
  }

  return (1);

} /* RestoreLikelihoods */


/* Function -------------------------------------------------------------------
   RunAllExpts

   Run all experiments (i.e. experiments at all levels below
   panal->pLevels[0]).
*/
int RunAllExpts (PANALYSIS panal, PDOUBLE pdLnData)
{
  PLEVEL plevel0 = panal->pLevels[0];
  long n;

  for (n = 0; n < plevel0->iInstances; n++) {
    if (!TraverseLevels1 (plevel0->pLevels[n], RunExpt, panal,
                          pdLnData, NULL)) {
      /* error */
      return(0);
    }
  }

  return(1);

} /* RunAllExpts */


/* Function -------------------------------------------------------------------
   RunExpt

   If `plevel' has experiments, modify the variables that have been sampled
   at this level and above using its two lists, and run the experiments.
   Return 1 on success and 0 if failure.
   Computes the data likelihood in case of success.

   Called from TraverseLevels1
*/
int RunExpt (PLEVEL plevel, char **args)
{
  PANALYSIS   panal = (PANALYSIS) args[0];
  double      *pdLnData = (double *) args[1];
  long        i;
  PEXPERIMENT pExpt = plevel->pexpt;

  /* Set level sequence */
  panal->pCurrentLevel[plevel->iDepth] = plevel;

  if (pExpt != NULL) {
    InitModel ();

    /* Set the model vars that have been sampled in this level and above */
    for (i = 0; i <= plevel->iDepth; i++) {
      SetModelVars (panal->pCurrentLevel[i]);
      SetFixedVars (panal->pCurrentLevel[i]);
    }

    if (!DoOneExperiment (pExpt)) {
      /* Error */
      printf ("Warning: DoOneExperiment failed\n");
      return 0;
    }
    else {
      pExpt->dLnLike = LnLikeData (plevel, panal);
      *pdLnData = (*pdLnData) + pExpt->dLnLike;
    }
  } /* if */

  return (1);

} /* RunExpt */


/* Function -------------------------------------------------------------------
   SampleTemperature
*/
long SampleTemperature (PGIBBSDATA pgd, double dLnPrior, double dLnData)
{
  int    indexT = pgd->indexT;
  int    indexT_new;

  /* Propose a new perk */
  if (indexT == 0) indexT_new = 1;
  else {
    if (indexT == pgd->nPerks - 1) indexT_new = indexT - 1;
    else {
      if (Randoms() > 0.5) indexT_new = indexT + 1;
        else indexT_new = indexT - 1;
    }
  }

  /* Test the temperature */
  if (TestTemper (pgd, indexT, indexT_new, dLnPrior, dLnData,
                  pgd->rgdlnPi[indexT], pgd->rgdlnPi[indexT_new])) {
    /* jump */
    return (indexT_new);
  }
  else
    return (indexT);

} /* SampleTemperature */


/* Function -------------------------------------------------------------------
   SampleTemperature2

   Records the count of attempted and accepted temperature jumps
*/
long SampleTemperature2 (PGIBBSDATA pgd, double dLnPrior, double dLnData)
{
  int indexT = pgd->indexT;
  int indexT_new;

  /* Propose a new perk */
  if (indexT == pgd->startT) indexT_new = indexT + 1;
  else {
    if (indexT == pgd->endT) indexT_new = indexT - 1;
    else {
      if (Randoms() > 0.5) indexT_new = indexT + 1;
        else indexT_new = indexT - 1;
    }
  }

  int minI = (indexT < indexT_new ? indexT : indexT_new);
  pgd->rglTransAttempts[minI]++;

  /* Test the temperature */
  if (TestTemper (pgd, indexT, indexT_new, dLnPrior, dLnData,
                  pgd->rgdlnPi[indexT], pgd->rgdlnPi[indexT_new])) {
    /* jump */
    pgd->rglTransAccepts[minI]++;
    return (indexT_new);
  }
  else
    return (indexT);

} /* SampleTemperature2 */


/* Function -------------------------------------------------------------------
   SampleTheta, samples from a normal kernel
*/
double SampleTheta (PMCVAR pMCVar) {

  /* if the parameter has a discrete distribution, round it - FB 12/06/97 */
  if (pMCVar->iType == MCV_BINOMIAL || pMCVar->iType == MCV_POISSON) {
   return floor (0.5 + TruncNormalRandom (pMCVar->dVal, pMCVar->dKernelSD,
                                          MinMCVar(pMCVar),
                                          MaxMCVar(pMCVar)));
  }
  else { /* FB fixed the uniform case - FB 30/06/97 */
    return TruncNormalRandom (pMCVar->dVal, pMCVar->dKernelSD,
                              MinMCVar(pMCVar), MaxMCVar(pMCVar));
  }

} /* SampleTheta */


/* Function -------------------------------------------------------------------
   SampleThetaUnif, samples from a uniform kernel
*/
double SampleThetaUnif (PMCVAR pMCVar) {

  /* if the parameter has a discrete distribution, round it */
  if (pMCVar->iType == MCV_BINOMIAL || pMCVar->iType == MCV_POISSON)
    return floor (0.5 + UniformRandom (MinMCVar(pMCVar), MaxMCVar(pMCVar)));
  else
    return UniformRandom (MinMCVar(pMCVar), MaxMCVar(pMCVar));

} /* SampleThetaUnif */


/* Function -------------------------------------------------------------------
   SampleThetas

   Perform a Metropolis step on all the random (MC) variables at the
   level passed as argument 1.
   Sample thetas in sequence - test using prior and likelihood -
   restore old values if necessary - write new values to output file.

   Called from TraverseLevels
*/
void SampleThetas (PLEVEL plevel, char **args)
{
  PANALYSIS  panal = (PANALYSIS) args[0];
  PGIBBSDATA pgd = (PGIBBSDATA) args[1];
  long *pnIter = (long *) args[2];
  long *pnUpdateAt = (long *) args[3];
  long *pnTotal = (long *) args[4];

  double dLnPrior, dLnLike, dLnData, dLnKern;
  double dLnPriorNew, dLnLikeNew, dLnDataNew, dLnKernNew;
  double dTheta, dJumps;
  PMCVAR pMCVar;
  long   n;

  /* Set level sequence. This is needed to call RunExpt from the middle
     of the tree with the right initialization of the nodes above */
  panal->pCurrentLevel[plevel->iDepth] = plevel;

  /* For all MC vars at this level */
  for (n = 0; n < plevel->nMCVars; n++) {

    pMCVar = plevel->rgpMCVars[n];

    /* If the MC var is fixed, no sampling is made, just write it out */
    if (pMCVar->bIsFixed)
      goto WriteIt;

    /* Compute prior and likelihood */
    dLnPrior = LnDensity (pMCVar, panal);
    dLnLike  = LnLike (pMCVar, panal);

    dLnData = 0.0;

    /* If data are dependent compute the data likelihood */
    if (pMCVar->bExptIsDep) {
      /* Form the likelihood of all experiments at this level or beneath.*/
      TraverseLevels1 (plevel, SumAllExpts, &dLnData, NULL);
    }

    /* Save current value */
    dTheta = pMCVar->dVal;

    /* Adjust the jumping kernel SD, depending on acceptance rates,
       make sure it does not exceed DBL_MAX or a third of the range */
    if (*pnIter == *pnUpdateAt) {

      dJumps = (double) pMCVar->lJumps / (double) (*pnTotal);

      if (dJumps > 0.3) { /* we will increase the kernel spread */
        if (dJumps == 1) {
          /* pathological case: chances are the parameter has such a
             small kernel that it does not change values, this leads
             to a "jump" each time. Drastic increase of the kernel is
             attempted */
          if (pMCVar->dKernelSD < sqrt(DBL_MAX)) {
            if (pMCVar->dKernelSD > 2)
              pMCVar->dKernelSD = pMCVar->dKernelSD * pMCVar->dKernelSD;
            else /* FB 28/03/1999 */
              pMCVar->dKernelSD = pMCVar->dKernelSD * 20;
          }
          else
            pMCVar->dKernelSD = pMCVar->dMaxKernelSD;
        }
        else { /* more normal case */
          if (pMCVar->dKernelSD < DBL_MAX / 2)
            pMCVar->dKernelSD = pMCVar->dKernelSD * 2;
          else
            pMCVar->dKernelSD = pMCVar->dMaxKernelSD;
        }

        /* check that kernel SD does not increase wildly */
        if (pMCVar->dKernelSD > pMCVar->dMaxKernelSD)
          pMCVar->dKernelSD = pMCVar->dMaxKernelSD;
      }
      else { /* we will decrease the kernel spread */
        if (dJumps == 0) {
          /* pathological case: chances are the parameter has such a
             big kernel that it will never jump. Drastic decrease of
             the kernel is attempted, dissymetric from the increase */
          if (pMCVar->dKernelSD > pow(DBL_MIN, 0.45)) {
            if (pMCVar->dKernelSD > 2) { /* FB 28/03/1999 */
              pMCVar->dKernelSD = pow(pMCVar->dKernelSD, 0.45);
            }
            else {
              pMCVar->dKernelSD = pMCVar->dKernelSD * 0.04;
            }
          }
          else
            pMCVar->dKernelSD = DBL_MIN;
        }
        else { /* more normal case */
          /* the kernel should not be infinitely decreased; decrease
             is otherwise slighly different from the increase to avoid
             oscillations */
          if (pMCVar->dKernelSD > DBL_MIN / 0.4)
            pMCVar->dKernelSD = pMCVar->dKernelSD * 0.4;
          else
            pMCVar->dKernelSD = DBL_MIN;
        }
      }

      pMCVar->lJumps = 0; /* reset the jumps counter */

    } /* end of kernel stuff */

    /* compute the integral of the jump kernel */
    dLnKern  = log(CDFNormal ((MaxMCVar(pMCVar) - pMCVar->dVal) /
                              pMCVar->dKernelSD) -
                   CDFNormal ((MinMCVar(pMCVar) - pMCVar->dVal) /
                              pMCVar->dKernelSD));

    /* sample a new value */
    pMCVar->dVal = SampleTheta (pMCVar);

    /* update the integral of the jump kernel */
    dLnKernNew  =  log(CDFNormal ((MaxMCVar(pMCVar) - pMCVar->dVal) /
                                  pMCVar->dKernelSD) -
                       CDFNormal ((MinMCVar(pMCVar) - pMCVar->dVal) /
                                  pMCVar->dKernelSD));

    /* update prior and likelihood */
    dLnPriorNew = LnDensity (pMCVar, panal);
    dLnLikeNew  = LnLike (pMCVar, panal);

    dLnDataNew  = 0.0;

    /* If data are dependent compute the data likelihood */
    if (pMCVar->bExptIsDep) {
      /* Run all experiments at this level or beneath.
         We should in fact run only the dependent experiments ! */

      if (!TraverseLevels1 (plevel, RunExpt, panal, &dLnDataNew, NULL)) {
        /* If running experiments fails, do not jump */
        pMCVar->dVal = dTheta;
        TraverseLevels1 (plevel, RestoreLikelihoods, NULL);
        goto WriteIt;
      }
    }

    /* Test the results and act accordingly */
    if (!TestImpRatio (pgd, pMCVar->bExptIsDep, dLnKern, dLnKernNew,
                       dLnPrior, dLnPriorNew,
                       dLnLike, dLnLikeNew, dLnData, dLnDataNew)) {
      /* reject, restore */
      pMCVar->dVal = dTheta;
      if (pMCVar->bExptIsDep)
        TraverseLevels1 (plevel, RestoreLikelihoods, NULL);
    }
    else {
      /* accept, save likelihoods */
      pMCVar->lJumps = pMCVar->lJumps + 1;

      if(pMCVar->bExptIsDep)
        TraverseLevels1 (plevel, SaveLikelihoods, NULL);
    }

    /* calculate the running mean and variance for this value */
    CalculateMeanAndVariance((*pnIter+1),pMCVar->dVal,&pMCVar->dVal_mean,
                                                  &pMCVar->dVal_var);

    WriteIt: /* Write the MC var value to output file */

    if (((*pnIter+1) % pgd->nPrintFreq == 0) &&
        (*pnIter >= pgd->nMaxIter - pgd->nPrintIter)) {
      fprintf(pgd->pfileOut, "%5g\t", pMCVar->dVal);
    }
  } /* end for all pMCVar */

} /* SampleThetas */


/* Function -------------------------------------------------------------------
   SampleThetasTempered

   Use annealing MCMC
   Sample thetas in sequence - test using prior and likelihood -
   restore old values if necessary - write new values to output file
   Called from TraverseLevels

*/
void SampleThetasTempered (PLEVEL plevel, char **args)
{
  PANALYSIS  panal = (PANALYSIS) args[0];
  PGIBBSDATA pgd = (PGIBBSDATA) args[1];
  long *pnIter = (long *) args[2];
  long *pnUpdateAt = (long *) args[3];
  long *pnTotal = (long *) args[4];
  long *pindexT = (long *) args[5];

  double dLnPrior, dLnLike, dLnData, dLnKern;
  double dLnPriorNew, dLnLikeNew, dLnDataNew, dLnKernNew;
  double dTheta, dJumps, old_dKernelSD;
  PMCVAR pMCVar;
  long   n;

  /* Set level sequence. This is needed to call RunExpt from the middle
     of the tree with the right initialization of the nodes above */
  panal->pCurrentLevel[plevel->iDepth] = plevel;

  /* For all MC vars at this level */
  for (n = 0; n < plevel->nMCVars; n++) {

    pMCVar = plevel->rgpMCVars[n];

    /* If the MC var is fixed, no sampling is made, just write it out */
    if (pMCVar->bIsFixed)
      goto WriteIt;

    /* Compute prior and likelihood */
    dLnPrior = LnDensity (pMCVar, panal);
    dLnLike = LnLike (pMCVar, panal);

    dLnData = 0.0;

    /* If data are dependent compute the data likelihood */
    if (pMCVar->bExptIsDep) {
      /* Form the likelihood of all experiments at this level or beneath.*/
      TraverseLevels1 (plevel, SumAllExpts, &dLnData, NULL);
    }

    /* Save current value */
    dTheta = pMCVar->dVal;

    /* Adjust the jumping kernel SD, depending on acceptance rates,
       make sure it does not exceed DBL_MAX or a third of the range */
    if (*pnIter == *pnUpdateAt) {

      dJumps = (double) pMCVar->lJumps / (double) (*pnTotal);

      if (dJumps > 0.3) { /* we will increase the kernel spread */
        if (dJumps == 1) {
          /* pathological case: chances are the parameter has such a
             small kernel that it does not change values, this leads
             to a "jump" each time. Drastic increase of the kernel is
             attempted */
          if (pMCVar->dKernelSD < sqrt(DBL_MAX)) {
            if (pMCVar->dKernelSD > 2)
              pMCVar->dKernelSD = pMCVar->dKernelSD * pMCVar->dKernelSD;
            else /* FB 28/03/1999 */
              pMCVar->dKernelSD = pMCVar->dKernelSD * 20;
          }
          else
            pMCVar->dKernelSD = DBL_MAX;
        }
        else { /* more normal case */
          if (pMCVar->dKernelSD < DBL_MAX / 2)
            pMCVar->dKernelSD = pMCVar->dKernelSD * 2;
          else
            pMCVar->dKernelSD = DBL_MAX;
        }

        /* check that kernel SD does not increase wildly */
        if (pMCVar->dKernelSD > pMCVar->dMaxKernelSD)
          pMCVar->dKernelSD = pMCVar->dMaxKernelSD;
      }
      else { /* we will decrease the kernel spread */
        if (dJumps == 0) {
          /* pathological case: chances are the parameter has such a
             big kernel that it will never jump. Drastic decrease of
             the kernel is attempted, dissymetric from the increase */
          if (pMCVar->dKernelSD > pow(DBL_MIN, 0.45)) {
            if (pMCVar->dKernelSD > 2) { /* FB 28/03/1999 */
              pMCVar->dKernelSD = pow(pMCVar->dKernelSD, 0.45);
            }
            else {
              pMCVar->dKernelSD = pMCVar->dKernelSD * 0.04;
            }
          }
          else
            pMCVar->dKernelSD = DBL_MIN;
        }
        else { /* more normal case */
          /* the kernel should not be infinitely decreased; decrease
             is otherwise slighly different from the increase to avoid
             oscillations */
          if (pMCVar->dKernelSD > DBL_MIN / 0.4)
            pMCVar->dKernelSD = pMCVar->dKernelSD * 0.4;
          else
            pMCVar->dKernelSD = DBL_MIN;
        }
      }

      pMCVar->lJumps = 0; /* reset the jumps counter */

    } /* end kernel updating */

    /* sample a new value */

    if (pgd->rgdPerks[*pindexT] > 0) { /* usual case */

      /* first scale temporarily the kernelSD according to the temperature */

      /* save the current kernelSD */
      old_dKernelSD = pMCVar->dKernelSD;

      /* update the kernelSD according to the temperature, the formula
         comes from that of the variance of the powered standard normal */
      pMCVar->dKernelSD = pMCVar->dKernelSD *
                          exp((1 - pgd->rgdPerks[*pindexT]) * LN2PI * 0.25 -
                              0.75 * log(pgd->rgdPerks[*pindexT]));

      /* check that kernel SD does not increase wildly */
      if (pMCVar->dKernelSD > pMCVar->dMaxKernelSD)
        pMCVar->dKernelSD = pMCVar->dMaxKernelSD;

      /* compute the integral of the jump kernel (for truncation) */
      dLnKern  = log(CDFNormal((MaxMCVar(pMCVar) - pMCVar->dVal) /
                               pMCVar->dKernelSD) -
                     CDFNormal((MinMCVar(pMCVar) - pMCVar->dVal) /
                               pMCVar->dKernelSD));

      /* sample a new value */
      pMCVar->dVal = SampleTheta (pMCVar);

      /* update the integral of the jump kernel (for truncation) */
      dLnKernNew  =  log(CDFNormal((MaxMCVar(pMCVar) - pMCVar->dVal) /
                                   pMCVar->dKernelSD) -
                         CDFNormal((MinMCVar(pMCVar) - pMCVar->dVal) /
                                   pMCVar->dKernelSD));
    }
    else {
      /* inverse temperature is zero, sample uniformly or from the prior,
         the jump will always be accepted */
      if (pgd->nSimTypeFlag == 3) { /* the posterior is tempered */
        /* sample a new value uniformly in its range */
        pMCVar->dVal = SampleThetaUnif(pMCVar);
      }
      else { /* the likelihood is tempered, sample from the prior */
        CalculateOneMCParm(pMCVar);
      }
    }

    /* recompute prior and likelihood */
    dLnPriorNew = LnDensity (pMCVar, panal);
    dLnLikeNew  = LnLike (pMCVar, panal);

    dLnDataNew = 0.0;

    /* If data are dependent compute the data likelihood */
    if (pMCVar->bExptIsDep) {
      /* Run all experiments beneath this level.
         We should in fact run only the dependent experiments ! */
      if (!TraverseLevels1 (plevel, RunExpt, panal, &dLnDataNew, NULL)) {
        /* If running experiments fails, do not jump */
        pMCVar->dVal = dTheta; /* restore */
        TraverseLevels1 (plevel, RestoreLikelihoods, NULL);
        goto WriteIt;
      }
    }

    /* Test the results and act accordingly */
    if (!TestImpRatioTemper (pgd, pMCVar->bExptIsDep, dLnKern, dLnKernNew,
                             dLnPrior, dLnPriorNew,
                             dLnLike, dLnLikeNew, dLnData, dLnDataNew,
                             *pindexT)) {
      /* reject, restore */
      pMCVar->dVal = dTheta;
      if (pMCVar->bExptIsDep)
        TraverseLevels1 (plevel, RestoreLikelihoods, NULL);
    }
    else {
      /* accept, save likelihoods */
      pMCVar->lJumps = pMCVar->lJumps + 1;
      if (pMCVar->bExptIsDep)
        TraverseLevels1 (plevel, SaveLikelihoods, NULL);
    }

    if (pgd->rgdPerks[*pindexT] > 0) /* restore kernelSD */
      pMCVar->dKernelSD = old_dKernelSD;

WriteIt: /* Write the current MC var value to output file  */

    if (((*pnIter+1) % pgd->nPrintFreq == 0) &&
      (*pnIter >= pgd->nMaxIter - pgd->nPrintIter)) {
      fprintf(pgd->pfileOut, "%5g\t", pMCVar->dVal);
    }
  } /* Metrolopolis-Hastings */

} /* SampleThetasTempered */


/* Function -------------------------------------------------------------------
   SampleThetaVector

   Sample thetas in block, do Metropolis test.
*/
void SampleThetaVector (PLEVEL pLevel, PANALYSIS panal, long nThetas,
                        double *pdTheta, double *pdSum, double **prgdSumProd,
                        long iter, long nUpdateAt, long nTotal,
                        PDOUBLE pdLnPrior, PDOUBLE pdLnData)
{
  register long i, j;
  double dTmp, dAccept, dLnPrior_old, dLnData_old;
  BOOL   bInBounds;

  static long lAccepted = 0;
  static double dJumpSpread;
  static PDOUBLE pdTheta_old = NULL; /* previous model parameters values */
  static PDOUBLE *prgdComponent;
  static PDOUBLE *prgdVariance;
  static PDOUBLE dNormVar; /* storage for nParms normal deviates */

  if ((pdTheta_old == NULL) || (iter == nUpdateAt)) {

    if (pdTheta_old == NULL) { /* initialize */
      if ( !(pdTheta_old = InitdVector (nThetas)) ||
           !(dNormVar    = InitdVector (nThetas)) ||
           !(prgdVariance  = InitdMatrix (nThetas, nThetas)) ||
           !(prgdComponent = InitdMatrix (nThetas, nThetas)))
        ReportRunTimeError (panal, RE_OUTOFMEM | RE_FATAL,
                            "SampleThetaVector");

      /* initialize dJumpSpread */
      dJumpSpread = 2.4 / sqrt(nThetas); /* Gelman's Normal theory result */
    }
    else {
      /* done if iter == nUpdateAt, but not at start:
         if some vector samplings have been made, check that the current
         dJumpSpread leads to an acceptation rate of 15 to 30% over the
         last batch of simulations. Adjust eventually. */
      dAccept = ((double) lAccepted) / (double) (nTotal);

      if ( dAccept > 0.3) dJumpSpread = dJumpSpread * 1.5;
      else if (dAccept < 0.15) dJumpSpread = dJumpSpread * 0.7;

      printf ("Monitoring: iter\t%ld\t", iter);
      printf ("success rate\t%g\tspread\t%g\n", dAccept, dJumpSpread);
      lAccepted = 0; /* reset the counter */
    }

    /* other updates: */

    /* generate the covariance matrix */
    for (i = 0; i < nThetas; i++)
      for (j = 0; j < nThetas; j++)
        prgdVariance[i][j] = (prgdSumProd[i][j] -
                              pdSum[i] * pdSum[j] / (double) (iter+1)) /
                             (double) iter;

    /* do the Cholesky decomposition of the covariance matrix */
    if (!Cholesky (prgdVariance, prgdComponent, nThetas)) {
      /* try to save the computation by zeroing all non-diagonal elements */
      for (i = 0; i < nThetas; i++)
        for (j = 0; j < nThetas; j++) {
          if (i == j)
            prgdVariance[i][j] = prgdSumProd[i][j] / (double) (iter);
          else
            prgdVariance[i][j] = 0.0;
        }

        /* if it still does not work, exit */
      if (!Cholesky (prgdVariance, prgdComponent, nThetas)) {
        printf ("Error: impossible to compute a jumping kernel - Exiting."
                "(You should check or change the restart file).\n\n");
        exit (0);
      }
    }
  }

  /* keep the value of all thetas */
  for (i = 0; i < nThetas; i++)
    pdTheta_old[i] = pdTheta[i];

  /* keep old prior and old likelihood */
  dLnPrior_old = *pdLnPrior;
  dLnData_old  = *pdLnData;

  /* generate new pdTheta vector */
  for (i = 0; i < nThetas; i++)
    dNormVar[i] = NormalRandom(0.0, 1.0);

  for (i = 0; i < nThetas; i++) {
    dTmp = 0;
    for (j = 0; j <= i; j++) /* only the non-zero part of prgdComponent */
      dTmp = dTmp + dNormVar[j] * prgdComponent[i][j];

    pdTheta[i] = pdTheta_old[i] + dJumpSpread * dTmp;
  }

  /* Set the dVals of the variables to the values sampled and check that
     we are within bounds */
  long iTmp = 0; /* use a dummy variable to avoid resetting nThetas */
  bInBounds = TraverseLevels1 (pLevel, SetMCVars, pdTheta, &iTmp, NULL);

  if (!bInBounds) { /* reject */
    for (i = 0; i < nThetas; i++)
      pdTheta[i] = pdTheta_old[i]; /* restore */
    goto DontJump;
  }

  /* Calculate the new prior */
  *pdLnPrior = 0.0;
  TraverseLevels (pLevel, CalculateTotals, panal, pdLnPrior, NULL);

  /* compute the model at the newly drawn point and the likelihood */
  *pdLnData = 0.0;
  if (!RunAllExpts (panal, pdLnData)) {
    /* computation failed, don't jump, restore */
    for (i = 0; i < nThetas; i++)
      pdTheta[i] = pdTheta_old[i];
    *pdLnPrior = dLnPrior_old;
    *pdLnData  = dLnData_old;
  }
  else {
    /* Test */
    if (log(Randoms()) >
        ((*pdLnPrior) + (*pdLnData) - dLnPrior_old - dLnData_old)) {
      /* don't jump, restore */
      for (i = 0; i < nThetas; i++)
        pdTheta[i] = pdTheta_old[i];
      *pdLnPrior = dLnPrior_old;
      *pdLnData  = dLnData_old;
    }
    else { /* jump */
      lAccepted++; /* this is used above to adjust the acceptation rate */
    }
  }

  DontJump:

  /* update arrays */
  for (i = 0; i < nThetas; i++) {
    pdSum[i] += pdTheta[i];
    for (j = 0; j < nThetas; j++)
      prgdSumProd[i][j] += pdTheta[i] * pdTheta[j];
  }

} /* SampleThetaVector */


/* Function -------------------------------------------------------------------
   SaveLikelihoods

   Called from TraverseLevels1
*/
int SaveLikelihoods (PLEVEL plevel, char **args)
{
  PEXPERIMENT pExpt = plevel->pexpt;

  if (pExpt != NULL) {
    pExpt->dLnLikeSave = pExpt->dLnLike;
  }

  return (1);

} /* SaveLikelihoods */


/* Function -------------------------------------------------------------------
   SetFixedVars

   Set the array of fixed variables
*/
void SetFixedVars (PLEVEL plevel)
{
  long n;
  PVARMOD pFVar;

  for (n = 0; n < plevel->nFixedVars; n++) {
    pFVar = plevel->rgpFixedVars[n];
    if (IsInput (pFVar->hvar))
      SetInput (pFVar->hvar, pFVar->uvar.pifn);
    else
      SetVar (pFVar->hvar, pFVar->uvar.dVal);
  }

} /* SetFixedVars */


/* Function -------------------------------------------------------------------
   SetKernel

   Set initial values of the MCMC jumping kernel and eventually
   initializes the sampled parameters (contained in plevel->rgpMCVars[]).

   The first argument of the list is a flag indicating whether the
   values of the sampled parameters should be restored (to the values
   passed as a second arument in an array) (case 1), or left at their
   last sampled value (case 2).
*/
void SetKernel (PLEVEL plevel, char **args)
{
  intptr_t useMCVarVals = (intptr_t) args[0]; /* 1 to restore dVals, else 2 */
  double *pdMCVarVals = (double *) args[1];
  double dMin, dMax, dTmp;
  long   n, m;
  static long nThetas;
  PMCVAR pMCVar;

  /* set the jumping kernel's SD: sample 4 variates and take the range */
  for (n = 0; n < plevel->nMCVars; n++) {

    if (!(plevel->rgpMCVars[n]->bIsFixed)) {

      pMCVar = plevel->rgpMCVars[n];
      CalculateOneMCParm (pMCVar);

      /* set the maximum kernel SD value to about half of the SD of a 
         uniform distribution having the same range */
      if (pMCVar->iType == MCV_UNIFORM || pMCVar->iType == MCV_LOGUNIFORM )
        pMCVar->dMaxKernelSD = (*(pMCVar->pdParm[1]) / 6.0) -
                               (*(pMCVar->pdParm[0]) / 6.0);
      else
        pMCVar->dMaxKernelSD = (*(pMCVar->pdParm[3]) / 6.0) -
                               (*(pMCVar->pdParm[2]) / 6.0);

      /* we want a kernel SD compatible with the SD of the prior; this could
         be done analytically, depending on the prior. We approximate it by
         sampling 4 variates from the prior */
      dMin = dMax = pMCVar->dVal;
      for (m = 0; m < 3; m++) {
        CalculateOneMCParm(pMCVar);
        dTmp = pMCVar->dVal;
        if (dMin >= dTmp) dMin = dTmp;
        else if (dMax < dTmp) dMax = dTmp;
      }

      /* set the range safely because max - min could be too large */
      if ((*(pMCVar->pdParm[2]) == -DBL_MAX) ||
          (*(pMCVar->pdParm[3]) ==  DBL_MAX))
        pMCVar->dKernelSD = (0.5 * dMax) - (0.5 * dMin);
      else
        pMCVar->dKernelSD = dMax - dMin;

      /* take care of the case in which the range is zero
         (can happens for discrete variables) */
      if (pMCVar->dKernelSD == 0) {
        pMCVar->dKernelSD = pMCVar->dMaxKernelSD;
      }

      /* check that we do not exceed the maximum SD */
      if (pMCVar->dKernelSD > pMCVar->dMaxKernelSD) {
        pMCVar->dKernelSD = pMCVar->dMaxKernelSD;
      }
    }

    /* restore the value of the variable - FB 02/07/97 */
    if (useMCVarVals == 1)
      plevel->rgpMCVars[n]->dVal = pdMCVarVals[nThetas++];

  }

} /* SetKernel */


/* Function -------------------------------------------------------------------
   WriteKernel

   Write values of the MCMC jumping kernel
*/
void WriteKernel (PLEVEL plevel, char **args)
{
  FILE   *pfile = (FILE *) args[0];
  long   n;
  PMCVAR pMCVar;

  for (n = 0; n < plevel->nMCVars; n++) {
    if ( !(plevel->rgpMCVars[n]->bIsFixed)) {
      pMCVar = plevel->rgpMCVars[n];
      fprintf(pfile, "%lg\t", pMCVar->dKernelSD);
    }
  }

} /* WriteKernel */


/* Function -------------------------------------------------------------------
   ReadKernel

   Read values of the MCMC jumping kernel
*/
void ReadKernel (PLEVEL plevel, char **args)
{
  FILE   *pfile = (FILE *) args[0];
  long   n;
  PMCVAR pMCVar;

  for (n = 0; n < plevel->nMCVars; n++) {
    if ( !(plevel->rgpMCVars[n]->bIsFixed)) {
      pMCVar = plevel->rgpMCVars[n];

      /* set the maximum kernel SD value */
      pMCVar->dMaxKernelSD = (MaxMCVar(pMCVar) - MinMCVar(pMCVar)) / 6.0;

      if (!fscanf(pfile, "%lg", &(pMCVar->dKernelSD))) {
        ReportError (NULL, RE_READERROR | RE_FATAL, "kernel file", NULL);
      }
    }
  }

} /* ReadKernel */


/* Function -------------------------------------------------------------------
   SetModelVars

   Sets the array of model variables to the sampled values. Does not set fixed
   variables. That has to be done by SetFixedVars.
*/
void SetModelVars(PLEVEL plevel)
{
  long n;
  PMCVAR  pMCVar;

  for (n = 0; n < plevel->nMCVars; n++) {
    pMCVar = plevel->rgpMCVars[n];
    if (!(pMCVar->bIsFixed) && (IsParm (pMCVar->hvar)))
      SetVar (pMCVar->hvar, pMCVar->dVal);
  }

} /* SetModelVars */


/* Function -------------------------------------------------------------------
   SetMCVars

   Set initial values of thetas after reading input file or sampling them.
   Values are assumed to be in proper order.
   It also checks that the ranges are respected.
   Called from TraverseLevels.
*/
int SetMCVars (PLEVEL plevel, char **args)
{
  double *pdMCVarVals = (double *) args[0];
  long   *nThetas = (long *) args[1];
  PMCVAR pMCVar;
  double dVar;
  long   n;

  for (n = 0; n < plevel->nMCVars; n++) {
    dVar = pdMCVarVals[(*nThetas)++];
    pMCVar = plevel->rgpMCVars[n];
    if ((pMCVar->iType == MCV_UNIFORM) || (pMCVar->iType == MCV_LOGUNIFORM)) {
      if ((dVar < *(pMCVar->pdParm[0])) || (dVar > *(pMCVar->pdParm[1]))) {
        /* error */
        return (0);
      }
    }
    else {
      if ((dVar < *(pMCVar->pdParm[2])) || (dVar > *(pMCVar->pdParm[3]))) {
        /* error */
        return (0);
      }
    }

    pMCVar->dVal = dVar;
  }

  /* success */
  return (1);

} /* SetMCVars */


/* Function -------------------------------------------------------------------
   SetPointers

   Called from TraverseLevels

   FB 26 nov 96: For each Monte Carlo variable, pdParms are set to point to the
   parent's dVals rather than to model parameters. If there is no parent,
   pdParms point to their own dParms.
   The pdVals and pdParms of the data and output arrays are also set.
*/
void SetPointers (PLEVEL plevel, char **args)
{
  long i, j, k;
  PMCVAR pMCVar;
  POUTSPEC pos;
  BOOL bFound;

  for (i = 0; i < plevel->nMCVars; i++) { /* for parameters */
    pMCVar = plevel->rgpMCVars[i];

    /* For each distribution parameter */
    for (j = 0; j < 4; j++) {
      if (pMCVar->pMCVParent[j] == NULL) /* Point to its own values */
        pMCVar->pdParm[j] = &(pMCVar->dParm[j]);
      else /* Point to the parent dVal */
        pMCVar->pdParm[j] = &(pMCVar->pMCVParent[j]->dVal);
    }
  }

  if (plevel->pexpt != NULL) /* if the level has experiments */
  for (i = 0; i < plevel->nLikes; i++) { /* for likelihoods */
    pMCVar = plevel->rgpLikes[i];
    pos = &(plevel->pexpt->os);

    /* Set pdVal of pMCVar to a data array, first find it */
    bFound = FALSE;
    j = 0;
    while ((j < pos->nData) && (!bFound)) {
      bFound = (pMCVar->hvar == pos->phvar_dat[j]);
      if (!bFound) j++;
    }
    if (bFound) {
      pMCVar->pdVal = pos->prgdDataVals[j];
      pMCVar->lCount = pos->pcData[j]; /* number of data points */
    }
    else { /* no corresponding Data found: error */
      printf ("Error: no Data for %s in Simulation %d - Exiting.\n\n",
              pMCVar->pszName, plevel->pexpt->iExp);
      exit (0);
    }

    /* we also have to set the pdParms of pMCVar to either data or output
       arrays */
    for (j = 0; j < 4; j++) {

      if (pMCVar->iParmType[j] == MCVP_PRED) { /* it's an output */
        /* set pdParm[j] to an output, first find it */
        bFound = FALSE;
        k = 0;
        while ((k < pos->nOutputs) && (!bFound)) {
          bFound = (pMCVar->hParm[j] == pos->phvar_out[k]);
          if (!bFound) k++;
        }
        if (bFound) {
          pMCVar->pdParm[j] = &(pos->prgdOutputVals[k][0]);
        }
        else { /* no corresponding Print statement found: error */
          printf ("Error: missing Print statement for parameter number %ld\n"
                  "of %s distribution - Exiting.\n\n", j, pMCVar->pszName);
          exit (0);
        }
      } /* end if MCVP_PRED */

      else if (pMCVar->iParmType[j] == MCVP_DATA) { /* it's data */
        /* set pdParm[j] to a data array, first find it */
        bFound = FALSE;
        k = 0;
        while ((k < pos->nData) && (!bFound)) {
          bFound = (pMCVar->hParm[j] == pos->phvar_dat[k]);
          if (!bFound) k++;
        }
        if (bFound) {
          pMCVar->pdParm[j] = &(pos->prgdDataVals[k][0]);
        }
        else { /* no Data found: error */
          printf ("Error: no Data for %s in Simulation %d - Exiting.\n\n",
                  pMCVar->pszName, plevel->pexpt->iExp);
          exit (0);
        }
      } /* end if MCVP_DATA */

      else { /* it's either a parameter or a numeric */
        if (pMCVar->pMCVParent[j] == NULL) /* Point to its own values */
          pMCVar->pdParm[j] = &(pMCVar->dParm[j]);
        else /* Point to the parent dVal */
          pMCVar->pdParm[j] = &(pMCVar->pMCVParent[j]->dVal);
      }
    } /* end for j */
  }

} /* SetPointers */


/* Function -------------------------------------------------------------------
   SumAllExpts

   If `plevel' has experiments, add the current Likelihood to the total
   passed as argument 2.

   Called from TraverseLevels1
*/
int SumAllExpts (PLEVEL plevel, char **args)
{
  double      *pdLnData = (double*)args[0];
  PEXPERIMENT pExpt = plevel->pexpt;

  if (pExpt != NULL) {
    *pdLnData += pExpt->dLnLike;
  }
  return (1);

} /* SumAllExpts */


/* Function -------------------------------------------------------------------
   TestImpRatio

   Test prior, likelihood against random number between 0 and 1
*/
BOOL TestImpRatio (PGIBBSDATA pgd,  BOOL bExptIsDep,
                   double dLnKern,  double dLnKernNew,
                   double dLnPrior, double dLnPriorNew,
                   double dLnLike,  double dLnLikeNew,
                   double dLnData,  double dLnDataNew)
{
  double dPjump;

  if (dLnKernNew  == NULL_SUPPORT ||
      dLnPriorNew == NULL_SUPPORT ||
      dLnLikeNew  == NULL_SUPPORT ||
      dLnDataNew  == NULL_SUPPORT)
    return FALSE;

  dPjump = dLnPriorNew - dLnPrior + dLnLikeNew - dLnLike +
           dLnKern - dLnKernNew;

  if (bExptIsDep)
    dPjump += dLnDataNew - dLnData;

  if (pgd->nSimTypeFlag == 0)
    return ((BOOL) (log(Randoms()) <= dPjump));
  else {
    if (pgd->nSimTypeFlag == 5)
      return ((BOOL) (0 <= dPjump));
    else {
      printf ("Error: simTypeFlag should be 0 or 5 in TestImpRatio "
              "- Exiting.\n\n");
      exit (0);
    }
  }

} /* TestImpRatio */


/* Function -------------------------------------------------------------------
   TestImpRatioTemper

   Test prior, likelihood against a random number between 0 and 1 according to
   the temperature
*/
BOOL TestImpRatioTemper (PGIBBSDATA pgd,  BOOL bExptIsDep,
                         double dLnKern,  double dLnKernNew,
                         double dLnPrior, double dLnPriorNew,
                         double dLnLike,  double dLnLikeNew,
                         double dLnData,  double dLnDataNew, long indexT)
{
  double dPjump;

  if (dLnPriorNew == NULL_SUPPORT ||
      dLnLikeNew  == NULL_SUPPORT ||
      dLnDataNew  == NULL_SUPPORT)
    return FALSE;

  if (pgd->rgdPerks[indexT] == 0) /* always accept jumps at perk zero */
    return TRUE;

  if (pgd->nSimTypeFlag == 3) { /* posterior is tempered */
    dPjump = pgd->rgdPerks[indexT] *
             (dLnPriorNew - dLnPrior + dLnLikeNew - dLnLike) +
             dLnKern - dLnKernNew;
  }
  else { /* only the likelihood is tempered */
    dPjump = dLnPriorNew - dLnPrior +
             pgd->rgdPerks[indexT] * (dLnLikeNew - dLnLike) +
             dLnKern - dLnKernNew;
  }

  if (bExptIsDep)
    dPjump += pgd->rgdPerks[indexT] * (dLnDataNew - dLnData);

  return ((BOOL) (log(Randoms()) <= dPjump));

} /* TestImpRatioTemper */


/* Function -------------------------------------------------------------------
   TestTemper

   Test temperature against a random number between 0 and 1
*/
BOOL TestTemper (PGIBBSDATA pgd, long indexT, long indexT_new, double dLnPrior,
                 double dLnData, double pseudo, double pseudonew)
{
  double dPjump = 0;
  #define MINUSLN2  -0.6931471805599452862268

  if (dLnPrior + dLnData == NULL_SUPPORT)
    return FALSE;

  if (pgd->nSimTypeFlag == 3) { /* the posterior is tempered */
    dPjump = (pgd->rgdPerks[indexT_new] -
              pgd->rgdPerks[indexT]) * (dLnPrior + dLnData) +
             pseudonew - pseudo +
             ((indexT_new == 0) || (indexT_new == pgd->nPerks - 1) ?
              0 : MINUSLN2) -
             ((indexT     == 0) || (indexT     == pgd->nPerks - 1) ?
              0 : MINUSLN2);
  }
  else { /* only the likelihood is tempered, the prior cancels out */
    dPjump = (pgd->rgdPerks[indexT_new] -
              pgd->rgdPerks[indexT]) * dLnData +
             pseudonew - pseudo +
             ((indexT_new == 0) || (indexT_new == pgd->nPerks - 1) ?
              0 : MINUSLN2) -
             ((indexT     == 0) || (indexT     == pgd->nPerks - 1) ?
              0 : MINUSLN2);
  }

  return ((BOOL) (log(Randoms()) <= dPjump));

} /* TestTemper */


/* Function -------------------------------------------------------------------
   TraverseLevels (recursive)

   Called with variable argument list ending in NULL;
   arguments should be pointers only; if you call this with a value
   that can be zero, you will be very sorry

   Find all allocated levels, execute `routinePtr' for each, starting at the
   top, passing the argument list as char**

   The argument list is saved from the initial call; on recursive calls the
   list is NULL
*/
void TraverseLevels (PLEVEL plevel,
                     void (*routinePtr)(PLEVEL plevel, char **args), ...)
{
  va_list ap;
  static char *arg[MAX_ARGS], **args = arg;
  char *arg1;
  long n, nargs = 0;

  va_start(ap, routinePtr);
  if ((arg1 = va_arg (ap, char*)) != NULL) {
    arg[0] = arg1;
    while ((arg[++nargs] = va_arg(ap, char*)) != NULL) {};
  }
  va_end (ap);

  routinePtr (plevel, args);

  for (n = 0; n < plevel->iInstances; n++)
    TraverseLevels (plevel->pLevels[n], routinePtr, NULL);

} /* TraverseLevels */


/* Function -------------------------------------------------------------------
   TraverseLevels1 (recursive)

   Same as TraverseLevels, but checks the return code of the routine passed
   and returns the same code (0 if error, 1 if success).
*/
int TraverseLevels1 (PLEVEL plevel,
                     int (*routinePtr)(PLEVEL plevel, char **args), ...)
{
  va_list ap;
  static char *arg[MAX_ARGS], **args = arg;
  char *arg1;
  long n, nargs = 0;

  va_start (ap, routinePtr);
  if ((arg1 = va_arg (ap, char*)) != NULL) {
    arg[0] = arg1;
    while ((arg[++nargs] = va_arg(ap, char*)) != NULL) {};
  }
  va_end (ap);

  if (routinePtr (plevel, args)) {
    for (n = 0; n < plevel->iInstances; n++) {
      if (!TraverseLevels1(plevel->pLevels[n], routinePtr, NULL)) {
        /* error */
        return (0);
      }
    }
  }
  else /* error */
    return (0);

  /* success */
  return (1);

} /* TraverseLevels1 */


/* Function -------------------------------------------------------------------
   WriteHeader

   Write the complete output file header
*/
void WriteHeader (PANALYSIS panal)
{
  PGIBBSDATA pgd = &panal->gd;
  long i;

  fprintf(pgd->pfileOut, "iter\t");
  TraverseLevels (panal->pLevels[0], WriteParameterNames, panal,
                  pgd->pfileOut, NULL);
  if ((pgd->nSimTypeFlag == 3) || (pgd->nSimTypeFlag == 4)) {
    /* if the user has required tempering and specified a perk scale,
       then print header for temperature index and pseudo-priors etc. */
    fprintf(pgd->pfileOut, "IndexT\t");
    for (i = 0; i < pgd->nPerks; i++)
      fprintf(pgd->pfileOut, "LnPseudoPrior(%ld)\t",i+1);
  }
  fprintf(pgd->pfileOut, "LnPrior\tLnData\tLnPosterior\n");
  fflush(pgd->pfileOut);

} /* WriteHeader */


/* Function -------------------------------------------------------------------
   WriteParameterNames

   Called from Traverse Levels
   Write the names of the sampled parameters to output file header
*/
void WriteParameterNames (PLEVEL plevel, char **args)
{
  PANALYSIS panal = (PANALYSIS)args[0];
  PFILE outFile = (FILE*)args[1];
  long n, m;

  panal->iInstance[plevel->iDepth] = plevel->iSequence;

  for (n = 0; n < plevel->nMCVars; n++) {
    fprintf (outFile, "%s(", plevel->rgpMCVars[n]->pszName);
    for (m = 1; m < plevel->iDepth; m++)
      fprintf (outFile, "%d.", panal->iInstance[m]);
    fprintf (outFile, "%d)\t", panal->iInstance[plevel->iDepth]);
  }

} /* WriteParameterNames */


/* Function -------------------------------------------------------------------

   WriteMCVars

   Write the values of MC vars for one level to output file
*/
void WriteMCVars (PLEVEL plevel, char **args)
{
  PFILE pOutFile = (PFILE)args[0];
  long n;
  PMCVAR pMCVar;

  for (n = 0; n < plevel->nMCVars; n++) {
    pMCVar = plevel->rgpMCVars[n];
    fprintf(pOutFile, "%5g\t", pMCVar->dVal);
  }

} /* WriteMCVars */

/* End */
