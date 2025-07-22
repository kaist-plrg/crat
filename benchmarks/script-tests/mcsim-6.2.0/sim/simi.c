/* simi.c

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

   Contains input routines for simulation.
   This file checks whether the SUNDIALS CVODES libraries are installed.
   If so, the symbols HAVE_LIBSUNDIALS_CVODES and HAVE_LIBSUNDIALS_NVECSERIAL
   should be defined in config.h (or in the makefile).
   Otherwise, the corresponding features are disabled (and the program
   switches back to the Lsodes integrator and issues a warming message).

*/

#include <assert.h>
#include <ctype.h>
#include <float.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

#include "lexerr.h"
#include "list.h"
#include "simi.h"
#include "siminit.h"
#include "simmonte.h"
#include "config.h"

PSTRLEX vrgszlexArgs[ARGS_MAX];

/* Keyword Map Structure */

typedef struct tagKM {
  PSTR szKeyword;
  int  iKWCode;   /* Enumeration code of Keyword KM_* */
  WORD fContext;  /* Bit flags of valid context for KW */
} KM, *PKM; /* Keyword Map */

KM vrgkmKeywordMap[] = { /* Global Keyword - code map */

  /* Simulation syntax */

  {"Level",         KM_LEVEL,       CN_GLOBAL},

  {"Experiment",    KM_EXPERIMENT,  CN_GLOBAL},    /* Obsolete!! */
  {"Simulation",    KM_EXPERIMENT,  CN_GLOBAL},

  {"OutputFile",    KM_OUTPUTFILE,  CN_GLOBAL},    /* Output file name spec */

  {"MCMC",          KM_MCMC,        CN_GLOBAL},    /* Metropolis spec */

  {"OptimalDesign", KM_OPTDESIGN,   CN_GLOBAL},    /* Optimal design spec */
  {"MonteCarlo",    KM_MONTECARLO,  CN_GLOBAL},    /* Monte Carlo spec */

  {"Distrib",       KM_MCVARY,      CN_GLOBAL},    /* MC Variable mod */
  {"Likelihood",    KM_MCVARY,      CN_GLOBAL},    /* special MC Variable */
  {"Density",       KM_MCVARY,      CN_GLOBAL},    /* special MC Variable */
  {"MCVary",        KM_MCVARY,      CN_GLOBAL},    /* Obsolete!! */
  {"InvTemperature",KM_TEMPERATURE, CN_GLOBAL},    /* Obsolete!! */
  {"Perks",         KM_TEMPERATURE, CN_GLOBAL},    /* tempered MCMC option */

  {"SetPoints",     KM_SETPOINTS,   CN_GLOBAL},    /* Forced point runs*/

  {"Integrate",     KM_INTEGRATE,   CN_GLOBAL | CN_EXPERIMENT},
  {"Simulate",      KM_SIMULATE,    CN_GLOBAL | CN_EXPERIMENT}, /* obsolete */
  {"StartTime",     KM_STARTTIME,   CN_GLOBAL | CN_EXPERIMENT},

  {"Print",         KM_PRINT,       CN_EXPERIMENT},
  {"Prediction",    KM_PRINT,       CN_EXPERIMENT},
  {"PrintStep",     KM_PRINTSTEP,   CN_EXPERIMENT},
  {"Data",          KM_DATA,        CN_EXPERIMENT},

  /* If a type is not seen before the first section is found, that section
     becomes the type.  Subsequent statements are ignored. */
  {"SimType",       KM_SIMTYPE,     CN_GLOBAL},    /* Optional SimType */

  {"End",           KM_END,         CN_GLOBAL},    /* Optional End statement */
  {"END",           KM_END,         CN_GLOBAL},    /* Optional End statement */

  /* Function arguments */

  {"DefaultSim",    KM_DEFAULTSIM,  CN_FUNCARG},   /* For SimType() only */

  {"No",            KM_NO,          CN_FUNCARG},   /* Use YesNoFromLex() */
  {"Yes",           KM_YES,         CN_FUNCARG},

  {"Beta",            KM_BETA,            CN_FUNCARG}, /* Use McvFromLex() */
  {"Binomial",        KM_BINOMIAL,        CN_FUNCARG},
  {"BinomialBeta",    KM_BINOMIALBETA,    CN_FUNCARG},
  {"Cauchy",          KM_CAUCHY,          CN_FUNCARG},
  {"Chi2",            KM_CHI2,            CN_FUNCARG},
  {"Exponential",     KM_EXPONENTIAL,     CN_FUNCARG},
  {"Gamma",           KM_GGAMMA,          CN_FUNCARG},
  {"GenLogNormal",    KM_GENLOGNORMAL,    CN_FUNCARG},
  {"HalfCauchy",      KM_HALFCAUCHY,      CN_FUNCARG},
  {"HalfNormal",      KM_HALFNORMAL,      CN_FUNCARG},
  {"InvGamma",        KM_INVGGAMMA,       CN_FUNCARG},
  {"LogNormal",       KM_LOGNORMAL,       CN_FUNCARG},
  {"LogNormal_v",     KM_LOGNORMALV,      CN_FUNCARG},
  {"LogUniform",      KM_LOGUNIFORM,      CN_FUNCARG},
  {"NegativeBinomial",KM_NEGATIVEBINOM,   CN_FUNCARG},
  {"Normal",          KM_NORMAL,          CN_FUNCARG},
  {"Normal_cv",       KM_NORMALCV,        CN_FUNCARG},
  {"Normal_v",        KM_NORMALV,         CN_FUNCARG},
  {"Piecewise",       KM_PIECEWISE,       CN_FUNCARG},
  {"Poisson",         KM_POISSON,         CN_FUNCARG},
  {"StudentT",        KM_STUDENTT,        CN_FUNCARG},
  {"TruncInvGamma",   KM_TRUNCINVGGAMMA,  CN_FUNCARG},
  {"TruncLogNormal",  KM_TRUNCLOGNORMAL,  CN_FUNCARG},
  {"TruncLogNormal_v",KM_TRUNCLOGNORMALV, CN_FUNCARG},
  {"TruncNormal",     KM_TRUNCNORMAL,     CN_FUNCARG},
  {"TruncNormal_cv",  KM_TRUNCNORMALCV,   CN_FUNCARG},
  {"TruncNormal_v",   KM_TRUNCNORMALV,    CN_FUNCARG},
  {"Uniform",         KM_UNIFORM,         CN_FUNCARG},
  {"UserSpecifiedLL", KM_USERLL,          CN_FUNCARG},

  {"Prediction",      KM_PREDICTION,      CN_FUNCARG},
  {"Data",            KM_DATA,            CN_FUNCARG},

  {"Euler",           KM_EULER,           CN_FUNCARG},
  {"Lsodes",          KM_LSODES,          CN_FUNCARG},
  {"LSODES",          KM_LSODES,          CN_FUNCARG},
  {"Cvodes",          KM_CVODES,          CN_FUNCARG},
  {"CVODES",          KM_CVODES,          CN_FUNCARG},

  {"Forward",         KM_FORWARD,         CN_FUNCARG},
  {"Backward",        KM_BACKWARD,        CN_FUNCARG},

  {"Replace",         KM_REPLACE,         CN_FUNCARG},
  {"Add",             KM_ADD,             CN_FUNCARG},
  {"Multiply",        KM_MULTIPLY,        CN_FUNCARG},

  /* Variables names valid in all CN_ */

  {"", 0, CN_ALL} /* End flag */

}; /* vrgkmKeywordMap[] */


/* ----------------------------------------------------------------------------
   GetKeywordCode

   Returns the code of the szKeyword given. If the string is not
   a valid keyword or abbreviation, returns 0.

   If pfContext is non-NULL, contexts in which the code is valid is
   set too.
*/
int GetKeywordCode (PSTR szKeyword, PINT pfContext)
{
  PKM pkm = &vrgkmKeywordMap[0];

  while (*pkm->szKeyword && MyStrcmp(szKeyword, pkm->szKeyword))
    pkm++;

  if (pfContext)
    *pfContext = pkm->fContext; /* Set iContext flag */

  return(pkm->iKWCode);         /* Return Keyword Code or 0 */

} /* GetKeywordCode */


/* ----------------------------------------------------------------------------
   GetKeywordCode_in_context

   Returns the code of the szKeyword given, in a given context.
   If the string is not a valid keyword or abbreviation in context, returns 0.

*/
int GetKeywordCode_in_context (PSTR szKeyword, WORD fContext)
{
  PKM pkm = &vrgkmKeywordMap[0];

  while (*pkm->szKeyword && !((pkm->fContext == fContext) &&
                              !MyStrcmp(szKeyword, pkm->szKeyword)))
    pkm++;

  return(pkm->iKWCode); /* Return Keyword Code or 0 */

} /* GetKeywordCode_in_context */


/* ----------------------------------------------------------------------------
   GetKeyword

   Returns the first string of the KM_ keyword map code given.  If the
   code is not a valid keyword code, returns NULL.
*/
PSTR GetKeyword (int iKWCode)
{
  PKM pkm = &vrgkmKeywordMap[0];

  while (*pkm->szKeyword && iKWCode != pkm->iKWCode)
    pkm++;

  return(pkm->szKeyword);        /* Return Keyword Code or 0 */

} /* GetKeyword */


/* ----------------------------------------------------------------------------
   YesNoFromLex

   Converts an string input argument into a Boolean,
   Yes being TRUE and No being FALSE.  Also, a numeric argument
   is converted to Yes if it is non-zero.
*/
BOOL YesNoFromLex (PSTR szLex)
{
  int ikwcode = GetKeywordCode(szLex, NULL);
  BOOL bReturn;

  bReturn = (!isalpha(szLex[0]) ? atoi(szLex)
             : ikwcode == KM_YES ? TRUE
             : ikwcode == KM_NO ? FALSE
             : FALSE);

  return bReturn;

} /* YesNoFromLex */


/* ----------------------------------------------------------------------------
   ImFromLex

   Converts an string input argument into the correct IM_
   integration method.
*/
long ImFromLex (PSTR szLex)
{
  int ikwcode = GetKeywordCode(szLex, NULL);
  long lReturn;

  lReturn = (!isalpha(szLex[0]) ? atoi(szLex)
            : ikwcode == KM_LSODES ? IAL_LSODES
            : ikwcode == KM_CVODES ? IAL_CVODES
            : ikwcode == KM_EULER  ? IAL_EULER
            : 0);

  return(lReturn);

} /* ImFromLex */


/* ----------------------------------------------------------------------------
   McvFromLex

   Converts a string input argument into the correct MCV_
   Monte Carlo variation distribution type.
*/
int McvFromLex (PSTR szLex)
{
  int ikwcode = GetKeywordCode(szLex, NULL);
  int iReturn;

  iReturn = (ikwcode == KM_UNIFORM          ? MCV_UNIFORM
             : ikwcode == KM_LOGUNIFORM     ? MCV_LOGUNIFORM
             : ikwcode == KM_BETA           ? MCV_BETA
             : ikwcode == KM_NORMAL         ? MCV_NORMAL
             : ikwcode == KM_HALFNORMAL     ? MCV_HALFNORMAL
             : ikwcode == KM_LOGNORMAL      ? MCV_LOGNORMAL
             : ikwcode == KM_TRUNCNORMAL    ? MCV_TRUNCNORMAL
             : ikwcode == KM_TRUNCLOGNORMAL ? MCV_TRUNCLOGNORMAL
             : ikwcode == KM_CHI2           ? MCV_CHI2
             : ikwcode == KM_BINOMIAL       ? MCV_BINOMIAL
             : ikwcode == KM_PIECEWISE      ? MCV_PIECEWISE
             : ikwcode == KM_EXPONENTIAL    ? MCV_EXPONENTIAL
             : ikwcode == KM_GGAMMA         ? MCV_GGAMMA
             : ikwcode == KM_POISSON        ? MCV_POISSON
             : ikwcode == KM_INVGGAMMA      ? MCV_INVGGAMMA
             : ikwcode == KM_TRUNCINVGGAMMA ? MCV_TRUNCINVGGAMMA
             : ikwcode == KM_NORMALV        ? MCV_NORMALV
             : ikwcode == KM_NORMALCV       ? MCV_NORMALCV
             : ikwcode == KM_LOGNORMALV     ? MCV_LOGNORMALV
             : ikwcode == KM_TRUNCNORMALV   ? MCV_TRUNCNORMALV
             : ikwcode == KM_TRUNCNORMALCV  ? MCV_TRUNCNORMALCV
             : ikwcode == KM_TRUNCLOGNORMALV? MCV_TRUNCLOGNORMALV
             : ikwcode == KM_BINOMIALBETA   ? MCV_BINOMIALBETA
             : ikwcode == KM_GENLOGNORMAL   ? MCV_GENLOGNORMAL
             : ikwcode == KM_STUDENTT       ? MCV_STUDENTT
             : ikwcode == KM_CAUCHY         ? MCV_CAUCHY
             : ikwcode == KM_HALFCAUCHY     ? MCV_HALFCAUCHY
             : ikwcode == KM_USERLL         ? MCV_USERLL
             : ikwcode == KM_NEGATIVEBINOM  ? MCV_NEGATIVEBINOM
             : (-1));

  return iReturn;

} /* McvFromLex */


/* ----------------------------------------------------------------------------
   GetTerminator

   Tries to read a statement terminator.  Reports Errors.
*/
int GetTerminator (PINPUTBUF pibIn, PSTR szLex)
{
  int iErr;

  if ((iErr = !GetPunct(pibIn, szLex, CH_STMTTERM))) {
    szLex[1] = CH_STMTTERM;
    ReportError(pibIn, RE_EXPECTED, szLex, NULL);
  }

  return(iErr);

} /* GetTerminator */


/* ----------------------------------------------------------------------------
   GetSimTYpe

   Get the type of simulation to perform.
*/
BOOL GetSimType (PINPUTBUF pibIn)
{
#define NAT_ARGS 1     /* the type */

  static int vrgiAtArgTypes[NAT_ARGS] = {LX_IDENTIFIER};

  PANALYSIS panal = (PANALYSIS) pibIn->pInfo;

  int  iAT = AT_DEFAULTSIM;
  int  iKwCode = 0;
  BOOL bErr=!GetFuncArgs(pibIn, NAT_ARGS, vrgiAtArgTypes, vrgszlexArgs[0]);

  if (!bErr) {
    iKwCode = GetKeywordCode(vrgszlexArgs[0], NULL);
    switch (iKwCode) {

    case KM_MONTECARLO:
      iAT = AT_MONTECARLO;
      break;

    case KM_SETPOINTS:
      iAT = AT_SETPOINTS;
      break;

    case KM_MCMC:
      iAT = AT_MCMC;
      break;

    case KM_OPTDESIGN:
      iAT = AT_OPTDESIGN;
      break;

    case KM_DEFAULTSIM:
      iAT = AT_DEFAULTSIM;
      break;

    default:
      ReportError(pibIn, RE_SPECERR | RE_FATAL, "Unknown SimType ",
                  vrgszlexArgs[0]);
      break;
    } /* switch */
  } /* if */
  else
    printf("Syntax: %s (Normal | MonteCarlo | SetPoints | MCMC)\n"
           "  -- if not specified, the first spec section will be used.\n\n",
           GetKeyword(KM_SIMTYPE));

  if (!bErr)
    panal->iType = iAT;

  return(bErr);

} /* GetSimType */


/* ----------------------------------------------------------------------------
   GetPerks

   In case of tempered MCMC, read in a user-defined inverse
   temperature schedule.
   Return TRUE if the structure is defined, FALSE on error.  The
   command syntax includes the number of inverse temperatures then the list of
   them (numbers in [0,inf] interval).
*/

BOOL GetPerks (PINPUTBUF pibIn, PSTR szLex, PGIBBSDATA pgd)
{
  int iType;
  int i;
  BOOL bOK = TRUE;
  BOOL bErr = FALSE; /* Return value flags error condition */

  if ((bErr = EGetPunct(pibIn, szLex, CH_LPAREN)))
    goto Exit_GetPerks;

  if ((bErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetPerks;

  pgd->nPerks = atoi(szLex);

  if ((bErr = (pgd->nPerks <= 0))) {
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, "positive-integer", szLex);
    goto Exit_GetPerks;
  }

  pgd->endT = pgd->nPerks - 1;

  /* allocate inverse temperature array */
  if (!(pgd->rgdPerks = InitdVector(pgd->nPerks)))
    ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetPerks", NULL);

  /* allocate working arrays */
  if (!(pgd->rgdlnPi  = InitdVector(pgd->nPerks)) ||
      !(pgd->rglCount = InitlVector(pgd->nPerks)))
    ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetPerks", NULL);

  /* try to get temperatures list: */

  for (i = 0; i < pgd->nPerks && bOK; i++) {

    /* get comma */
    if (!(bOK = GetOptPunct(pibIn, szLex, ','))) {
      szLex[0] = *pibIn->pbufCur++;
      szLex[1] = '\0';
      ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);
    } /* if */

    /* get floating point number */
    NextLex(pibIn, szLex, &iType);
    if (!(bOK &= (iType & LX_NUMBER) > 0))
      ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, "number", szLex);
    /* assign */
    pgd->rgdPerks[i] = atof(szLex);
    /* initialize */
    pgd->rgdlnPi[i]  = 0; /* perks */
    pgd->rglCount[i] = 0; /* temperature counts */
    if (pgd->rgdPerks[i] < 0)
      ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL,
                  "positive inverse temperature", szLex);
    if ((i > 0) && (pgd->rgdPerks[i] <= pgd->rgdPerks[i-1]))
      ReportError(pibIn, RE_SPECERR | RE_FATAL,
                  "Inverse temperatures out of order", NULL);
  } /* for */

  bErr = EGetPunct(pibIn, szLex, CH_RPAREN);

Exit_GetPerks:

  if (bErr)
    printf("Syntax: Inverse temperatures (nPerks, "
           "<n increasing inverse temperature values >= 0>)\n\n");

  return(!bErr);

} /* GetPerks */


/* ----------------------------------------------------------------------------
   GetLsodesOptions
*/
BOOL GetLsodesOptions (PINPUTBUF pibIn, PSTR szLex, PINTSPEC pis)
{
  BOOL bErr = FALSE;

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetLsodes;
  pis->dRtol = atof(szLex);

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetLsodes;
  pis->dAtol = atof(szLex);

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetLsodes;
  pis->iMf   = atoi(szLex);

  /* translate input iMf to original lsodes mf codes */
  switch (pis->iMf) {

    case 0:
    pis->iMf = 10;
    break;

    case 1:
    pis->iMf = 222;
    break;

    case 2:
    pis->iMf = 121;
    break;

    default:
    printf("Error: method flag must be 0, 1 or 2 for Lsodes - ");
    printf("Exiting\n\n");
    exit(0);
    break;

  } /* switch */

  pis->iDSFlag = 1;

Exit_GetLsodes:

  if (bErr) {
    printf("Lsodes options are: relative tolerance, absolute tolerance, "
           "method.\n\n");
    exit(0);
  }

  return(bErr);

} /* GetLsodesOptions */


/* ----------------------------------------------------------------------------
   GetCvodesOptions
*/
BOOL GetCvodesOptions (PINPUTBUF pibIn, PSTR szLex, PINTSPEC pis)
{
  BOOL bErr = FALSE;

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetCvodes;
  pis->dRtol = atof(szLex);

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetCvodes;
  pis->dAtol = atof(szLex);

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetCvodes;
  pis->maxsteps = atoi(szLex);

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetCvodes;
  pis->maxnef = atoi(szLex);

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetCvodes;
  pis->maxcor = atoi(szLex);

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetCvodes;
  pis->maxncf = atoi(szLex);

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetCvodes;
  pis->nlscoef = atof(szLex);

Exit_GetCvodes:

  if (bErr) {
    printf("Cvodes options are: relative tolerance, absolute tolerance, "
           "maxsteps, maxnef, maxcor, maxncf, nlscoef.\n\n");
    exit(0);
  }

  return(bErr);

} /* GetCvodesOptions */


/* ----------------------------------------------------------------------------
   GetEulerOptions
*/
BOOL GetEulerOptions (PINPUTBUF pibIn, PSTR szLex, PINTSPEC pis)
{
  BOOL bErr = FALSE;

  if (!(GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  if ((bErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetEuler;
  pis->dTStep = atof(szLex);

  if (pis->dTStep <= 0) {
    printf("Error: Time step specified is null or negative - Exiting\n\n");
    exit(0);
  }
  
Exit_GetEuler:

  if (bErr) {
    printf("Euler has one option: time-step.\n\n");
    exit(0);
  }

  return(bErr);

} /* GetEulerOptions */


/* ----------------------------------------------------------------------------
   GetIntegrate
*/
BOOL GetIntegrate (PINPUTBUF pibIn, PSTR szLex, PINTSPEC pis)
{
  BOOL bErr = FALSE;

  if ((bErr = EGetPunct(pibIn, szLex, CH_LPAREN)))
    goto Exit_GetIntegrate;

  if ((bErr = ENextLex(pibIn, szLex, LX_IDENTIFIER)))
    goto Exit_GetIntegrate;

  pis->iAlgo = ImFromLex(szLex);

  switch (pis->iAlgo) {
    case IAL_LSODES:
    GetLsodesOptions(pibIn, szLex, pis);
    break;

    case IAL_CVODES:
#ifndef HAVE_LIBSUNDIALS_CVODES
#ifndef HAVE_LIBSUNDIALS_NVECSERIAL
    /* switch to lsodes if cvodes is not installed */
    printf("Warning: Cvodes libraries are not available -\n"
           "         Switching to Lsodes with default options\n\n");
    pis->iAlgo = IAL_DEFAULT;
    goto Exit_GetIntegrate;
#endif
#endif
    GetCvodesOptions(pibIn, szLex, pis);
    break;

    case IAL_EULER:
    GetEulerOptions(pibIn, szLex, pis);
    break;

    default:
    printf("Error: Unknown integration method: %s - Exiting\n\n",
           vrgszlexArgs[0]);
    bErr = TRUE;
    goto Exit_GetIntegrate;
    break;
  }

  bErr = EGetPunct(pibIn, szLex, CH_RPAREN);

Exit_GetIntegrate:

  if (bErr) {
    printf("Syntax: %s([Lsodes | Cvodes | Euler], [OPTIONS]);\n\n",
           GetKeyword(KM_INTEGRATE));
    exit(0);
  }

  return(!bErr);

} /* GetIntegrate */


/* ----------------------------------------------------------------------------
   OneDToArray

   Copies one double from the list to the newly formed array.
   Increments the info pointer which is the pointer into the array.
*/
int OneDToArray (PVOID pData, PVOID pInfo)
{
  PDOUBLE *ppdArrayVal = (PDOUBLE *) pInfo;

  *(*ppdArrayVal)++ = *(PDOUBLE) pData;

  return 0;

} /* OneDToArray */


/* ----------------------------------------------------------------------------
   DListToArray

   Converts a list a doubles to an array of doubles. *pcDouble is
   the count of doubles in the array, and *ppDouble is the array
   pointer.
*/
void DListToArray (PLIST plist, PLONG pcDouble, PDOUBLE *ppDouble)
{
  PDOUBLE pdTmp; /* Temp pointer to get incremented */

  *pcDouble = ListLength(plist);

  if (!(pdTmp = *ppDouble = InitdVector(*pcDouble)))
    ReportError(NULL, RE_OUTOFMEM | RE_FATAL, "DListToArray", NULL);

  ForAllList(plist, &OneDToArray, (PVOID) &pdTmp);

} /* DListToArray */


/* ----------------------------------------------------------------------------
   GetListOfTimes

   Reads an arbitrary length list of times and closing parenthesis.
   Defines the count and array of times in the PRINTREC structure.
*/
BOOL GetListOfTimes (PINPUTBUF pibIn, int nRecs, PPRINTREC *ppr, PSTR szLex)
{
  PLIST plistTimes = InitList();
  PDOUBLE pdTmp;
  int iNLI, i, j;
  BOOL bErr;

  do {
    if ( !(pdTmp = InitdVector(1)))
      ReportError(NULL, RE_OUTOFMEM | RE_FATAL, "GetListOfTimes", NULL);

    *pdTmp = atof(szLex);
    QueueListItem(plistTimes, (PVOID) pdTmp);
  } while ((iNLI = NextListItem(pibIn, szLex, LX_NUMBER, 1, CH_RPAREN)) > 0);

  if (!iNLI) /* List terminator */
    bErr = EGetPunct(pibIn, szLex, CH_RPAREN) || !ListLength(plistTimes);
  else {
    bErr = TRUE;
    ReportError(pibIn, RE_LEXEXPECTED, "number", szLex);
  }

  if (!bErr)
    for (i = 0; i < nRecs; ++i)
      DListToArray(plistTimes, &ppr[i]->cTimes, &ppr[i]->pdTimes);

  FreeList(&plistTimes, NULL, TRUE); /* Free list and cells */

  for (i = 1; i < ppr[0]->cTimes && !bErr; i++) /* Verify Times */
    if ((bErr = (*(ppr[0]->pdTimes+i) <= *(ppr[0]->pdTimes+i-1)))) {
      for (j = 0; j < nRecs; ++j)
        free(ppr[j]->pdTimes);
      ReportError(pibIn, RE_SPECERR | RE_FATAL, "Times out of order", NULL);
    } /* if */

  return(bErr);

} /* GetListOfTimes */


/* ----------------------------------------------------------------------------
   GetListOfData

   Reads an arbitrary length list of data and closing parenthesis.
   Defines the count and array of data in the DATAREC structure.
*/
BOOL GetListOfData (PINPUTBUF pibIn, PDATAREC pda, PSTR szLex)
{
  PLIST plistData = InitList();
  PDOUBLE pdTmp;
  int iNLI;
  BOOL bErr;

  while ((iNLI = NextListItem(pibIn, szLex, LX_NUMBER, 1, CH_RPAREN))
         > 0) {
    if ( !(pdTmp = InitdVector(1)))
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetListOfData", NULL);

    *pdTmp = atof(szLex);
    QueueListItem(plistData, (PVOID) pdTmp);

  } /* while */

  if (!iNLI) /* List terminator */
    bErr = EGetPunct(pibIn, szLex, CH_RPAREN)
           || !ListLength(plistData);
  else {
    bErr = TRUE;
    ReportError(pibIn, RE_LEXEXPECTED, "number", szLex);
  } /* else */

  if (!bErr) DListToArray(plistData, &pda->cData, &pda->pdData);

  FreeList(&plistData, NULL, TRUE); /* Free list and cells */
  return(bErr);

} /* GetListOfData */


/* ----------------------------------------------------------------------------
   GetPrint

   Gets the arguments to a Print() statement. Put them in
   a list plistPrintRecs of PRINTREC structures
*/
BOOL bGavePrintUsage = FALSE; /* prevent multiple diagnostics */

BOOL GetPrint (PINPUTBUF pibIn, PSTR szLex, POUTSPEC pos)
{
  PPRINTREC pprintrec[MAX_PRINT_VARS];
  BOOL      bErr = FALSE;
  HVAR      hvar;
  int       nVars = 0, n, iLex;
  long      i, iLB, iUB;
  PSTRLEX   szTmp;

  /* get the opening parenthesis and list of variables to print */
  if (!(bErr = EGetPunct(pibIn, szLex, CH_LPAREN))) {
    for (;;) {
      NextLex(pibIn, szLex, &iLex);

      if (iLex != LX_IDENTIFIER)
        break; /* hitting the list of times, stop reading variable names */

      /* check if it is a scalar or an array and act accordingly */
      iLB = iUB = -1;
      if (GetPunct(pibIn, szTmp, '[')) /* array found, read bounds */
        GetArrayBounds(pibIn, &iLB, &iUB);

      if (iUB == -1) { /* scalar */

        if (nVars == MAX_PRINT_VARS)
          ReportError(pibIn, RE_TOOMANYPVARS | RE_FATAL, "GetPrint", NULL);

        if ((bErr = !(hvar = GetVarHandle(szLex))))
          ReportError(pibIn, RE_UNDEFINED | RE_FATAL, szLex, NULL);
        else {
          if ( !(pprintrec[nVars] = (PPRINTREC) malloc(sizeof(PRINTREC))) ||
               !(pprintrec[nVars]->szOutputName =
                (PSTR) malloc(MyStrlen(szLex)+1)))
            ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetPrint", NULL);
          MyStrcpy(pprintrec[nVars]->szOutputName, szLex);
          pprintrec[nVars]->hvar = hvar;
          assert(pprintrec[nVars]);
          ++nVars;
        }
        GetOptPunct(pibIn, szLex, ',');
      }
      else { /* array */

        for (i = iLB; i < iUB; i++) {
          sprintf(szTmp, "%s_%ld", szLex, i); /* create names */

          if (nVars == MAX_PRINT_VARS)
            ReportError(pibIn, RE_TOOMANYPVARS | RE_FATAL, "GetPrint", NULL);

          if ((bErr = !(hvar = GetVarHandle(szTmp))))
            ReportError(pibIn, RE_UNDEFINED | RE_FATAL, szTmp, NULL);
          else {
            if ( !(pprintrec[nVars] = (PPRINTREC) malloc(sizeof(PRINTREC))) ||
                 !(pprintrec[nVars]->szOutputName =
                  (PSTR) malloc(strlen(szTmp)+1))) {
              ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetPrint", NULL);
            }
            strcpy(pprintrec[nVars]->szOutputName, szTmp);
            pprintrec[nVars]->hvar = hvar;
            assert(pprintrec[nVars]);
            ++nVars;
          }

          if (i == (iUB - 1))
            GetOptPunct(pibIn, szTmp, ',');
        } /* end for i */
      } /* end else */
    } /* end for ever */

    /* check the number of variables read in */
    if (nVars < 1)
      ReportError(pibIn, RE_LEXEXPECTED, "identifier", szLex);

    /* get list of output times */
    bErr = GetListOfTimes(pibIn, nVars, pprintrec, szLex);

    if (bErr) {
      for (n = 0; n < nVars; ++n) {
        free(pprintrec[n]->szOutputName);
        free(pprintrec[n]);
      }
    }
    else
      for (n = 0; n < nVars; ++n)
        QueueListItem(pos->plistPrintRecs, (PVOID) pprintrec[n]);
  } /* if */

  if (!bErr) bErr = GetTerminator(pibIn, szLex);
  else {
    if (!bGavePrintUsage) {
      printf("Syntax: %s (<Identifiers>, Time1, Time2, ...)\n\n",
             GetKeyword(KM_PRINT));
      bGavePrintUsage = TRUE;
    }
  }

  return(bErr);

} /* GetPrint */


/* ----------------------------------------------------------------------------
   GetPrintStep

   Gets the arguments to a PrintStep() statement. They are: a list of
   identifiers, a start time, an end time, a time step.
   If the time period is not congruent with the time step the last step
   will be shortened.
*/
BOOL bGavePrintStepUsage = FALSE; /* prevent multiple diagnostics */

BOOL GetPrintStep (PINPUTBUF pibIn, PSTR szLex, POUTSPEC pos)
{
  PPRINTREC pprintrec[MAX_PRINT_VARS];
  BOOL      bErr = FALSE, bOK = TRUE;
  HVAR      hvar = 0;
  int       nVars = 0, n, iLex;
  long      i, iLB, iUB;
  double    dStart = 0, dEnd = 0, dStep = 0, dTmp;
  PSTRLEX   szTmp;

  /* get the opening parenthesis and list of variables to print */
  bErr = EGetPunct(pibIn, szLex, CH_LPAREN);
  if (bErr)
    goto Exit_GetPrintStep;

  for (;;) {
    NextLex(pibIn, szLex, &iLex);

    if (iLex != LX_IDENTIFIER)
      break; /* hitting the list of times, stop reading variable names */

    /* check if it is a scalar or an array and act accordingly */
    iLB = iUB = -1;
    if (GetPunct(pibIn, szTmp, '[')) /* array found, read bounds */
      GetArrayBounds(pibIn, &iLB, &iUB);

    if (iUB == -1) { /* scalar */

      if (nVars == MAX_PRINT_VARS)
        ReportError(pibIn, RE_TOOMANYPVARS | RE_FATAL, "GetPrint", NULL);

      if ((bErr = !(hvar = GetVarHandle(szLex))))
        ReportError(pibIn, RE_UNDEFINED | RE_FATAL, szLex, NULL);
      else {
        if ( !(pprintrec[nVars] = (PPRINTREC) malloc(sizeof(PRINTREC))) ||
             !(pprintrec[nVars]->szOutputName =
              (PSTR) malloc(MyStrlen(szLex)+1)))
          ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetPrint", NULL);
        MyStrcpy(pprintrec[nVars]->szOutputName, szLex);
        pprintrec[nVars]->hvar = hvar;
        assert(pprintrec[nVars]);
        ++nVars;
      }
      GetOptPunct(pibIn, szLex, ',');
    }
    else { /* array */

      for (i = iLB; i < iUB; i++) {
        sprintf(szTmp, "%s_%ld", szLex, i); /* create names */

        if (nVars == MAX_PRINT_VARS)
          ReportError(pibIn, RE_TOOMANYPVARS | RE_FATAL, "GetPrint", NULL);

        if ((bErr = !(hvar = GetVarHandle(szTmp))))
          ReportError(pibIn, RE_UNDEFINED | RE_FATAL, szTmp, NULL);
        else {
          if ( !(pprintrec[nVars] = (PPRINTREC) malloc(sizeof(PRINTREC))) ||
               !(pprintrec[nVars]->szOutputName =
               (PSTR) malloc(strlen(szTmp)+1))) {
            ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetPrint", NULL);
          }
          strcpy(pprintrec[nVars]->szOutputName, szTmp);
          pprintrec[nVars]->hvar = hvar;
          assert(pprintrec[nVars]);
          ++nVars;
        }

        if (i == (iUB - 1))
          GetOptPunct(pibIn, szTmp, ',');
      } /* end for i*/
    } /* end else */
  } /* end for ever */

  /* check the number of variables read in */
  if (nVars < 1) {
    ReportError(pibIn, RE_LEXEXPECTED, "identifier", szLex);
    bErr = TRUE;
    goto Exit_GetPrintStep;
  }

  /* the starting output time has already been read */
  dStart = atof(szLex);
  if (!(bOK = GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  /* get ending output time */
  if ((bErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetPrintStep;
  dEnd   = atof(szLex);
  if (!(bOK = GetPunct(pibIn, szLex, ',')))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);

  /* get output time step */
  if ((bErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetPrintStep;
  dStep  = atof(szLex);

  /* get closing parenthesis */
  if (!(bOK = GetPunct(pibIn, szLex, CH_RPAREN)))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ")", szLex);

  /* check times for consistency  */
  if ((bErr = (dEnd <= dStart))) {
    ReportError(pibIn, RE_SPECERR, "End_time must be > Start_time", NULL);
    goto Exit_GetPrintStep;
  }
  else if ((bErr = (dStep > (dEnd - dStart)))) {
    ReportError(pibIn, RE_SPECERR, "Time_step too large", NULL);
    goto Exit_GetPrintStep;
  }

  /* fix an unlikely overflow */
  dTmp = 1 + ceil((dEnd - dStart) / dStep);
  for (n = 0; n < nVars; ++n) {
    if (dTmp < LONG_MAX)
      pprintrec[n]->cTimes = (long) dTmp;
    else
      pprintrec[n]->cTimes = LONG_MAX;
  }

  for (n = 0; n < nVars; ++n) {
    if ( !(pprintrec[n]->pdTimes = InitdVector(pprintrec[n]->cTimes)))
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetPrintStep", NULL);

    for (i = 0; i < pprintrec[n]->cTimes - 1; i++)
      pprintrec[n]->pdTimes[i] = dStart + (i * dStep);

    pprintrec[n]->pdTimes[pprintrec[n]->cTimes - 1] = dEnd;

    QueueListItem(pos->plistPrintRecs, (PVOID) pprintrec[n]);
  }

Exit_GetPrintStep:

  if (bErr)
    if (!bGavePrintStepUsage) {
      printf("Syntax: %s (<Identifiers>, Start_time, End_time, Time_step)\n\n",
             GetKeyword(KM_PRINTSTEP));
      bGavePrintStepUsage = TRUE;
    }

  return(bErr);

} /* GetPrintStep */


/* ----------------------------------------------------------------------------
   GetData

   Gets the arguments to a Data() statement
*/
BOOL bGaveDataUsage = FALSE; /* prevent multiple diagnostics */

BOOL GetData (PINPUTBUF pibIn, PSTR szLex, POUTSPEC pos)
{
  PDATAREC pdatarec;
  BOOL bErr = FALSE;
  HVAR hvar;

  if (!(bErr = EGetPunct(pibIn, szLex, CH_LPAREN))) {
    if (!(bErr = ENextLex(pibIn, szLex, LX_IDENTIFIER))) {

      if ((bErr = !(hvar = GetVarHandle(szLex))))
        ReportError(pibIn, RE_UNDEFINED, szLex, NULL);

      else {
        if ( !(pdatarec = (PDATAREC) malloc(sizeof(DATAREC))))
          ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetData", NULL);

        if ( !(pdatarec->szDataName = (PSTR) malloc(MyStrlen(szLex)+1)))
          ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetData", NULL);

        MyStrcpy(pdatarec->szDataName, szLex);
        assert(pdatarec);

        pdatarec->hvar = hvar;

        bErr = GetListOfData(pibIn, pdatarec, szLex);

        if (bErr) {
          free(pdatarec->szDataName);
          free(pdatarec);
        } /* if */
        else
          QueueListItem(pos->plistDataRecs, (PVOID) pdatarec);
      } /* else */
    } /* if */
  } /* if */

  if (!bErr) bErr = GetTerminator(pibIn, szLex);
  else {
    if (!bGaveDataUsage) {
      printf ("Syntax: %s (identifier, Time1, Time2, ...)\n\n",
               GetKeyword(KM_DATA));
      bGaveDataUsage = TRUE;
    } /* if */
  } /* else */

  return(bErr);

} /* GetData */


/* ----------------------------------------------------------------------------
   GetStringArg

   tries to read a string argument from pibIn and assign it to
   *pszArg.  If pszArg is NULL, the argument is read, but no
   assigment is made.  If pszArg is not NULL, space is allocated for
   argument read.  szLex is a workspace.  If bDelim is TRUE, a
   delimiter is skipped in the input buffer.

   The return value is TRUE for error.  Errors are reported.
*/
BOOL GetStringArg (PINPUTBUF pibIn, PSTR *pszArg, PSTR szLex, BOOL bDelim)
{
  BOOL bErr;

  assert(szLex); /* Workspace must be given */

  if (bDelim)
    GetOptPunct(pibIn, szLex, ',');

  bErr = ENextLex(pibIn, szLex, LX_STRING);

  if (!bErr) {
    if (szLex[0]) {
      /* Allocate and copy the string */
      if ( !(*pszArg = (PSTR) malloc(MyStrlen(szLex) + 1)))
        ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetStringArg", NULL);

      MyStrcpy(*pszArg, szLex);
    }
    else
      *pszArg = NULL; /* No string given */
  } /* if */

  return(bErr);

} /* GetStringArg */


/* ----------------------------------------------------------------------------
   GetOutputFile

   Use a name different from the default for the regular output
*/
BOOL GetOutputFile (PINPUTBUF pibIn, PSTR szLex, PANALYSIS panal)
{
  BOOL bErr = FALSE;

  bErr = EGetPunct(pibIn, szLex, CH_LPAREN)
         || GetStringArg(pibIn, &panal->szOutfilename, szLex, FALSE);

  if (!bErr) {
    panal->bAllocatedFileName = TRUE;
    bErr = EGetPunct(pibIn, szLex, CH_RPAREN);
  }

  if (!bErr)
    bErr = GetTerminator(pibIn, szLex);
  else
    printf("Syntax: %s (szOutputFilename)\n\n", GetKeyword(KM_OUTPUTFILE));

  return(bErr);

} /* GetOutputFile */


/* ----------------------------------------------------------------------------
   GetSimulate: obsolete, give an ignore warning
*/
BOOL bGaveSimulateUsage = FALSE; /* prevent multiple diagnostics */

BOOL GetSimulate ()
{

  if (!bGaveSimulateUsage) {
    printf ("Warning: %s statements are obsolete and ignored.\n\n",
            GetKeyword(KM_SIMULATE));
    bGaveSimulateUsage = TRUE;
  }

  return(1);

} /* GetSimulate */


/* ----------------------------------------------------------------------------
   GetStartTime
   Reads the starting time for the integration (one argument).

   * If the argument is a number, the start time takes that value.

   * If the argument is an identifier which is a valid model
     parameter, the start time is defined as being dependent on that parameter.
     The symbolic parameter will be replaced by its value after model
     initialization in DoOneExperiment.
*/
BOOL bGaveSrtTUsage = FALSE; /* prevent multiple diagnostics */

BOOL GetStartTime (PINPUTBUF pibIn, PEXPERIMENT pexp)
{
  static int vrgiSimArgTypes[1] = {LX_NUMBER | LX_IDENTIFIER};

  BOOL bErr=!GetFuncArgs(pibIn, 1, vrgiSimArgTypes, vrgszlexArgs[0]);

  if (!bErr) {
    if (!DefDepParm(vrgszlexArgs[0], &pexp->dT0, &pexp->hT0))
      ReportError(pibIn, RE_EXPECTED, "StartTime spec", NULL);
  }
  else {
    if (!bGaveSrtTUsage) {
      printf("Syntax: %s (InitialTime)\n\n", GetKeyword(KM_STARTTIME));
      bGaveSrtTUsage = TRUE;
    }
  }

  return(bErr);

} /* GetStartTime */


/* ----------------------------------------------------------------------------
   GetMCMCSpec

   get the MCMC specification.
*/
BOOL GetMCMCSpec (PINPUTBUF pibIn, PEXPERIMENT pexp)
{
#define NMCMC_ARGS 8 /* # Function arguments for MCMC spec */

static int vrgiGibbsArgTypes[NMCMC_ARGS] = {LX_STRING,  LX_STRING,  LX_STRING,
                                            LX_INTEGER, LX_INTEGER, LX_INTEGER,
                                            LX_INTEGER, LX_NUMBER};

  PANALYSIS panal = (PANALYSIS) pibIn->pInfo;

  BOOL bErr= !GetFuncArgs(pibIn, NMCMC_ARGS,
                          vrgiGibbsArgTypes, vrgszlexArgs[0]);

  static char vszGibbsOutDefault[] = "MCMC.default.out";

  if (!bErr) {
    if (*vrgszlexArgs[0]) { /* Get output Filename */
      if ( !(panal->gd.szGout = (PSTR) malloc(MyStrlen(vrgszlexArgs[0]) + 1)))
        ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetMCMCSpec", NULL);

      MyStrcpy(panal->gd.szGout, vrgszlexArgs[0]);
      panal->bAllocatedFileName = TRUE;
    }
    else panal->gd.szGout = vszGibbsOutDefault;

    if (*vrgszlexArgs[1]) { /* Get restart file */
      if ( !(panal->gd.szGrestart =
            (PSTR) malloc(MyStrlen(vrgszlexArgs[1]) + 1)))
        ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetMCMCSpec", NULL);

      MyStrcpy(panal->gd.szGrestart, vrgszlexArgs[1]);
    }

    if (panal->gd.szGrestart != NULL &&
        !strcmp(panal->gd.szGout, panal->gd.szGrestart))
      ReportError(pibIn, RE_OUTISRESTART | RE_FATAL, "GetMCMCSpec", NULL);

    if (*vrgszlexArgs[2]) { /* get external data file name */
      if ( !(panal->gd.szGdata = (PSTR) malloc(MyStrlen(vrgszlexArgs[2]) + 1)))
        ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetMCMCSpec", NULL);

      MyStrcpy(panal->gd.szGdata, vrgszlexArgs[2]);
    }

    panal->gd.nMaxIter     = atol(vrgszlexArgs[3]);
    panal->gd.nSimTypeFlag = atol(vrgszlexArgs[4]); /* should be 0 to 5 */
    panal->gd.nPrintFreq   = atol(vrgszlexArgs[5]);
    panal->gd.nPrintIter   = atol(vrgszlexArgs[6]);

    panal->dSeed = atof(vrgszlexArgs[7]);

    if (((panal->gd.nSimTypeFlag==1) && (panal->gd.szGrestart == NULL)) ||
        ((panal->gd.nSimTypeFlag==2) && (panal->gd.szGrestart == NULL))) {
      printf ("Error: if simTypeFlag is one or two a restart file must be "
              "given - Exiting\n\n");
      exit(0);
    }
  } /* if */
  else {
    printf ("Syntax:\n%s (szOut, szRestart, szData, "
            "nMaxIters, simTypeFlag, nPrintFreq,\n"
            "      nIterToPrint, dSeed)\nExiting.\n\n",
            GetKeyword(KM_MCMC));
    exit(0);
  }

  if (!bErr)
   panal->iType = AT_MCMC;

  return(!bErr);

} /* GetMCMCSpec */


/* ----------------------------------------------------------------------------
   GetOptDSpec

   get the optimal design specification. It is based on the GetSetPointsSpec
   routine.
   The modification list is kept in MCVAR variation records, although this is
   not really a Monte Carlo analysis. This structure should eventually be
   changed to reflect a more general variation specification.
*/
BOOL GetOptDSpec (PINPUTBUF pibIn, PANALYSIS  panal, PSTR szLex)
{
  PMCVAR pMCVar;
  HVAR hvar;
  int iErr = 0;
  int iNLI;
  int ikwcode;

  /* Try to get open paren and filenames */
  if ((iErr = EGetPunct(pibIn, szLex, CH_LPAREN)                   ||
              GetStringArg(pibIn, &panal->gd.szGout, szLex, FALSE) ||
              GetStringArg(pibIn, &panal->gd.szGrestart, szLex, TRUE))) {
    goto Exit_GetOptDSpec;
  }
  else {
    panal->bAllocatedFileName = TRUE;
  }

  /* There has to be a restart file */
  if (!panal->gd.szGrestart)
    ReportError(pibIn, RE_SPECERR | RE_FATAL, "Missing restart file", NULL);

  /* Try to get number of parameter samples to read in */
  GetOptPunct(pibIn, szLex, ',');
  if ((iErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetOptDSpec;
  panal->mc.nRuns = atol(szLex);

  /* Try to get the random seed */
  GetOptPunct(pibIn, szLex, ',');
  if ((iErr = ENextLex(pibIn, szLex, LX_NUMBER)))
    goto Exit_GetOptDSpec;
  panal->dSeed = atof(szLex);

  /* Try to get the style (Forward or Backward) */
  GetOptPunct(pibIn, szLex, ',');
  if ((iErr = ENextLex(pibIn, szLex, LX_IDENTIFIER)))
    goto Exit_GetOptDSpec;
  ikwcode = GetKeywordCode(szLex, NULL);
  if (ikwcode == KM_FORWARD)
    panal->mc.style = forward;
  else if (ikwcode == KM_BACKWARD)
         panal->mc.style = backward;
       else {
         iErr = TRUE;
         goto Exit_GetOptDSpec;
       }

  /* Try to get identifier list */
  while ((iNLI=NextListItem(pibIn, szLex, LX_IDENTIFIER, 1, CH_RPAREN)) > 0) {
    hvar = GetVarHandle(szLex);
    if ((iErr = (!hvar || IsInput(hvar))))
      break; /* Is this reported ? */

    if ( !(pMCVar = (PMCVAR) malloc(sizeof(MCVAR))))
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetOptDSpec", NULL);

    pMCVar->hvar = hvar;
    pMCVar->iType = MCV_SETPOINTS;
    pMCVar->dParm[2] = pMCVar->dParm[3] = 0.0;

    QueueListItem(panal->mc.plistMCVars, pMCVar);

  } /* while */

  panal->mc.nSetParms = ListLength(panal->mc.plistMCVars);

  if (panal->mc.nSetParms == 0) {
    iErr = TRUE;
    printf(
    "\nError: you must specify a list of parameters to read.\n\n");
    goto Exit_GetOptDSpec;
  }

  if (!iNLI) /* List terminator */
    iErr = EGetPunct(pibIn, szLex, CH_RPAREN);
  else {
    iErr = TRUE;
    ReportError(pibIn, RE_LEXEXPECTED, "identifier", szLex);
  } /* else */

Exit_GetOptDSpec: ;

  if (iErr) {
    printf ("Syntax:\n"
            "%s (\"Output_File\", \"Param_Sample_File\", nSamples, "
            "random_seed, <Forward or Backward>, "
            "<param-id-list...>)\n\n", GetKeyword(KM_OPTDESIGN));
    printf("Exiting...\n");
    exit(0);
  }
  else
    panal->iType = AT_OPTDESIGN; /* Flag SetPoints anal if not chosen */

  return(iErr);

} /* GetOptDSpec */


/* ----------------------------------------------------------------------------
   GetDistribSpec

   reads in a Distrib statement (previously MCVary statement).
*/
BOOL bGaveMCVaryUsage = FALSE; /* prevent multiple diagnostics */

int GetDistribSpec (PINPUTBUF pibIn, PSTR szLex, PANALYSIS panal)
{
  PLIST plist;
  PMCVAR pMCVar = NULL;
  HVAR hvar;
  int n, iErr = 0;
  PSTRLEX szDummy;

  /* if we are not doing Monte Carlo, SetPoints, Optimal Design or
     MCMC sampling, then ignore this specification */
  if (panal->iType && !((panal->iType == AT_MONTECARLO) ||
                        (panal->iType == AT_SETPOINTS)  ||
                        (panal->iType == AT_OPTDESIGN)  ||
                        (panal->iType == AT_MCMC))) {
    EatStatement(pibIn);  /* Ignore this Distrib() */
    goto Exit_MCVarySpec;
  }

  /* Get the Distrib() spec. Check syntax at each element. */

  /* Get the parameter to be varied */
  if ((iErr = (EGetPunct(pibIn, szLex, CH_LPAREN) ||
               ENextLex(pibIn, szLex, LX_IDENTIFIER))))
    goto Done_GetMCVary;

  if (GetKeywordCode(szLex, NULL) == KM_DATA) { /* Data keyword used */
    /* try to get opening parenthesis */
    if (EGetPunct(pibIn, szLex, CH_LPAREN))
      exit(0); /* error */ /* this should be more graceful */

    /* get the name of variable predicted */
    ENextLex(pibIn, szLex, LX_IDENTIFIER);

    /* Data variable must exist and must be an input, state or output */
    if (!(hvar = GetVarHandle(szLex)) || IsParm(hvar))
      ReportError (pibIn, RE_LEXEXPECTED | RE_FATAL,
                  "input, output or state variable", szLex);

    /* try to read off closing parenthesis */
    if (EGetPunct(pibIn, szDummy, CH_RPAREN))
      exit(0); /* error */ /* this should be more graceful */
  }
  else
    if ((iErr = (!(hvar = GetVarHandle(szLex)) || /* Invalid variable name? */
                 IsInput(hvar))))
    {
      ReportError(pibIn, RE_LEXEXPECTED, "state, output or parameter", szLex);
      goto Done_GetMCVary;
    } /* if */

  /* Find the list in which pMCVar will be queued */
  if (panal->iCurrentDepth == 0) /* not an MCMC simulation */
    plist = panal->mc.plistMCVars;
  else { /* an MCMC simulation */
    if (!IsParm(hvar)) /* Distrib is for a likelihood definition */
      plist = panal->pCurrentLevel[panal->iCurrentDepth-1]->plistLikes;
    else
      plist = panal->pCurrentLevel[panal->iCurrentDepth-1]->plistMCVars;
  }

  if (!(pMCVar = (PMCVAR) malloc(sizeof(MCVAR))))
    ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetDistribSpec", NULL);

  if(!(pMCVar->pszName = (PSTR) malloc(strlen(szLex)+1)))
    ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetDistribSpec", NULL);

  /* Initialize pMCVar */
  strcpy(pMCVar->pszName, szLex);
  pMCVar->hvar = hvar;
  pMCVar->pdVal = &(pMCVar->dVal);
  pMCVar->iDepth = panal->iCurrentDepth - 1;
  pMCVar->plistDependents = InitList();
  pMCVar->bExptIsDep = pMCVar->bIsFixed = FALSE;
  pMCVar->lJumps = pMCVar->lCount = 0;
  pMCVar->dKernelSD = INIT_KERNELSD;
  pMCVar->bGibbs = FALSE;
  for (n = 0; n < 4; n++) {
    pMCVar->hParm[n] = 0;
    pMCVar->pMCVParent[n] = NULL;
    pMCVar->pdParm[n] = &(pMCVar->dParm[n]);
    pMCVar->iParmType[n] = 0;
  }

  /* Get the distribution type */
  GetOptPunct(pibIn, szLex, ',');
  iErr |= ENextLex(pibIn, szLex, LX_IDENTIFIER);
  pMCVar->iType = McvFromLex(szLex);
  if (iErr |= pMCVar->iType < 0) {
    ReportError(pibIn, RE_LEXEXPECTED, "distribution-type", szLex);
    goto Done_GetMCVary;
  }

  /* Get parameters of the distribution. These vary by distribution type.
     No value checking is made because assignements can be symbolic
  */
  switch (pMCVar->iType) {
    /* ----------------------------------------------------------------------*/
    case MCV_UNIFORM:
    case MCV_LOGUNIFORM: /* 2 parameters: min and max */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      pMCVar->dParm[2] = -DBL_MAX * 0.5;
      pMCVar->dParm[3] =  DBL_MAX * 0.5;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_NORMAL:     /* 2 parameters, mean and SD */
    case MCV_LOGNORMAL:  /* ~ */
    case MCV_NORMALCV:   /* 2 parameters, mean and coefficient of variation */
    case MCV_NORMALV:    /* 2 parameters, mean and VARIANCE */
    case MCV_LOGNORMALV: /* ~ */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      /* set the range */
      if ((pMCVar->iType == MCV_NORMAL) || (pMCVar->iType == MCV_NORMALCV) ||
	  (pMCVar->iType == MCV_NORMALV)) {
        pMCVar->dParm[2] = -DBL_MAX * 0.5;
        pMCVar->dParm[3] =  DBL_MAX * 0.5;
      }
      else {
        pMCVar->dParm[2] = 0.0;
        pMCVar->dParm[3] = DBL_MAX;
      }

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_HALFNORMAL: /* 1 parameter: SD; mean is set to zero */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      /* Set the mean */
      pMCVar->dParm[0] = 0.0;

      /* Set the range */
      pMCVar->dParm[2] = 0.0;
      pMCVar->dParm[3] = DBL_MAX;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_BETA:            /* 2 or 4 parameters */
    case MCV_TRUNCNORMAL:     /* 4 parameters, the last 2 are min and max */
    case MCV_TRUNCLOGNORMAL:  /* ~ */
    case MCV_TRUNCNORMALCV:   /* Coefficient of variation instead of SD */
    case MCV_TRUNCNORMALV:    /* VARIANCE instead of SD */
    case MCV_TRUNCLOGNORMALV: /* ~ */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      /* Set min-max range defaults */
      pMCVar->dParm[2] = 0.0; /* Standard range for beta */
      pMCVar->dParm[3] = 1.0;
      if ((pMCVar->iType == MCV_TRUNCNORMAL)   ||
          (pMCVar->iType == MCV_TRUNCNORMALCV) ||
          (pMCVar->iType == MCV_TRUNCNORMALV)) {
        pMCVar->dParm[2] = -DBL_MAX * 0.5;
        pMCVar->dParm[3] =  DBL_MAX * 0.5;
      }
      else if ((pMCVar->iType == MCV_TRUNCLOGNORMAL) ||
               (pMCVar->iType == MCV_TRUNCLOGNORMALV))
        pMCVar->dParm[3] = DBL_MAX;

      /* Look if a min-max range is included. For truncated types
         it is required. */
      SkipWhitespace(pibIn);
      if ((pMCVar->iType == MCV_BETA) && NextChar(pibIn) == CH_RPAREN)
        break; /* The spec is finished */

      /* Get the min and max */
      if ((iErr = GetDistribParam(pibIn, szLex, plist, 2, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 3, pMCVar)))
        goto Done_GetMCVary;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_CHI2: /* only one parameter: degrees of freedom */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      /* set the range */
      pMCVar->dParm[2] = 0.0;
      pMCVar->dParm[3] = DBL_MAX;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_BINOMIAL: /* 2 parameters, p and N */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      /* set the range */
      pMCVar->dParm[2] = 0.0; /* minimum */
      if (pMCVar->iParmType[1] != MCVP_FIXD)
        /* N symbolic: could be anything: assign DBL_MAX as maximum */
        pMCVar->dParm[3] = DBL_MAX; /* FB 18/07/97 */
      else
        /* N numeric: assign it as maximum */
        pMCVar->dParm[3] = pMCVar->dParm[1];

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_NEGATIVEBINOM: /* 2 parameters, r and p */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      /* set the range */
      pMCVar->dParm[2] = 0.0;     /* minimum */
      pMCVar->dParm[3] = DBL_MAX; /* maximum */

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_PIECEWISE: /* 4 parameters, note the particular order */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 2, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 3, pMCVar)))
        goto Done_GetMCVary;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_EXPONENTIAL: /* only one parameter: inverse scale */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      /* set the range */
      pMCVar->dParm[2] = 0.0;
      pMCVar->dParm[3] = DBL_MAX;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_GGAMMA:
    case MCV_INVGGAMMA: /* 2 parameter: shape and inverse scale */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      /* set the range */
      pMCVar->dParm[2] = 0.0;
      pMCVar->dParm[3] = DBL_MAX;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_TRUNCINVGGAMMA: /* 4 parameter: shape, inverse scale and bounds */

#ifndef HAVE_LIBGSL
      printf("Warning: The truncated inverse gamma density cannot be\n");
      printf("         used in MCMC simulations if the GNU Scientific\n");
      printf("         Library is not installed.\n");
#endif

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      /* Get the min and max */
      if ((iErr = GetDistribParam(pibIn, szLex, plist, 2, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 3, pMCVar)))
        goto Done_GetMCVary;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_POISSON: /* 1 parameter: rate */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      /* set the range */
      pMCVar->dParm[2] = 0.0;
      pMCVar->dParm[3] = DBL_MAX;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_BINOMIALBETA: /* 3 parameter: mean, alpha,    beta */
    case MCV_GENLOGNORMAL: /* 3 parameter: mean, sdnorm,   sdlognorm */
    case MCV_STUDENTT:     /* 3 parameter: dof,  location, scale */

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 1, pMCVar)))
        goto Done_GetMCVary;

      if ((iErr = GetDistribParam(pibIn, szLex, plist, 2, pMCVar)))
        goto Done_GetMCVary;

      /* set the last parameter, unused */
      pMCVar->dParm[3] = DBL_MAX;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_CAUCHY:     /* one parameter: scale */

       if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      pMCVar->dParm[1] =  DBL_MAX;
      pMCVar->dParm[2] = -DBL_MAX * 0.5;
      pMCVar->dParm[3] =  DBL_MAX * 0.5;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_HALFCAUCHY: /* one parameter: scale */

       if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      pMCVar->dParm[1] = DBL_MAX * 0.5;
      pMCVar->dParm[2] = 0;
      pMCVar->dParm[3] = DBL_MAX;

      break;

    /* ----------------------------------------------------------------------*/
    case MCV_USERLL: /* one parameter: the model computed likelihood */

      if (!panal->pCurrentLevel[panal->iCurrentDepth-1] ||
          (plist != panal->pCurrentLevel[panal->iCurrentDepth-1]->plistLikes)) {
        printf("UserSpecifiefLL can only be used in Likelihood().");
        iErr = 1;
        goto Done_GetMCVary;
      }

       if ((iErr = GetDistribParam(pibIn, szLex, plist, 0, pMCVar)))
        goto Done_GetMCVary;

      pMCVar->dParm[1] = DBL_MAX * 0.5;
      pMCVar->dParm[2] = 0;
      pMCVar->dParm[3] = DBL_MAX;

      break;

    /* ----------------------------------------------------------------------*/
    default:
        ReportRunTimeError(panal, RE_UNKNOWNDIST | RE_FATAL, "GetDistribSpec");
        break;

  } /* switch */

  EGetPunct(pibIn, szLex, CH_RPAREN);

  /* Check for a range error if the bounds are numeric. If there is a problem,
     correct it, but issue a warning in case this is wrong. */
  if ((pMCVar->iParmType[2] == MCVP_FIXD) &&
      (pMCVar->iParmType[3] == MCVP_FIXD) &&
      (pMCVar->dParm[3] < pMCVar->dParm[2])) {
    double dTmp = pMCVar->dParm[3];    /* Swap ranges */
    pMCVar->dParm[3] = pMCVar->dParm[2];
    pMCVar->dParm[2] = dTmp;
    ReportError(pibIn, RE_MAXMIN_RANGE | RE_WARNING, NULL, NULL);
  }

  /* If there's no error at this point, queue the variation in
     the Monte Carlo record(s). */
  if (!iErr) {
    QueueListItem(plist, pMCVar);
  } /* if */

Done_GetMCVary: ;

  if (iErr) {
    if (pMCVar) free(pMCVar);

    if (!bGaveMCVaryUsage) {
      printf("\nSyntax: Check the syntax of %s.\n", GetKeyword (KM_MCVARY));
      bGaveMCVaryUsage = TRUE;
    }

    ReportError(pibIn, RE_SYNTAXERR | RE_FATAL, NULL, NULL);
  }

Exit_MCVarySpec: ;

  return(iErr);

} /* GetDistribSpec */


/* ----------------------------------------------------------------------------
   CheckDistribParam

   If the nth distribution parameter for hvar2 is a parameter and the same as
   the variable hvar1 for which the Distrib statement is specified,
   check false. Used to check that you don't have parameter self-dependency
   at level 0.
*/
BOOL CheckDistribParam (PLIST plist, HVAR hvar1, HVAR hvar2) {
  int n;
  PLISTELEM p = plist->pleHead;
  PMCVAR pMCVar;

  if (plist == NULL) return TRUE;

  for (n = 0; n < plist->iSize; ++n) {
    pMCVar = (PMCVAR) p->pData;
    if (hvar2 == pMCVar->hvar) {
      if ((pMCVar->iParmType[0] == MCVP_PARM) && (hvar1 == pMCVar->hParm[0]))
        return FALSE;
      if ((pMCVar->iParmType[1] == MCVP_PARM) && (hvar1 == pMCVar->hParm[1]))
        return FALSE;
      if ((pMCVar->iParmType[2] == MCVP_PARM) && (hvar1 == pMCVar->hParm[2]))
        return FALSE;
      if ((pMCVar->iParmType[3] == MCVP_PARM) && (hvar1 == pMCVar->hParm[3]))
        return FALSE;
    }
    p = p->pleNext;
  }

  return TRUE;

} /* CheckDistribParam */


/* ----------------------------------------------------------------------------
   GetDistribParam

   Determine if argument `n' of the Distrib statement is a variable name or
   a number; set the parameter accordingly.

   If the argument is a variable, set a pointer to its MC structure
*/
int GetDistribParam (PINPUTBUF pibIn, PSTR szLex, PLIST plist, int n,
                     PMCVAR pMCVar) {
  PANALYSIS panal = (PANALYSIS) pibIn->pInfo;
  int iLex, iCode;
  HVAR hvar;

  GetOptPunct(pibIn, szLex, ',');
  if (n != 3)
    NextLex(pibIn, szLex, &iLex);
  else {
    SkipWhitespace(pibIn);
    iLex = LX_NULL;
    if (NextChar(pibIn) != CH_RPAREN)
      NextLex(pibIn, szLex, &iLex);
  }

  if (iLex == LX_IDENTIFIER) { /* symbol used for parameter */

    iCode = GetKeywordCode_in_context(szLex, CN_FUNCARG);

    if ((iCode == KM_PREDICTION) || (iCode == KM_DATA)) {
      /* Prediction or Data keywords used */

      /* Only inputs, states and outputs can have predicted or data
         parameters  */
      if (IsParm(pMCVar->hvar))
        ReportError(pibIn, RE_BADCONTEXT | RE_FATAL, szLex, NULL);

      /* Try to get opening parenthesis */
      if (EGetPunct(pibIn, szLex, CH_LPAREN))
        return 1; /* error */

      /* Get the name of variable specified */
      NextLex(pibIn, szLex, &iLex);

      /* Specified variable must exist and be input, state or output */
      if (!(hvar = GetVarHandle(szLex)) || IsParm(hvar))
        ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL,
                   "input, output or state variable", szLex);

      /* Try to get closing parenthesis */
      if (EGetPunct(pibIn, szLex, CH_RPAREN))
        return 1; /* error */

    } /* end of Prediction/Data case */
    else {
      /* No keyword used, a regular symbol: that symbol should
         be declared and should not be an input or an output or a state
         (because those should use keywords); so it should be
         a parameter - FB 18/10/98 */
      if (!(hvar = GetVarHandle(szLex)) || !IsParm(hvar))
        ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, "parameter", szLex);
    }

    /* No self-dependency at level 0 allowed, except in Optimal design */
    if (!(panal->iType == AT_OPTDESIGN) &&
        ((panal->iCurrentDepth == 0 && hvar == pMCVar->hvar) ||
         !CheckDistribParam(plist, pMCVar->hvar, hvar)))
      ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, "valid parameter", szLex);

    /* Declare it a symbolic parameter */
    if (iCode == KM_PREDICTION)
      pMCVar->iParmType[n] = MCVP_PRED;
    else if (iCode == KM_DATA)
      pMCVar->iParmType[n] = MCVP_DATA;
    else
      pMCVar->iParmType[n] = MCVP_PARM;

    /* Attach handle */
    pMCVar->hParm[n] = hvar;

  }
  else /* i.e. not an identifier */
    if (iLex == LX_FLOAT || iLex == LX_INTEGER) {
      pMCVar->iParmType[n] = MCVP_FIXD;
      pMCVar->dParm[n] = atof(szLex);
    }
    else {
      /* allow max to be absent - set to default */
      if (n == 3) {
        pMCVar->iParmType[n] = MCVP_FIXD;
        pMCVar->dParm[n] = DBL_MAX;
      }
      else
        return 1; /* error */
  }

  return 0; /* OK */

} /* GetDistribParam */


/* ----------------------------------------------------------------------------
   GetSetPointsSpec

   Reads the SetPoints() arguments. The modification list is kept
   in MCVAR variation records, although this is not really a Monte
   Carlo analysis.  This structure should eventually be changed to
   reflect a more general variation specification.
*/
int GetSetPointsSpec (PINPUTBUF pibIn, PANALYSIS  panal, PSTR szLex)
{
  PMCVAR  pMCVar;
  PSTRLEX szTmp;
  HVAR    hvar;
  int     iErr = 0;
  int     iNLI;
  long    j, iLB, iUB;

  /* MonteCarlo sampling can be mixed with SetPoints sampling if Distrib
     specs appear after the SetPoint spec */
  if (ListLength(panal->mc.plistMCVars) > 0) {
    printf ("Error: Distrib() statements can only appear after the SetPoints()"
            "specification, not before - Exiting\n\n");
    exit(0);
  }

  /* Try to get open paren and filenames */
  if ((iErr = EGetPunct(pibIn, szLex, CH_LPAREN) ||
              GetStringArg(pibIn, &panal->mc.szMCOutfilename, szLex, FALSE) ||
              GetStringArg(pibIn, &panal->mc.szSetPointsFilename, szLex,
                            TRUE))) {
    goto Exit_GetSetPointsSpec;
  }
  else {
    panal->bAllocatedFileName = TRUE;
  }

  /* There has to be a restart file */
  if (!panal->mc.szSetPointsFilename)
    ReportError(pibIn, RE_SPECERR | RE_FATAL, "Missing setpoints file", NULL);

  /* Output file and setpoint file have to be different */
  if (!MyStrcmp(panal->mc.szMCOutfilename, panal->mc.szSetPointsFilename))
    ReportError(pibIn, RE_SPECERR | RE_FATAL, "Same name for 2 files", NULL);

  /* Try to get number of runs */
  GetOptPunct(pibIn, szLex, ',');
  if ((iErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetSetPointsSpec;

  panal->mc.nRuns = atol(szLex);

  /* Try to get identifier list */
  while ((iNLI=NextListItem(pibIn, szLex, LX_IDENTIFIER, 1, CH_RPAREN)) > 0) {
    /* check if szLex is a scalar or an array and act accordingly */
    iLB = iUB = -1;
    if (GetPunct(pibIn, szTmp, '[')) /* array found, read bounds */
      GetArrayBounds(pibIn, &iLB, &iUB);

    if (iUB == -1) { /* scalar, store and continue */
      hvar = GetVarHandle(szLex);
      if ((iErr = (!hvar || IsInput(hvar))))
        break; /* will generate an error message */

      if ( !(pMCVar = (PMCVAR) malloc(sizeof(MCVAR))))
        ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetSetPointsSpec", NULL);

      pMCVar->hvar = hvar;
      pMCVar->iType = MCV_SETPOINTS;
      pMCVar->dParm[2] = pMCVar->dParm[3] = 0.0;

      QueueListItem(panal->mc.plistMCVars, pMCVar);
    }
    else { /* array */

      for (j = iLB; j < iUB; j++) {
        sprintf(szTmp, "%s_%ld", szLex, j); /* create names */

        hvar = GetVarHandle(szTmp);
        if ((iErr = (!hvar || IsInput(hvar))))
          break; /* will generate an error message */

        if ( !(pMCVar = (PMCVAR) malloc(sizeof(MCVAR))))
          ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetSetPointsSpec", NULL);

        pMCVar->hvar = hvar;
        pMCVar->iType = MCV_SETPOINTS;
        pMCVar->dParm[2] = pMCVar->dParm[3] = 0.0;

        QueueListItem(panal->mc.plistMCVars, pMCVar);
      }
    } /* end else */

  } /* while */

  panal->mc.nSetParms = ListLength(panal->mc.plistMCVars);

  /* FB 19 nov 96 */
  if (panal->mc.nSetParms == 0) {
    iErr = TRUE;
    printf(
    "\nError: you must specify a list of parameters to read.\n\n");
    goto Exit_GetSetPointsSpec;
  }

  if (!iNLI) /* List terminator */
    iErr = ((szTmp[0] != CH_RPAREN) && (EGetPunct(pibIn, szLex, CH_RPAREN))) ||
           InitSetPoints(&panal->mc);
  else {
    iErr = TRUE;
    ReportError(pibIn, RE_LEXEXPECTED, "identifier", szLex);
  } /* else */

Exit_GetSetPointsSpec: ;

  if (iErr) {
    printf ("Syntax:\n"
             "%s (\"OutputFile\", \"SetPtsFile\", nRuns, "
             "<param-id-list...>)\n\n", GetKeyword(KM_SETPOINTS));
    printf("Exiting...\n");
    exit(0);
  }
  else
    panal->iType = AT_SETPOINTS; /* Flag SetPoints anal */

  return(iErr);

} /* GetSetPointsSpec */


/* ----------------------------------------------------------------------------
   GetMonteCarloSpec
*/
int GetMonteCarloSpec (PINPUTBUF pibIn, PANALYSIS panal, PSTR szLex)
{
#define NMC_ARGS 3     /* 3 MonteCarlo Args */

static int vrgiMCArgTypes[NMC_ARGS] = {LX_STRING, LX_INTEGER, LX_NUMBER};

  int iErr = 0;

  iErr = !GetFuncArgs(pibIn, NMC_ARGS, vrgiMCArgTypes, vrgszlexArgs[0]);

  if (!iErr) {
    if (*vrgszlexArgs[0]) {
      if ( !(panal->mc.szMCOutfilename =
             (PSTR) malloc(MyStrlen(vrgszlexArgs[0]) + 1)))
        ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetMonteCarloSpec", NULL);

      MyStrcpy(panal->mc.szMCOutfilename, vrgszlexArgs[0]);
      panal->bAllocatedFileName = TRUE;
    }

    panal->mc.nRuns = atol(vrgszlexArgs[1]);
    panal->dSeed = atof(vrgszlexArgs[2]);
  } /* if */
  else
    printf ("Syntax: %s (szOutfilename, nRuns, dSeed)\n\n",
            GetKeyword(KM_MONTECARLO));

  if (!iErr)
    panal->iType = AT_MONTECARLO; /* Flag as MC */

  return(iErr);

} /* GetMonteCarloSpec */


/* ----------------------------------------------------------------------------
   GetParmMod

   Reads a variable assignment and store the assigned value (or link if
   symbolic assignment)
*/
BOOL GetParmMod (PINPUTBUF pibIn, PSTRLEX szLex)
{
  HVAR hvar = GetVarHandle(szLex);
  PANALYSIS panal = (PANALYSIS) pibIn->pInfo;
  PEXPERIMENT pexp = panal->pexpCurrent;

  PSTRLEX szPunct;
  int  iErr;
  PVARMOD pvarmod; /* Pointer to the variable modification */

  if ((iErr = !hvar))
    ReportError(pibIn, RE_LEXEXPECTED, "model-variable", szLex);

  else {
    /* Allocate space and initialize modification */

    if ( !(pvarmod = (PVARMOD) malloc(sizeof(VARMODIFICATION))))
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetParmMod", NULL);

    pvarmod->hvar = hvar; /* The variable handle */

    if (!GetOptPunct(pibIn, szPunct, '=')) { /* Try to get '=' */
      iErr = szPunct[1] = '=';
      ReportError(pibIn, RE_EXPECTED, szPunct, NULL);
    }

    else if (IsInput(hvar)) { /* Process INPUT */
      if ( !(pvarmod->uvar.pifn = (PIFN) malloc(sizeof(IFN))))
        ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetParmMod", NULL);

      iErr = !pvarmod->uvar.pifn
             || !GetInputFn(pibIn, NULL, pvarmod->uvar.pifn);
      if (iErr) {
        free(pvarmod->uvar.pifn); /* Cleanup if error */
        pvarmod->uvar.pifn = NULL;
      }
    } /* else if */
    else { /* PARM, STATE, etc */
      if (!(iErr = ENextLex(pibIn, szLex, LX_NUMBER)))
        pvarmod->uvar.dVal = atof(szLex);
    }

    if (!iErr) { /* No errors, add mod to list */
      if(panal->iCurrentDepth == 0 || panal->wContext == CN_EXPERIMENT)
        QueueListItem(pexp->plistParmMods, pvarmod);
      else
        QueueListItem(panal->pCurrentLevel[panal->iCurrentDepth-1]->plistVars,
                      pvarmod);
      iErr = GetTerminator(pibIn, szLex);
    } /* if */
    else /* Invalid mod, cleanup */
      free(pvarmod);

  } /* else valid id */

  return((BOOL) iErr);

} /* GetParmMod */


/* ----------------------------------------------------------------------------
   GetParmMod2

   Store the assigned value (or link if symbolic assignment) passed by szeqn.
*/
BOOL GetParmMod2 (PINPUTBUF pibIn, PSTRLEX szLex, PSTREQN szEqn)
{
  int         iErr;
  PVARMOD     pvarmod; /* Pointer to the variable modification */

  HVAR        hvar = GetVarHandle(szLex);
  PANALYSIS   panal = (PANALYSIS) pibIn->pInfo;
  PEXPERIMENT pexp = panal->pexpCurrent;

  if ((iErr = !hvar))
    ReportError(pibIn, RE_LEXEXPECTED, "model-variable", szLex);

  else { /* valid ID */
    /* Allocate space and initialize modification */

    if ( !(pvarmod = (PVARMOD) malloc(sizeof(VARMODIFICATION))))
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetParmMod", NULL);

    pvarmod->hvar = hvar; /* The variable handle */

    if (IsInput(hvar)) { /* Process INPUT */
      if ( !(pvarmod->uvar.pifn = (PIFN) malloc(sizeof(IFN))))
        ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "GetParmMod", NULL);

      iErr = !pvarmod->uvar.pifn
             || !GetInputFn(pibIn, NULL, pvarmod->uvar.pifn);
      if (iErr) {
        free(pvarmod->uvar.pifn); /* Cleanup if error */
        pvarmod->uvar.pifn = NULL;
      }
    }
    else { /* PARM, STATE, etc */
      pvarmod->uvar.dVal = atof(szEqn);
    }

    if (!iErr) { /* No errors, add mod to list */
      if(panal->iCurrentDepth == 0 || panal->wContext == CN_EXPERIMENT)
        QueueListItem(pexp->plistParmMods, pvarmod);
      else
        QueueListItem(panal->pCurrentLevel[panal->iCurrentDepth-1]->plistVars,
                      pvarmod);
    }
    else /* Invalid mod, cleanup */
      free(pvarmod);

  } /* else valid id */

  return((BOOL) iErr);

} /* GetParmMod2 */


/* ----------------------------------------------------------------------------
   NewExperiment

   creates a new experiment in the analysis and copies global defaults.
*/
void NewExperiment (PINPUTBUF pibIn)
{
  PANALYSIS panal = (PANALYSIS)pibIn->pInfo;
  PLEVEL plevel;
  int n;

  /* Allocate new experiment and assign list and current pointers */

  if (panal->iCurrentDepth < 0) { /* something is real wrong - FB 03/08/97 */
     ReportError (pibIn, RE_LEXEXPECTED | RE_FATAL, "Level statement",
                  "Simulation");
  }

  if (panal->iCurrentDepth == 0) {
    panal->expGlobal.iExp++;    /* Increment number of experiment */
    panal->pexpCurrent = panal->rgpExps[panal->expGlobal.iExp - 1] =
                         (PEXPERIMENT) malloc(sizeof(EXPERIMENT));
    if (!panal->pexpCurrent)
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "NewExperiment()", NULL);

    if (panal->rank == 0)
      printf("Reading experiment %d.\n", panal->expGlobal.iExp);
  }
  else {
    plevel = panal->pLevels[panal->iInstances - 1];
    for (n = 0; n < panal->iCurrentDepth-1; ++n) {
      plevel = plevel->pLevels[plevel->iInstances - 1];
    }
    if (plevel->iInstances == MAX_INSTANCES - 1)
      ReportError(pibIn, RE_TOOMANYINST | RE_FATAL, "NewExperiment", NULL);
    n = panal->pCurrentLevel[panal->iCurrentDepth - 1]->iInstances++;
    if (!(plevel = plevel->pLevels[n] = (PLEVEL)malloc(sizeof(LEVEL))))
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "NewExperiment", NULL);
    plevel->iInstances = 0;
    plevel->iSequence = n + 1;
    plevel->iDepth = panal->iCurrentDepth;
    panal->pCurrentLevel[panal->iCurrentDepth++] = plevel;
    if (panal->iDepth < panal->iCurrentDepth)
      panal->iDepth = panal->iCurrentDepth;

    plevel->nMCVars = plevel->nFixedVars = plevel->nLikes = 0;
    plevel->plistVars = InitList();
    plevel->plistMCVars = InitList();
    plevel->plistLikes = InitList();

    if (!(plevel->pexpt = (PEXPERIMENT) malloc(sizeof(EXPERIMENT))))
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "NewExperiment", NULL);

    panal->pexpCurrent = plevel->pexpt;
    panal->pexpCurrent->iExp = panal->expGlobal.iExp = ++panal->iExpts;
    panal->wContext = CN_EXPERIMENT;

    if (panal->rank == 0)
      printf ("Simulation %d - depth %d, instance %d\n", panal->iExpts,
              panal->iCurrentDepth,
              panal->pCurrentLevel[panal->iCurrentDepth-2]->iInstances);
  }

  memcpy(panal->pexpCurrent, &panal->expGlobal, sizeof(EXPERIMENT));
  panal->wContext = CN_EXPERIMENT;
  panal->pexpCurrent->plistParmMods = InitList();    /* Local mods */
  panal->pexpCurrent->os.plistPrintRecs = InitList();
  panal->pexpCurrent->os.plistDataRecs = InitList();

} /* NewExperiment */


/* ----------------------------------------------------------------------------
   EndExperiment

   cleans up at the end of defining a new experiment section.
*/
BOOL EndExperiment (PINPUTBUF pibIn, PANALYSIS panal)
{
  BOOL bReturn;

  bReturn = !ErrorsReported(pibIn);

  if (!bReturn) {
    /* Experiment had errors.  Cleanup this space and continue */
    ReportError(pibIn, RE_ERRORSINEXP | RE_FATAL,
         (PSTR) &panal->pexpCurrent->iExp, NULL);
    ClearErrors(pibIn);
    panal->rgpExps[--panal->expGlobal.iExp] = NULL;
    free(panal->pexpCurrent);
  } /* if */

  else {
    /* Create space for outputs and data */
    PrepareOutSpec(panal->pexpCurrent);
  }

  /* Reset current exp to global context. */

  panal->pexpCurrent = &panal->expGlobal;
  panal->wContext = CN_GLOBAL;

  if (panal->iType == AT_MCMC && panal->iCurrentDepth-- == 0)
    return FALSE;

  return(bReturn);

} /* EndExperiment */


/* ----------------------------------------------------------------------------
   SetLevel

   Encountered `Level' keyword, increments number of levels, allocates
   structure, initializes
*/
int SetLevel(PINPUTBUF pibIn)
{
  PSTRLEX   szPunct;
  PANALYSIS panal = (PANALYSIS)pibIn->pInfo;
  PLEVEL    plevel;
  int       n;

  if (panal->iType != AT_MCMC)
    ReportError(pibIn, RE_TYPENOTMCMC | RE_FATAL, "SetLevel", NULL);

  if (panal->iCurrentDepth == MAX_LEVELS)
    ReportError(pibIn, RE_TOOMANYLEVELS | RE_FATAL, "SetLevel", NULL);

  if (panal->wContext == CN_EXPERIMENT)
    ReportError(pibIn, RE_LEVINEXPT | RE_FATAL, "SetLevel", NULL);

  if (EGetPunct(pibIn, szPunct, CH_LBRACE))
    return 1;

  if (panal->iCurrentDepth == 0) {

    plevel = panal->pLevels[panal->iInstances++]
           = (PLEVEL) malloc(sizeof(LEVEL));
    if (plevel == NULL)
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "SetLevel", NULL);

    /* we do not want more than 1 top level, exit */
    if (panal->iInstances > 1) {
      printf("Error: only one top level is allowed - Exiting.\n");
      exit(0);
    }

    plevel->iSequence = panal->iInstances;
    if (panal->rank == 0)
      printf("New level - depth 1, instance %d\n", panal->iInstances);

  }
  else {

    plevel = panal->pLevels[panal->iInstances-1];

    for (n = 0; n < panal->iCurrentDepth-1; ++n)
      plevel = plevel->pLevels[plevel->iInstances - 1];

    if (plevel->iInstances == MAX_INSTANCES - 1)
      ReportError(pibIn, RE_TOOMANYINST | RE_FATAL, "SetLevel", NULL);

    n = panal->pCurrentLevel[panal->iCurrentDepth-1]->iInstances++;

    plevel = plevel->pLevels[n]
           = (PLEVEL)malloc(sizeof(LEVEL));

    if (plevel == NULL)
      ReportError(pibIn, RE_OUTOFMEM | RE_FATAL, "SetLevel", NULL);

    plevel->iSequence = n + 1;
    if (panal->rank == 0)
      printf("New level - depth %d, instance %d\n", panal->iCurrentDepth + 1,
             panal->pCurrentLevel[panal->iCurrentDepth-1]->iInstances);
  }

  plevel->iInstances = 0;
  plevel->iDepth = panal->iCurrentDepth;
  panal->pCurrentLevel[panal->iCurrentDepth++] = plevel;

  if (panal->iDepth < panal->iCurrentDepth)
    panal->iDepth = panal->iCurrentDepth;

  plevel->nMCVars = plevel->nFixedVars = plevel->nLikes = 0;
  plevel->plistVars = InitList();
  plevel->plistMCVars = InitList();
  plevel->plistLikes = InitList();
  plevel->pexpt = NULL;

  return 0;

} /* SetLevel */


/* ----------------------------------------------------------------------------
   EndLevel
*/
BOOL EndLevel (PANALYSIS panal)
{
  if(panal->iCurrentDepth-- == 0)
    return FALSE;
  return TRUE;

} /* EndLevel */


/* ----------------------------------------------------------------------------
   FreeLevels
*/
void FreeLevels (PANALYSIS panal)
{
  int n;

  for (n = 0; n < panal->iInstances; n++)
    if (panal->pLevels[n] != NULL)
      FreeOneLevel(panal->pLevels[n]);

  if (panal->bAllocatedFileName)
    free(panal->gd.szGout);

  FreeList(&panal->mc.plistMCVars, NULL, TRUE);
  FreeList(&panal->expGlobal.plistParmMods, NULL, TRUE);

  free(panal->expGlobal.is.iwork);
  free(panal->expGlobal.is.rwork);

  free(panal->modelinfo.pStateHvar);
  free(panal);

} /* FreeLevels */


/* ----------------------------------------------------------------------------
   FreeMCVar
*/
int FreeMCVar (PVOID pData, PVOID pUserInfo)
{
  PMCVAR pMCVar = (PMCVAR) pData;
  FreeList(&pMCVar->plistDependents, NULL, TRUE);
  free(pMCVar->pszName);
  /* dummy */
  return 0;

} /* FreeMCVar */


/* ----------------------------------------------------------------------------
   FreeDataRec

   Partial freeing (pdData is left in memory)
*/
int FreeDataRec (PVOID pData, PVOID pUserInfo)
{
  PDATAREC pDataRecord = (PDATAREC) pData;

  /* do not free(pDataRecord->pdData); */
  free(pDataRecord->szDataName);
  free(pDataRecord);
  return 0;

} /* FreeDataRec */


/* ----------------------------------------------------------------------------
   FreePrintRec
*/
int FreePrintRec (PVOID pData, PVOID pUserInfo)
{
  PPRINTREC pPrintRecord = (PPRINTREC) pData;

  free(pPrintRecord->pdTimes);
  free(pPrintRecord->szOutputName);
  free(pPrintRecord);
  return 0;

} /* FreePrintRec */


/* ----------------------------------------------------------------------------
   FreeOneLevel (recursive)
*/
void FreeOneLevel (PLEVEL plevel)
{
  int n;

  for (n = 0; n < plevel->iInstances; n++)
    if (plevel->pLevels[n] != NULL)
      FreeOneLevel(plevel->pLevels[n]);

  FreeList(&plevel->plistVars, NULL, TRUE);

  ForAllList(plevel->plistMCVars, &FreeMCVar, NULL);
  FreeList(&plevel->plistMCVars, NULL, TRUE);

  ForAllList(plevel->plistLikes, &FreeMCVar, NULL);
  FreeList(&plevel->plistLikes, NULL, TRUE);

  if (plevel->pexpt != NULL) {
    FreeList(&plevel->pexpt->plistParmMods, &FreeVarMod, FALSE);

    POUTSPEC pos = &plevel->pexpt->os;
    free(pos->pszOutputNames);
    free(pos->phvar_out);
    free(pos->pcOutputTimes);
    free(pos->piCurrentOut);
    free(pos->prgdOutputTimes);
    for (n = 0; n < pos->nOutputs; n++)
      free(pos->prgdOutputVals[n]);
    free(pos->prgdOutputVals);
    free(pos->rgdDistinctTimes);
    ForAllList(pos->plistPrintRecs, &FreePrintRec, NULL);
    FreeList(&pos->plistPrintRecs, NULL, FALSE);
    free(pos->plistPrintRecs);

    free(pos->pcData);
    free(pos->phvar_dat);
    free(pos->pszDataNames);
    for (n = 0; n < pos->nData; n++)
      free(pos->prgdDataVals[n]);
    free(pos->prgdDataVals);
    //ForAllList(pos->plistDataRecs, &FreeDataRec, NULL);
    //FreeList(&pos->plistDataRecs, NULL, FALSE);
    //free(pos->plistDataRecs);

    free(plevel->pexpt);
  }
  if (plevel->nFixedVars > 0)
    free(plevel->rgpFixedVars);
  /* for (n = 0; n < plevel->nMCVars; n++)  */
  /*   FreeList(&plevel->rgpMCVars[n]->plistDependents, NULL, TRUE); */
  if (plevel->nMCVars > 0)
    free(plevel->rgpMCVars);
    //if (plevel->nLikes > 0) {
    //printf("plevel->nLikes %ld\n", plevel->nLikes);
    //for (n = 0; n < plevel->nLikes; n++) {
      //printf("plevel->rgpLikes[n]-> %lu\n", &plevel->rgpLikes[n]);
      /* clean array of MCVars */
      //if (&plevel->rgpLikes[n] != NULL)
      //FreeMCVar ((PVOID) (plevel->rgpLikes[n]), NULL);
    //}
  free(plevel->rgpLikes);
  //}

  free(plevel);

} /* FreeOneLevel */


/* ----------------------------------------------------------------------------
   ProcessWord

   processes the word szLex.

   This is the main loop of the interpreter.  It is a big switch that
   recognizes keywords that are specifications and takes the
   appropriate action.

   If the word szLex is not a keyword, ProcessWord() attempts to
   define a parameter specification.
*/
void ProcessWord (PINPUTBUF pibIn, PSTR szLex, PSTR szEqn)
{
  int       iErr = 0;
  int       iKWCode, fContext;
  long      i, iLB, iUB;
  PSTREQN   szEqnU;
  PSTRLEX   szTmp;
  PANALYSIS panal;

  if (!pibIn || !szLex || !szLex[0] || !szEqn)
    return;

  panal = (PANALYSIS) pibIn->pInfo;

  iKWCode = GetKeywordCode(szLex, &fContext);

  assert(panal->wContext != CN_END);

  if ((iErr =
        (iKWCode                                 /* Is a keyword */
         && !(fContext & panal->wContext))))     /* In invalid context */
    ReportError(pibIn, RE_BADCONTEXT, szLex, NULL);

  else {
    switch (iKWCode) {

    default: /* If a keyword is not found, try to get an assignment */

      /* check if it is an array */
      iLB = iUB = -1;
      if (GetPunct(pibIn, szTmp, '[')) /* array found, read bounds */
        GetArrayBounds(pibIn, &iLB, &iUB);
      else
        if (!strcmp(szTmp, "=")) /* put it back... */
          pibIn->pbufCur--;

      if (iUB == -1) { /* scalar */
        iErr = GetParmMod(pibIn, szLex);
      }
      else { /* array */

        if (GetPunct(pibIn, szTmp, '=')) { /* read assignment */
          GetStatement(pibIn, szEqn);	
          for (i = iLB; i < iUB; i++) {
            sprintf(szTmp, "%s_%ld", szLex, i); /* create names */
            UnrollEquation(pibIn, i, szEqn, szEqnU);
            iErr = GetParmMod2(pibIn, szTmp, szEqnU);
            if (iErr)
	      break;
          }
        }
        else {
          ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, "= or [", NULL);
        }
      }
      break;

    /* Otherwise process the following keywords */

    case KM_PRINT:
      iErr = GetPrint(pibIn, szLex, &panal->pexpCurrent->os);
      break;

    case KM_PRINTSTEP:
      iErr = GetPrintStep(pibIn, szLex, &panal->pexpCurrent->os);
      break;

    case KM_EXPERIMENT:
      if (!(iErr = EGetPunct(pibIn, szTmp, CH_LBRACE)))
        NewExperiment(pibIn);
      break;

    case KM_LEVEL:
      iErr = SetLevel(pibIn);
      break;

    case KM_MCVARY:
      iErr = GetDistribSpec(pibIn, szLex, panal);
      break;

    case KM_OUTPUTFILE:
      if (panal->szOutfilename)
        EatStatement(pibIn);
      else
        iErr = GetOutputFile(pibIn, szLex, panal);
      break;

    case KM_DATA:
      iErr = GetData(pibIn, szLex, &panal->pexpCurrent->os);
      break;

    case KM_INTEGRATE:
      iErr = GetIntegrate(pibIn, szLex, &panal->pexpCurrent->is);
      break;

    case KM_MCMC:
      iErr = GetMCMCSpec(pibIn, panal->pexpCurrent);
      break;

    case KM_OPTDESIGN:
      iErr = GetOptDSpec(pibIn, panal, szLex);
      break;

    case KM_MONTECARLO:
      iErr = GetMonteCarloSpec(pibIn, panal, szLex);
      break;

    case KM_SETPOINTS:
      iErr = GetSetPointsSpec(pibIn, panal, szLex);
      break;

    case KM_SIMULATE:
      iErr = GetSimulate();
      break;

    case KM_STARTTIME:
      iErr = GetStartTime(pibIn, panal->pexpCurrent);
      break;

    case KM_SIMTYPE:
      iErr = GetSimType(pibIn);
      break;

    case KM_TEMPERATURE:
      iErr = GetPerks(pibIn, szLex, &panal->gd);
      break;

    case KM_END:
      panal->wContext = CN_END;
      break;

    } /* switch */
  } /* else */

  if (iErr)
    EatStatement(pibIn);

} /* ProcessWord */


/* ----------------------------------------------------------------------------
   ReadAnalysis

   Core routine for input file parsing. Called from sim.c in particular.
*/
BOOL ReadAnalysis (PINPUTBUF pibIn)
{
  PSTRLEX szLex;    /* Lex elem of MAX_LEX length */
  PSTREQN szEqn;    /* Equation buffer of MAX_EQN length */
  int     iLexType;

  BOOL      bReturn = TRUE;
  PANALYSIS panal;

  if (!pibIn) return(FALSE);

  panal = (PANALYSIS) pibIn->pInfo;
  panal->iDepth = panal->iCurrentDepth = panal->iInstances = 0;
  panal->mc.plistMCVars = InitList();

  do {
    /* State machine for parsing syntax */
    NextLex(pibIn, szLex, &iLexType);

    switch (iLexType) {

      case LX_NULL:
        if (panal->wContext != CN_GLOBAL)
          ReportError(pibIn, RE_WARNING, NULL, "Unexpected end of file");

        if (panal->wContext == CN_EXPERIMENT)
          bReturn &= EndExperiment(pibIn, panal);
        panal->wContext = CN_END;
        break;

      case LX_IDENTIFIER:
        ProcessWord(pibIn, szLex, szEqn);
        break;

      case LX_PUNCT:
        if (szLex[0] == CH_STMTTERM)
          break;
        else if (szLex[0] == CH_RBRACE) {
          if (panal->wContext & CN_EXPERIMENT) {
            bReturn &= EndExperiment(pibIn, panal);
            break;
          }
          else {
            bReturn &= EndLevel(panal);
            break;
          }
        }
        else
          if (szLex[0] == CH_COMMENT) {
            SkipComment(pibIn);
            break;
          }

        /* else -- fall through! */

      default:
        ReportError(pibIn, RE_UNEXPECTED, szLex, "* Ignoring");
        break;

      case LX_INTEGER:
      case LX_FLOAT:
        ReportError(pibIn, RE_UNEXPNUMBER, szLex, "* Ignoring");
        break;

    } /* switch */

  } while (panal->wContext != CN_END
           && (*pibIn->pbufCur || FillBuffer(pibIn) != EOF));

  if(panal->iCurrentDepth != 0)
    ReportError(pibIn, RE_OPENLEVEL | RE_FATAL, "ReadAnalysis", NULL);
  return(bReturn);

} /* ReadAnalysis */

/* End */
