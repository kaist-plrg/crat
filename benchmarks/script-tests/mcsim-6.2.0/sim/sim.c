/* sim.c

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

   Entry point and main simulation routines for 'sim' program.

   This file can use the SUNDIALS CVODES libraries if they are installed.
   If so, the symbols HAVE_LIBSUNDIALS_CVODES and HAVE_LIBSUNDIALS_NVECSERIAL
   should be defined in config.h (or in the makefile).
   Otherwise, the corresponding features are disabled.

*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <math.h>
#include <assert.h>

#include "delays.h"
#include "yourcode.h"
#include "getopt.h"
#include "mh.h"
#include "optdsign.h"
#include "lexerr.h"
#include "lsodes.h"
#include "sim.h"
#include "simi.h"
#include "siminit.h"
#include "simo.h"
#include "simmonte.h"
#include "strutil.h"
#include "config.h"

/* CVODES specific includes and routines */

#ifdef HAVE_LIBSUNDIALS_CVODES

#include <nvector/nvector_serial.h>  /* serial N_Vector types, fcts., macros */

#include <cvodes/cvodes.h>           /* prototypes CVODE fcts. and consts. */
#include <cvodes/cvodes_band.h>      /* prototype for CVBand */
#include <cvodes/cvodes_dense.h>     /* prototype for CVDense */
#include <sundials/sundials_dense.h> /* definitions DlsMat DENSE_ELEM */
#include <sundials/sundials_types.h> /* definition of type realtype */
#include <sundials/sundials_math.h>  /* definition of ABS and EXP */


typedef struct {
  int nVars; /* number of state and output variables */
} UserData;

/* Check function return value...
     opt == 0 means SUNDIALS function allocates memory so check if
              returned NULL pointer
     opt == 1 means SUNDIALS function returns a flag so check if
              flag >= 0
     opt == 2 means function allocates memory so check if returned
              NULL pointer */
static int check_flag(void *flagvalue, char *funcname, int opt)
{
  int *errflag;

  /* Check if SUNDIALS function returned NULL pointer - no memory allocated */

  if (opt == 0 && flagvalue == NULL) {
    fprintf(stderr, "[SUNDIALS ERROR] %s() failed - returned NULL pointer\n\n",
            funcname);
    return(1); }

  /* Check if flag < 0 */

  else if (opt == 1) {
    errflag = (int *) flagvalue;
    if (*errflag < 0) {
      fprintf(stderr, "[SUNDIALS ERROR] %s() failed with flag = %d\n\n",
              funcname, *errflag);
      return(1); }}

  /* Check if function returned NULL pointer - no memory allocated */

  else if (opt == 2 && flagvalue == NULL) {
    fprintf(stderr, "[MEMORY ERROR] %s() failed - returned NULL pointer\n\n",
            funcname);
    return(1); }

  return(0);
}


/* ----------------------------------------------------------------------------
   f_for_cvodes

   stupid routine interfacing cvodes derivative function call and CalcDeriv.
*/
static int f_for_cvodes(realtype t, N_Vector u, N_Vector udot, void *user_data)
{
  static realtype *rgMVars = NULL;
  static int nStates, nVars;
  int i;

  /* problem with the output variables, derivatives have the proper length
     and do not need to be translated */
  if (rgMVars == NULL) { /* initialize */

    nStates = NV_LENGTH_S(udot);

    /* Extract number of outputs from user_data */
    nVars = ((UserData *) user_data)->nVars;

    /* state and output vector */
    rgMVars = (realtype *) malloc(nVars * sizeof(realtype));

    if (/*!dudata ||*/ !rgMVars)
      ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "f_for_cvodes", NULL);
  }

  /* copy u to rgMVars start */
  for (i = 0; i < nStates; i++) {
    rgMVars[i] = NV_Ith_S(u, i);
  }

  CalcDeriv ((PDOUBLE) rgMVars, (PDOUBLE) NV_DATA_S(udot), (PDOUBLE) &t);

  return(0);

} /* f_for_cvodes */

#endif

/* MPI specific includes */

#ifdef USEMPI
#include "mpi.h"
#endif


/* ----------------------------------------------------------------------------
   CorrectInputToTransition

   resets the integrator and inputs when an input transition occurs.

   returns the simulation time pexp->dTime and input values to
   the input discontinuity, or transition point *pdTtrans.

   The inputs are updated to reflect their state just after the
   transition.  The integrator is initialized for a new segment.

   This does NOT affect state and output definitions.
*/
void CorrectInputToTransition (PEXPERIMENT pexp, PDOUBLE pdTtrans)
{
  pexp->dTime = *pdTtrans;
  UpdateInputs (&pexp->dTime, pdTtrans);

} /* CorrectInputToTransition */


/* ----------------------------------------------------------------------------
   Euler

   Simple Euler integrator.
*/
int Euler (long neq, double *y, double *t, double tout, double dTStep)
{
  static PDOUBLE rgdDeriv;
  double dTmp_step;
  long   i;

  if (!(rgdDeriv))
    if ( !(rgdDeriv = InitdVector (neq)))
      ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "Euler", NULL);

  /* Iterate through time out to prescrebed output */
  while (*t < tout) {

    /* Compute derivatives at current time */
    CalcDeriv (y, rgdDeriv, t);

    /* Update time */
    *t = *t + dTStep;

    /* But do not exceed prescribed tout */
    if (*t > tout) {
      dTmp_step = tout - (*t - dTStep);
      *t = tout;
    }
    else
      dTmp_step = dTStep;

    /* Update the state variables */
    for (i = 0; i < neq; i++)
      y[i] = y[i] + dTmp_step * rgdDeriv[i];

    if (bDelays)
      StoreDelayed(*t);

    DoStep_by_Step();
  }

  /* Calculate the derivatives at t = tout for cleanliness */
  CalcDeriv (y, rgdDeriv, t);

  return (0);

} /* Euler */


/* ----------------------------------------------------------------------------
   FreeVarMod

   Frees a VARMOD structure
*/
void FreeVarMod (PVOID pData)
{
  PVARMOD pvarmod = (PVARMOD) pData;

  if (IsInput (pvarmod->hvar))
    if (pvarmod->uvar.pifn)
      free (pvarmod->uvar.pifn);

  free (pvarmod);

  /* we might have to free those too... */
  /* PDOUBLE rgT0s;           /\* Array of start times *\/ */
  /* PDOUBLE rgMags;          /\* Array of magnitudes *\/ */
  /* HANDLE *rghT0s;          /\* Handles to start times *\/ */
  /* HANDLE *rghMags;         /\* Handles to magnitudes *\/ */
  /* PINT   rgOper;           /\* Array of operation types *\/ */

} /* FreeVarMod */


/* ----------------------------------------------------------------------------
   ModifyOneParm

   Callback function for ModifyParms.
*/

int ModifyOneParm (PVOID pData, PVOID pNullInfo)
{
  PVARMOD pvarmod = (PVARMOD) pData;

  if (IsInput(pvarmod->hvar))
    SetInput (pvarmod->hvar, pvarmod->uvar.pifn);
  else
    SetVar (pvarmod->hvar, pvarmod->uvar.dVal);

  return 0;

} /* ModifyOneParm */


/* ----------------------------------------------------------------------------
   ModifyParms

   Modifies the parameters in the plistParmMods LIST of the experiment
   spec by call ForAllList to increment through the list.
*/
void ModifyParms (PLIST plistParmMods)
{

  assert (plistParmMods);
  ForAllList (plistParmMods, &ModifyOneParm, NULL);

} /* ModifyParms */


/* ----------------------------------------------------------------------------
   DoOneExperiment

   Runs one experiment - return 1 on success and 0 in case of errors
*/
int DoOneExperiment (PEXPERIMENT pexp)
{

  double dTout;     /* next output time */
  double dTtrans;   /* next exposure transition time */
  double dTup;      /* the smaller one of dTout or dTtrans*/
  int    iOut;      /* index to next output time */
  PMODELINFO pmod;  /* pointer to the current model info */
  PINTSPEC   pis;   /* pointer to the integrator specs */

#ifdef HAVE_LIBSUNDIALS_CVODES
  /* CVODES specific variables */
  static N_Vector u = NULL;
  static UserData user_data;
  static void *cvode_mem = NULL;
  int flag, i;
#endif

  if (!pexp) return 0;

  pmod = pexp->pmodelinfo;
  pis  = &(pexp->is);

  if (!InitOutputs (pexp, &iOut, &dTout)) return 0;

  /* Resolve dependency for initial time, eventually */
  if (pexp->hT0)
    pexp->dT0 = GetVarValue (pexp->hT0);

   /* Resolve dependent inputs, which calls ScaleModel */
  UpdateInputs (&pexp->dT0, &dTtrans);

  if (bDelays)
    InitDelays(pexp->hT0);

  if (pexp->dT0 > dTtrans) {
    printf ("\nError: starting time is greater than first discontinuity,"
            "       check your inputs - Exiting.\n\n");
    exit (0);
  }

  if (pexp->dT0 > dTout) {
    printf ("\nError: starting time is greater than first output time,"
            "       check your outputs - Exiting.\n\n");
    exit (0);
  }

  pexp->dTime = pexp->dT0;

  /* integrator initializations */
  if (pis->iAlgo == IAL_LSODES) { /* Lsodes algorithm */
    /* set lsodes return flag to 1 for first call */
    pis->iDSFlag = 1;
  }
  else {
    if (pis->iAlgo == IAL_CVODES) { /* Sundials CVODE algorithm */

#ifdef HAVE_LIBSUNDIALS_CVODES

      if (1 || u == NULL) { /* always done for now */

        /* create a serial vector for state variables */
        u = N_VNew_Serial(pmod->nStates);  /* Allocate u vector */
        if (check_flag((void*)u, "N_VNew_Serial", 0)) return(1);

        /* call CVodeCreate to create the solver memory and specify the
           Backward Differentiation Formula and the use of a Newton iteration */
        cvode_mem = CVodeCreate(CV_BDF, CV_NEWTON);
        if (check_flag((void *)cvode_mem, "CVodeCreate", 0)) return(1);

        /* set initial state values */
        for (i = 0; i < pmod->nStates; i++)
          NV_Ith_S(u, i) = pmod->pdModelVars[i];

        /* initialize the integrator memory and specify the
           user's right hand side function in u'=f(t,u), the inital time T0,
           and the initial dependent variable vector u. */
        flag = CVodeInit(cvode_mem, f_for_cvodes, pexp->hT0, u);
        if (check_flag(&flag, "CVodeInit", 1)) return(1);

        /* set the scalar relative tolerance and scalar absolute tolerance */
        flag = CVodeSStolerances(cvode_mem,
                                 RCONST(pis->dRtol), RCONST(pis->dAtol));
        if (check_flag(&flag, "CVodeSStolerances", 1)) return(1);

        /* set the pointer to user-defined data used to store the total
           number of state and output variables */
        user_data.nVars = pmod->nModelVars;
        flag = CVodeSetUserData(cvode_mem, &user_data);
        if (check_flag(&flag, "CVodeSetUserData", 1)) return(1);

        /* set the maximum number of internal steps before t_out */
        flag = CVodeSetMaxNumSteps(cvode_mem, pis->maxsteps);
        if (check_flag(&flag, "CVodeSetMaxNumSteps", 1)) return(1);

        /* set the maximum number of error test failures */
        flag = CVodeSetMaxErrTestFails(cvode_mem, pis->maxnef);
        if (check_flag(&flag, "CVodeSetMaxErrTestFails", 1)) return(1);

        /* set the maximum number of nonlinear iterations */
        flag = CVodeSetMaxNonlinIters(cvode_mem, pis->maxcor);
        if (check_flag(&flag, "CVodeSetMaxNonlinIters", 1)) return(1);

        /* set the maximum number of convergence failures */
        flag = CVodeSetMaxConvFails(cvode_mem, pis->maxncf);
        if (check_flag(&flag, "CVodeSetMaxConvFails", 1)) return(1);

        /* set the oefficient in the nonlinear convergence test */
        flag = CVodeSetNonlinConvCoef(cvode_mem, RCONST(pis->nlscoef));
        if (check_flag(&flag, "CVodeSetNonlinConvCoef", 1)) return(1);
        
        /* call CVDense to specify the CVDENSE dense linear solver */
        flag = CVDense(cvode_mem, pmod->nStates);
        if (check_flag(&flag, "CVDense", 1)) return(1);

        /* set the user-supplied Jacobian routine Jac, not used now
        flag = CVDlsSetBandJacFn(cvode_mem, Jac);
        if(check_flag(&flag, "CVDlsSetBandJacFn", 1)) return(1); */

      }
      else { /* disabled for now */
        /* reset initial state values */
        for (i = 0; i < pmod->nStates; i++)
          NV_Ith_S(u, i) = pmod->pdModelVars[i];

        flag = CVodeReInit(cvode_mem, pexp->dT0, u);
      }
#endif
    } /* end if IAL_CVODES */
  }

  /* Iterate to final time */
  while (pexp->dTime < pexp->dTfinal) {

    /* If dynamics equations are defined */
    if (pmod->nStates > 0) {

      /* the upper limit of integration dTup should be either dTout
         or dTtrans, whichever is smaller */
      /* F. Bois, 08 April 2007: before that, if the difference between dTout
         and dTtrans is too small make dTtrans = dTout to avoid problems with
         the integration */
      if (fabs(dTout - dTtrans) < DBL_EPSILON * 2.0 *
                                  mymax(fabs(dTout), fabs(dTtrans)))
        dTtrans = dTout;

      dTup = (dTout < dTtrans) ? dTout : dTtrans;

      /* F. Bois, 07 October 2008: same rounding problem fix: */
      if (fabs(dTup - pexp->dTime) < DBL_EPSILON * 2.0 *
                                     mymax(fabs(dTup), fabs(pexp->dTime)))
        pexp->dTime = dTup;

      if (pis->iAlgo == IAL_LSODES) { /* Lsodes algorithm */

        pis->rwork[0] = dTup; /* do not overshoot dTup - FB 01/07/97 */

        lsodes_(&pmod->nStates, pmod->pdModelVars, &(pexp)->dTime,
                &dTup, &pis->itol, &pis->dRtol, &pis->dAtol,
                &pis->itask, &pis->iDSFlag, &pis->iopt, pis->rwork,
                &pis->lrw, pis->iwork, &pis->liw, &pis->iMf);

        /* Handle error returns : FB 25/11/96 : */
        if (pis->iDSFlag < 0) {
          /* We cannot guarantee the accuracy of the results, exit the routine
             with an error flag */
          return (0);
        }
      }
      else {
        if (pis->iAlgo == IAL_CVODES) { /* Sundials CVODE algorithm */

#ifdef HAVE_LIBSUNDIALS_CVODES
	  if (dTup > (pexp)->dTime) {
	    /* do not overshoot */
            flag = CVodeSetStopTime(cvode_mem, (realtype) dTup);
            flag = CVode(cvode_mem, (realtype) dTup, u, &(pexp)->dTime,
                         CV_NORMAL);
            if (check_flag(&flag, "CVode", 1)) {
              /* we cannot guarantee the accuracy of the results, exit routine
                 with an error flag */
              return (0);
            }
            /* copy back state values */
            for (i = 0; i < pmod->nStates; i++) {
	      pmod->pdModelVars[i] = NV_Ith_S(u, i);
	    }
          }
#endif
        }
        else if (pis->iAlgo == IAL_EULER) { /* Euler algorithm */
          Euler(pmod->nStates, pmod->pdModelVars, &(pexp)->dTime, dTup,
                pis->dTStep);
        }
      }
    }
    else {
      /* We still need to advance the time */
      pexp->dTime = (dTout < dTtrans) ? dTout : dTtrans;
    }

    if (dTtrans <= dTout) {
      /* dTime == dTtrans <= dTout: we are at a discontinuity.
         This point belongs to the NEW period UNLESS we are at
         the final time */
      if (dTtrans < dTout) {
        if (dTtrans < pexp->dTfinal) {
          CorrectInputToTransition (pexp, &dTtrans);
          pis->iDSFlag = 1;
        }
      }
      else {
        /* dTtrans == dTout */
        if (dTtrans < pexp->dTfinal) {
          CorrectInputToTransition (pexp, &dTtrans);
          pis->iDSFlag = 1;
        }
        SaveOutputs (pexp, &dTout);
        NextOutputTime (pexp, &dTout, &iOut);
      }
    }
    else {
      /* dTime == dTout < dTtrans: */
      SaveOutputs (pexp, &dTout);
      NextOutputTime (pexp, &dTout, &iOut);
    }

  } /* while dTime < final time */

  if (pis->iAlgo == IAL_CVODES) { /* cleanup Sundials CVODE algorithm */
#ifdef HAVE_LIBSUNDIALS_CVODES
    /* Free vector u */
    N_VDestroy_Serial(u);
    CVodeFree(&cvode_mem);  /* Free the integrator memory */
#endif
  }

  /* success */
  return 1;

} /* DoOneExperiment */


/* ----------------------------------------------------------------------------
   DoOneNormalExp

   Does one AT_DEFAULTSIM simulation.

   Return 1 on success and 0 in case of failure
*/
int DoOneNormalExp (PANALYSIS panal, PEXPERIMENT pexp)
{
  printf("experiment %d\n", pexp->iExp); /* Show what experiment it is */

  InitModel();
  ModifyParms(panal->expGlobal.plistParmMods); /* Global modifications */
  ModifyParms(pexp->plistParmMods); /* Mods for this experiment */
  if (!DoOneExperiment(pexp)) {
    /* Error */
    return 0;
  }

  return (1);

} /* DoOneNormalExp */


/* ----------------------------------------------------------------------------
   DoOneMCExp

   Does one AT_MONTECARLO simulation.

   Can maybe merge this with DoOneNormalExp() in the future.

   The major issue is the order of setting parameters.  For each
   experiment in a Monte Carlo run of an analysis, the order must be
   as follows:

   Each Run
    calc mc mods

     Each Simulation (old "Experiment")
     1)  Init the model
     2)  Global parm mods
     3)  Monte Carlo mods
     4)  Local mods override everything

   The problem becomes that for the simulation to be started over
   again, the inputs have to be told to initialize and parm mods for
   the current experiment must be made (body weight, etc).  This
   currently won't happen unless the model is init'd.  Maybe create a
   ResetInputs() with starting time which will do the funky stuff
   done by the global variable right now.

   Return 1 on success and 0 in case of failure
*/
int DoOneMCExp (PANALYSIS panal, PEXPERIMENT pexp)
{
  register MONTECARLO *pmc = &panal->mc;

  InitModel ();
  ModifyParms (panal->expGlobal.plistParmMods); /* Global modifications */
  SetParms (pmc->nParms, pmc->rghvar, pmc->rgdParms); /* MC mods */
  ModifyParms (pexp->plistParmMods); /* Mods for this experiment */
  if (!DoOneExperiment (pexp)) {
    /* Error */
    return 0;
  }

  return (1);

} /* DoOneMCExp */


/* ----------------------------------------------------------------------------
   DoNormal

   Does a normal analysis
*/
void DoNormal (PANALYSIS panal)
{
  int nExps = panal->expGlobal.iExp;
  int i;

  printf ("\nDoing analysis - %d normal experiment%c\n", nExps,
       (nExps > 1 ? 's' : ' '));

  for (i = 0; i < nExps; i++) {
    if (DoOneNormalExp (panal, panal->rgpExps[i])) {
      /* if successfull write out the results */
      WriteNormalOutput (panal, panal->rgpExps[i]);
    }
    else
      printf("[MCSIM ERROR] Integration failed - No output generated\n\n");
  }

} /* DoNormal */


/* ----------------------------------------------------------------------------
   DoMonteCarlo

   Does Monte Carlo simulations.
*/
void DoMonteCarlo (PANALYSIS panal)
{
  int       nExps = panal->expGlobal.iExp;
  long      nRuns = panal->mc.nRuns;
  MCPREDOUT mcpredout;
  BOOL      bOK;
  long      i, j;

  if (panal->rank == 0) {
    printf("Doing %ld Monte Carlo simulation%c, %d experiment%c%s\n",
           nRuns, (nRuns != 1 ? 's' : ' '),
           nExps, (nExps > 1 ? 's' : ' '), (nRuns != 1 ? " each" : "."));
    if (panal->size > 1)
      printf("Split between %d processors\n", panal->size);
  }
  else
    printf("\n");

  SetParents(&panal->mc, 0); /* do all requested simulations */

  OpenMCFiles(panal);
  mcpredout.pred = NULL;

#ifdef USEMPI
    MPI_Barrier(MPI_COMM_WORLD);
#endif

  /* split the work between processors */
  for (i = panal->rank; i < nRuns; i += panal->size) {

    /* output to screen, if required as a command-line option */
    if (i == 0)
      printf("\n");
    if (panal->bOutputIter && ((i+1) % panal->nOutputFreq == 0)) {
      if (panal->size > 1)
        printf("Processor %d, Iteration %ld\n", panal->rank, i + 1);
      else
        printf("Iteration %ld\n", i + 1);
    }

    panal->mc.lRun = i;

    CalcMCParms (&panal->mc, NULL, 0); /* start at 0, do them all */

    for (j = 0; j < nExps; j++) { /* do all experiments */
      bOK = DoOneMCExp (panal, panal->rgpExps[j]);
      if (!bOK)
        break;
    }

    if (bOK) { /* if successful write results */
      TransformPred(panal, &mcpredout); /* transform output run */
      WriteMCOutput(panal, &mcpredout);
    }
    else
      printf("Warning: Integration failed on iteration %ld, experiment %ld:\n"
             "         No output generated\n", panal->mc.lRun+1, j+1);

  } /* for i */

  CloseMCFiles(panal);
  if (mcpredout.pred)
    free(mcpredout.pred);

} /* DoMonteCarlo */


/* ----------------------------------------------------------------------------
   DoSetPoints

   Does Set Points analysis.

   If the number of runs (nRuns) is specified to be zero, set points
   are read from the set points file until end of file is reached.
   Otherwise, the number of runs explicity stated are read.  Not
   having enough points in the file in this latter case yields an
   error.

   If nRuns == 0, the test at the end of the while{} loop is not
   used, and the decision to continue is made by the return value of
   GetMCMods().
*/
void DoSetPoints (PANALYSIS panal)
{
  int       nExps = panal->expGlobal.iExp;
  long      nRuns = panal->mc.nRuns;
  MCPREDOUT mcpredout;
  BOOL      bOK = FALSE, bNotDone; /* Not finished with analysis */
  int       i;

  mcpredout.pred = NULL;

  OpenMCFiles(panal);

  if (panal->rank == 0) {
    printf ("Doing analysis - %ld SetPoints run%c... %d experiment%c%s\n",
            nRuns, (nRuns != 1 ? 's' : ' '), nExps, (nExps > 1 ? 's' : ' '),
            (nRuns != 1 ? " each" : " "));
    if (panal->size > 1)
      printf("Split between %d processors\n", panal->size);
  }
  else
    printf("\n");

  if ((!nRuns) && panal->rank == 0)
    printf("0 runs specified for SetPoint(). Reading entire file.\n\n");

  SetParents(&panal->mc, panal->mc.nSetParms);

#ifdef USEMPI
    MPI_Barrier(MPI_COMM_WORLD);
#endif

  panal->mc.lRun = 0; /* first run */
  bNotDone = TRUE;

  while (bNotDone) {

    bNotDone = GetSPMods (panal, NULL);

    /* split the work between processors */
    if ((bNotDone) && (panal->mc.lRun % panal->size == panal->rank)) {

      /* output to screen, if required as a command-line option */
      if (panal->bOutputIter &&
          ((panal->mc.lRun+1) % panal->nOutputFreq == 0)) {
        if (panal->size > 1)
          printf("Processor %d, Iteration %ld\n",
                 panal->rank, panal->mc.lRun + 1);
        else
          printf("Iteration %ld\n", panal->mc.lRun + 1);
      }

      /* do analysis if not finished */
      for (i = 0; i < nExps; i++) { /* do all experiments */
        bOK = DoOneMCExp (panal, panal->rgpExps[i]);
        if (!bOK) break;
      }

      if (bOK) {
        /* if successful write results */
        TransformPred (panal, &mcpredout); /* transform output run */
        WriteMCOutput (panal, &mcpredout);
      }
      else
        printf ("Warning: Integration failed on iteration %ld, experiment %d:\n"
                "         No output generated\n", panal->mc.lRun+1, i+1);
    } /* if bNotDone */

    panal->mc.lRun++; /* next run */
    if (nRuns) /* if a number of runs spec'd... */
      bNotDone = (panal->mc.lRun < nRuns);

  } /* while */

  CloseMCFiles (panal);

  if (mcpredout.pred) free(mcpredout.pred);

} /* DoSetPoints */


/* ----------------------------------------------------------------------------
   DoAnalysis

   Does the analysis in the given specification.
*/
void DoAnalysis (PANALYSIS panal)
{

  if (panal->size == 1)
    InitRandom (panal->rank, panal->dSeed, TRUE);
  else
    InitRandom (panal->rank, panal->dSeed + panal->rank, TRUE);
  
  switch (panal->iType) {

    default:
    case AT_DEFAULTSIM:
      if (panal->rank == 0) /* not parallel */
        DoNormal (panal);
      break;

    case AT_SETPOINTS:
      DoSetPoints (panal);
      break;

    case AT_MONTECARLO:
      DoMonteCarlo (panal);
      break;

    case AT_MCMC:
      DoMarkov (panal);
      break;

    case AT_OPTDESIGN:
      if (panal->rank == 0) /* not parallel */
        DoOptimalDesign (panal);
      break;

  } /* switch */

  if (panal->pfileOut) {
    fclose (panal->pfileOut);
    printf ("Wrote output file \"%s\"\n", panal->szOutfilename);
  }

} /* DoAnalysis */


/* ----------------------------------------------------------------------------
   FreeMemory

   To use in the case of simulations without Levels.
*/
void FreeMemory (PANALYSIS panal)
{
  int i, j;

  free(panal->modelinfo.pStateHvar);

  FreeList (&panal->mc.plistMCVars, NULL, TRUE);
  if (panal->mc.rgdParms) {
    free (panal->mc.rgdParms);
    free (panal->mc.rghvar);
  }

  PINTSPEC pis = &panal->rgpExps[0]->is;
  free (pis->iwork);
  free (pis->rwork);

  for (i = 0; i < panal->expGlobal.iExp; i++) {
    if (panal->rgpExps[i] != NULL) {
      FreeList (&panal->rgpExps[i]->plistParmMods, &FreeVarMod, TRUE);

      POUTSPEC pos = &panal->rgpExps[i]->os;
      free (pos->pszOutputNames);
      free (pos->phvar_out);
      free (pos->pcOutputTimes);
      free (pos->piCurrentOut);
      free (pos->prgdOutputTimes);
      for (j = 0; j < pos->nOutputs; j++)
        free(pos->prgdOutputVals[j]);
      free (pos->prgdOutputVals);
      free (pos->rgdDistinctTimes);
      ForAllList (pos->plistPrintRecs, &FreePrintRec, NULL);
      FreeList (&pos->plistPrintRecs, NULL, FALSE);
      free (pos->plistPrintRecs);
      ForAllList (pos->plistDataRecs, &FreeDataRec, NULL);
      FreeList (&pos->plistDataRecs, NULL, FALSE);
      free (pos->plistDataRecs);
      free (panal->rgpExps[i]);
    }
  }
  if (panal->bAllocatedFileName) {
    if (panal->szOutfilename)           free (panal->szOutfilename);
    if (panal->mc.szMCOutfilename)      free (panal->mc.szMCOutfilename);
    if (panal->gd.szGout)               free (panal->gd.szGout);
  }

  if (panal->mc.szSetPointsFilename)  free (panal->mc.szSetPointsFilename);
  if (panal->gd.szGrestart)           free (panal->gd.szGrestart);
  if (panal->gd.szGdata)              free (panal->gd.szGdata);

  FreeList (&panal->expGlobal.plistParmMods, NULL, TRUE);
  free (panal);

} /* FreeMemory */


/* ----------------------------------------------------------------------------
   MCVarListToArray

   converts a list of MCVAR to an array.  This must be a callback for
   ForAllList() since we are making the change here that will let us
   not to be forced to use list traversal in the future.
*/

MCVAR **vrgpMCVar; /* Avoid hairy pointers in here */
int   viMCVar;     /* Index to the array */

int MCVarListToArray (PVOID pv_pMCVar, PVOID pv_Null)
{

  vrgpMCVar[viMCVar] = (MCVAR *) pv_pMCVar; /* Copy the pointer and.. */
  viMCVar++; /* Advance to next element of array */
  return 1;

} /* MCVarListToArray */


/* ----------------------------------------------------------------------------
   PrepAnalysis

   makes the ANALYSIS structure easier to work with in the simulation
   code. Specifically, changes lists to arrays.
*/
void PrepAnalysis (PANALYSIS panal)
{
  register MONTECARLO *pmc = &panal->mc;
  register int l;

  pmc->nParms = ListLength (pmc->plistMCVars);
  /* avoid zero pmc->nParms which can confuse some implementations of
     malloc. If pmc->nParms is zero  no use is going to be made of these
     arrays anyway */
  if (pmc->nParms == 0) return;

  pmc->rgdParms = InitdVector (pmc->nParms);
  pmc->rgpMCVar = (MCVAR **) malloc((pmc->nParms)*sizeof(MCVAR *));
  if (!(pmc->rgdParms && pmc->rgpMCVar))
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "PrepAnalysis", NULL);

  /* Address of the first pointer */
  vrgpMCVar = &pmc->rgpMCVar[0];

  /* Initialize global array index */
  viMCVar = 0;
  ForAllList (pmc->plistMCVars, MCVarListToArray, (PVOID) NULL);
  FreeList (&pmc->plistMCVars, NULL, FALSE);

  /* Make a handle vector */
  pmc->rghvar = (HVAR *) malloc((pmc->nParms)*sizeof(HVAR));
  if (pmc->rghvar) {
    for (l = 0; l < pmc->nParms; l++)
      pmc->rghvar[l] = pmc->rgpMCVar[l]->hvar;
  }
  else
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "PrepAnalysis", NULL);

} /* PrepAnalysis */


/* ----------------------------------------------------------------------------
   PromptFilenames

   prompts for both input and output file names.  The space allocated
   for inputting the files is reallocated to their actual size.
*/
void PromptFilenames (PSTR *pszFileIn, PSTR *pszFileOut)
{
  *pszFileIn  = (PSTR) calloc (1, MAX_FILENAMESIZE);
  *pszFileOut = (PSTR) calloc (1, MAX_FILENAMESIZE);

  printf ("Input filename? ");

  if (!fgets (*pszFileIn, MAX_FILENAMESIZE, stdin)) {
    ReportError (NULL, RE_READERROR | RE_FATAL, "stdin", NULL);
  }
  else
    *pszFileIn = strtok (*pszFileIn, " \t\n");

  if (!(*pszFileIn)) /* Nothing entered, quit */
    return;

  if ((*pszFileIn)[0]) { /* Input file specified */
    printf ("Output filename? ");

    if (!fgets (*pszFileOut, MAX_FILENAMESIZE, stdin)) {
      ReportError (NULL, RE_READERROR | RE_FATAL, "stdin", NULL);
    }
    else
      *pszFileOut = strtok (*pszFileOut, " \t\n");
  }

  if (!(*pszFileOut) || !(*pszFileOut)[0]) { /* If no output specified */
    free (*pszFileOut);                      /* .. use default later */
    *pszFileOut = NULL;
  }
  else {
    *pszFileIn = (PSTR) realloc (*pszFileIn, MyStrlen(*pszFileIn) + 1);
    *pszFileOut = (PSTR) realloc (*pszFileOut, MyStrlen(*pszFileOut) + 1);
  }

} /* PromptFilenames */


/* ----------------------------------------------------------------------------
   GetCmdLineArgs

   retrieves options and filenames from the command line arguments passed to
   the program.

   The command line syntax is:

     mcsim [-options] [input-file [output-file]]

   If the output filename is not given a default is used.
   If neither the input, nor output filenames are given, the
   program prompts for them both.

   The options can appear anywhere in the line and in any order.

   The options are parsed with _getopt(). After _getopt() is called,
   the args in rgszArg have been permuted so that non-option args are
   first, which in this case means the filenames.

   Uses the following globals (see getopt.c comments):
     char *optarg;    -- Contains the string argument to each option in turn

   The following options are defined:
   -c (without argument)             : print MCMC convergence check to stdout,
                                       inhibits -i option if set (redundant)
   -h or -H (without argument)       : give minimal help
   -i (with integer argument)        : print one in every x MCMC iteration
                                       counter to stdout; x is given by the
                                       argument
   -D (with argument print-hierarchy): print the MCMC Level structure
*/
static char vszOptions[] = "c::h::H::i:D:";

void GetCmdLineArgs (int cArg, char *const *rgszArg, PSTR *pszFileIn,
                     PSTR *pszFileOut, PANALYSIS panal)
{
  int c;

  *pszFileIn = *pszFileOut = (PSTR) NULL;

  while (1) {

    c = _getopt (cArg, rgszArg, vszOptions);
    if (c == EOF) /* finished with option args */
      break;

    switch (c) {
      case 'c':
#ifdef USEMPI
        panal->bPrintConvergence = TRUE;
        if (optarg)
          printf(">> Option -c argument %s will be ignored\n\n", optarg);
#else
        printf(">> MPI parallelization not active: option -c is ignored\n\n");
#endif
        break;

      case 'D':
        /* remove optional leading '=' character */
        if (optarg[0] == '=')
          optarg++;
        if (!strcmp(optarg, "print-hierarchy")) {
          printf (">> Debug option %s\n\n", optarg);
          panal->bDependents = TRUE;
        }
        else {
          printf(">> A known debugging code must follow -D\nExiting.\n\n");
          exit(-1);
        }
        break;

      case 'H':
      case 'h':
        printf("Usage: %s [options] <input-file> [<output-file>]\n",
               rgszArg[0]);
        printf("Options:\n");
        printf("  -c                   "
               "Display MCMC convergence (if MPI is used)\n");
        printf("  -D=print-hierarchy   "
               "Print out the hierarchy for debugging\n");
        printf("  -h                   "
               "Display this information\n");
        printf("  -H                   "
               "Display this information\n");
        printf("  -i=<arg>             "
               "Print out every <arg> iteration\n");
        printf("\nFor further help on GNU MCSim please see:\n"
               "http://www.gnu.org/software/mcsim.\n\n");
        exit(-1);
        break;

      case 'i':
        /* remove optional leading '=' character */
        if (optarg[0] == '=')
          optarg++;
        /* convert argument to base 10 */
        panal->nOutputFreq = strtol(optarg, NULL, 10);
        if (panal->nOutputFreq > 0) {
          if (panal->rank == 0)
            printf (">> Print iteration frequency %d\n\n", panal->nOutputFreq);
          panal->bOutputIter = TRUE;
        }
        else {
          printf(">> Error: An integer print step must "
                 "follow -i\nExiting.\n\n");
          exit(-1);
        }
        break;

      default:
        printf("Exiting.\n\n");
        exit(-1);

    } /* switch */

  } /* while */

  switch (cArg - optind) { /* Remaining args are filenames */
    case 2: /* Output and input file specificed */
      *pszFileOut = rgszArg[optind + 1];
      /* Fall through! */

    case 1: /* Input file specificed */
      *pszFileIn = rgszArg[optind];
      break;

    case 0: /* No file names specified */
      PromptFilenames (pszFileIn, pszFileOut);
      break;

    default:
      exit (-1);
      break;

  } /* switch */

  while (*pszFileIn && (*pszFileIn)[0] &&      /* files specified   */
         !MyStrcmp(*pszFileIn, *pszFileOut)) { /* but not different */
    printf ("\n** Input and output filename must be different.\n");
    PromptFilenames (pszFileIn, pszFileOut);
  }

  if (!(*pszFileIn && (*pszFileIn)[0])) { /* no input name given is an error */
    printf ("Error: an input file name must be specified - Exiting.\n\n");
    exit (-1);
  }

} /* GetCmdLineArgs */


/* ----------------------------------------------------------------------------
*/
void AnnounceProgram (int iRank)
{
  if (iRank == 0) { /* serial */
    printf ("\n________________________________________\n");
    printf ("\nMCSim " VSZ_VERSION "\n\n");
    printf (VSZ_COPYRIGHT "\n\n");

    printf ("MCSim comes with ABSOLUTELY NO WARRANTY;\n"
            "This is free software, and you are welcome to redistribute it\n"
            "under certain conditions; "
            "see the GNU General Public License.\n\n");

#ifdef HAVE_LIBGSL
    printf ("* Using GNU Scientific Library (GSL)\n\n");
#endif

    printf ("* Using `%s' model in file \"%s\" created by %s\n\n",
            szModelDescFilename, szModelSourceFilename, szModelGenAndVersion);

  } /* end if iRank == 0 */

} /* AnnounceProgram */


/* ----------------------------------------------------------------------------
   main

   Entry point for simulation and analysis program.
*/
int main (int nArg, char **rgszArg)
{

#ifdef USEMPI
  MPI_Init(&nArg,&rgszArg);
#endif

  int rank = 0;
  int size = 1;

#ifdef USEMPI
  MPI_Comm_rank(MPI_COMM_WORLD, &rank);
  MPI_Comm_size(MPI_COMM_WORLD, &size);
#endif

  PSTR szFileIn, szFileOut;
  INPUTBUF ibIn;
  PANALYSIS panal = (PANALYSIS) malloc (sizeof(ANALYSIS));

  panal->rank = rank;
  panal->size = size;

  AnnounceProgram(panal->rank);

  if (!panal)
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL,
                 "ANALYSIS specification too large", NULL);

  InitAnalysis (panal);
  GetCmdLineArgs (nArg, rgszArg, &szFileIn, &szFileOut, panal);

  /* Define the output file as the global experiment default  */
  panal->szOutfilename = szFileOut;
  szFileOut == NULL ? (panal->bCommandLineSpec = FALSE) :
                      (panal->bCommandLineSpec = TRUE);

  if (!InitBuffer (&ibIn, szFileIn))
    ReportError (&ibIn, RE_INIT | RE_FATAL, "ReadInput", NULL);

  ibIn.pInfo = (PVOID) panal; /* Attach analysis specification to input */

  if (ReadAnalysis (&ibIn)) {
    PrepAnalysis (panal);
    DoAnalysis (panal);
  }

  if (panal->rank == 0)
    printf("Done.\n\n");

  if (panal->iType == AT_MCMC)
    FreeLevels (panal);
  else {
    FreeMemory (panal);
    free (ibIn.pbufOrg);
  }

#ifdef USEMPI
  MPI_Finalize();
#endif

  return 0;

} /* main */
