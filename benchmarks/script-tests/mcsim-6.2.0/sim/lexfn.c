/* lexfn.c

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

   Handles lexical parsing for functions.

   This file contains the GetInputFn() routine and the definition of
   input functions which are shared between the model code generator
   facility and the simulation input routines.

*/

#include <ctype.h>
#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "lex.h"
#include "lexerr.h"
#include "lexfn.h"
#include "matutil.h"
#include "modelu.h"
#include "simi.h"
#include "strutil.h"
#include "modiface.h"

/* ----------------------------------------------------------------------------
   Macros */

#define IsIdentifier(sz)  ((sz) ? isalpha(*(sz)) || *(sz) == '_' : FALSE)

/* If this is compiled for the model generator, the symbol table
   doesn't exist yet.  We defined it here.  GetParmHandle is
   defined to calculate the parm handle *knowing how the symbol
   table is going to be written*.  Be careful if this is monkeyed with. */

/* Call model utility routine */
#define GetParmHandle(sz)  (GetVarHandle((sz)))

/* Keyword Map Structure */

typedef struct tagINPUTFUNCTIONMAP {
  PSTR szName;
  int  iIFNType;  /* Input function type */
} IFM, *PIFM; /* Input function map */


IFM vrgifmMap[] = { /* Input function map */

  {"PerDose",    IFN_PERDOSE},
  {"PerExp",     IFN_PEREXP},
  {"PerTransit", IFN_PERTRANS},
  {"NDoses",     IFN_NDOSES},
  {"Spikes",     IFN_SPIKES},
  {"Events",     IFN_EVENTS},

  {"", IFN_NULL} /* End flag */

};  /* vrgifnMap[] = */

/*  variables from lex.c, ZGY  */
extern PSTR vrgszLexTypes[];
extern VMMAPSTRCT vrgvmGlo[];

/* -----------------------------------------------------------------------------
   GetFnType

   Returns the code of the szKeyword given.  If the string is not
   a valid keyword or abbreviation, returns 0.
*/
int GetFnType (PSTR szName)
{
  PIFM pifm = &vrgifmMap[0];

  while (*pifm->szName && MyStrcmp (szName, pifm->szName))
    pifm++;

  return (pifm->iIFNType);  /* Return Keyword Code or 0 */

} /* GetFnType */


/* -----------------------------------------------------------------------------
   InitIFN

   Initialize an input function.
*/
void InitIFN (PIFN pifn)
{
  pifn->dTStartPeriod = 0.0;
  pifn->bOn = FALSE;

  pifn->dMag   = 0.0;  /* Initialize parameters */
  pifn->dTper  = 0.0;
  pifn->dT0    = 0.0;
  pifn->dTexp  = 0.0;
  pifn->dDecay = 0.0;
  pifn->dVal   = 0.0;
  pifn->nDoses = 0;

  pifn->hMag   = 0;    /* Initialize handles to parameters */
  pifn->hTper  = 0;
  pifn->hT0    = 0;
  pifn->hTexp  = 0;
  pifn->hDecay = 0;

  pifn->nDoses   = 0;  /* Initialize  nDoses param */
  pifn->iDoseCur = 0;

  /* The arrays and handles of nDoses will be initialized in DefDepParm */

} /* InitIFN */


/* -----------------------------------------------------------------------------
   DefDepParm

   Defines a parameter or a handle to a model parameter on which the
   parameter is dependent.  Returns TRUE on success.

   NOTE:  The call to GetParmHandle() is actually a macro which inquires
      into the symbol table for a handle to the dependent variable.
*/

BOOL DefDepParm (PSTR szLex, PDOUBLE pdValue, HANDLE *phvar)
{
  BOOL bReturn = TRUE;

  if (IsIdentifier(szLex)) { /* Define handle to model parameter */

    if (!(*phvar = (HANDLE) GetParmHandle(szLex))) {
      bReturn = FALSE;
      ReportError (NULL, RE_UNDEFINED, szLex, NULL);
    }
  }
  else {
    *pdValue = atof(szLex); /* Define actual parm from number */
    *phvar = 0;
  }

  return (bReturn);

} /* DefDepParm */


/* -----------------------------------------------------------------------------
   GetInputsArgs

   Reads the arguments to the input function pifn.

   * If the argument is a number, the parameter is defined.

   * If the argument is an identifier which is a valid model
     parameter, the input function parameter is defined as
     being dependent on this model parameter.  The input parameter
     will be defined in the first UpdateInputs() call after model
     initialization.
*/

BOOL GetInputArgs (PINPUTBUF pibIn, PIFN pifn, int nArgs)
{
  PSTRLEX *rgszLex = malloc (nArgs * sizeof(PSTRLEX));
  int *rgiTypes = malloc (nArgs * sizeof(int));
  int  i;
  BOOL bReturn = FALSE;

  for (i = 0; i < nArgs; i++)
    rgiTypes[i] = LX_INTEGER | LX_FLOAT | LX_IDENTIFIER;

  if (GetFuncArgs (pibIn, nArgs, rgiTypes, rgszLex[0])) {

    /* Try to get each parm to show all errors */

    bReturn = TRUE;
    bReturn &= DefDepParm (rgszLex[0], &pifn->dMag, &pifn->hMag);
    bReturn &= DefDepParm (rgszLex[1], &pifn->dTper, &pifn->hTper);
    bReturn &= DefDepParm (rgszLex[2], &pifn->dT0, &pifn->hT0);

    if ((pifn->iType == IFN_PEREXP) || (pifn->iType == IFN_PERTRANS))
      bReturn &= DefDepParm (rgszLex[3], &pifn->dDecay, &pifn->hDecay);
    else
      bReturn &= DefDepParm (rgszLex[3], &pifn->dTexp, &pifn->hTexp);

    if ((pifn->iType == IFN_PERTRANS) && (nArgs == 5))
      bReturn &= DefDepParm (rgszLex[4], &pifn->dNcpt, &pifn->hNcpt);

    if (!bReturn)
      ReportError (pibIn, RE_EXPECTED, "input-spec", NULL);
  } /* if */

  free(rgiTypes);
  free(rgszLex);

  return (bReturn);

} /* GetInputArgs */


/* -----------------------------------------------------------------------------
   GetNNumbers

   Tries to read nNumbers from pibIn.  Returns TRUE on error.
*/

BOOL GetNNumbers (PINPUTBUF pibIn, PSTR szLex, int nNumbers, PDOUBLE rgd)
{
  BOOL bErr = FALSE;
  int i;

  for (i = 0; i < nNumbers  && !bErr; i++) {
    if (i)
      GetOptPunct (pibIn, szLex, ',');
    if (!(bErr = ENextLex (pibIn, szLex, LX_NUMBER)))
      rgd[i] = atof(szLex);
  } /* for */

  return bErr;

} /* GetNNumbers */


/* -----------------------------------------------------------------------------
   GetNDoses

   Reads the arguments for the NDoses() input type.
   Return TRUE if the structure is defined, FALSE on error.
   The command syntax includes the number of doses then
   the list of the dose magnitudes and then the list of
   starting times. The final time is set to DBL_MAX (i.e.
   the last dose is forever.
*/
BOOL GetNDoses (PINPUTBUF pibIn, PSTR szLex, PIFN pifn)
{
  PSTRLEX *rgszLex;
  PSTRLEX  szTmp;
  int *rgiTypes, iType;
  long i, j, iLB, iUB, iDoseArg;
  BOOL bOK = TRUE;
  BOOL bErr = FALSE; /* Return value flags error condition */
  HVAR hvar;

  if ((bErr = EGetPunct(pibIn, szLex, CH_LPAREN)))
    goto Exit_GetNDoses;

  if ((bErr = ENextLex(pibIn, szLex, LX_INTEGER)))
    goto Exit_GetNDoses;

  pifn->nDoses = atoi(szLex);

  if ((bErr = (pifn->nDoses <= 0))) {
    ReportError (pibIn, RE_LEXEXPECTED | RE_FATAL, "positive-integer", szLex);
    goto Exit_GetNDoses;
  }

  /* iDoseArg is the number of arguments (doses and starting times)
     in NDoses after the first integer, number of doses */
  iDoseArg = 2 * pifn->nDoses;

  if ( !(rgiTypes = InitiVector (iDoseArg)))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetNDoses", NULL);

  if ( !(rgszLex = (PSTRLEX *) malloc (iDoseArg * sizeof(PSTRLEX))))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetNDoses", NULL);

  if ( !(pifn->rgT0s = InitdVector (pifn->nDoses + 1)) ||
       !(pifn->rgMags = InitdVector (pifn->nDoses)))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetNDoses", NULL);

  if (!(pifn->rghT0s =
       (HANDLE *) malloc ((pifn->nDoses+1) * sizeof(HANDLE))) || 
      !(pifn->rghMags =
       (HANDLE *) malloc (pifn->nDoses * sizeof(HANDLE))))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetNDoses", NULL);

  /* Read the leading comma */
  if (!(bOK = GetPunct (pibIn, rgszLex[0], ','))) {
    *(rgszLex[0] + 1)  = ',';
    ReportError(pibIn, RE_EXPECTED | RE_FATAL, rgszLex[0], NULL);
  }

  /* Try to get a list of n magnitudes */
  for (i = 0; i < (iDoseArg / 2) && bOK; i++) {

    rgiTypes[i] = LX_INTEGER | LX_FLOAT | LX_IDENTIFIER;

    /* read the magnitude, numeric or symbolic */
    NextLex (pibIn, szLex, &iType);
    if (!(bOK &= (iType & rgiTypes[i]) > 0))
      ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, vrgszLexTypes[rgiTypes[i]],
                  szLex);

    /* check if it is a scalar or an array and act accordingly */
    iLB = iUB = -1;
    if (GetPunct (pibIn, szTmp, '[')) /* array found, read bounds */
      GetArrayBounds (pibIn, &iLB, &iUB);

    if (iUB == -1) { /* scalar, copy to rgszLex and continue */
      strcpy (rgszLex[i], szLex);
    }
    else { /* array */

      if (2 * (iUB - iLB) != iDoseArg)
        ReportError (pibIn, RE_TOOMANYPVARS | RE_FATAL, "GetNDoses", NULL);

      for (j = iLB; j < iUB; j++) {
        sprintf (szTmp, "%s_%ld", szLex, j); /* create names */          

        if ((bErr = !(hvar = GetVarHandle (szTmp))))
          ReportError (pibIn, RE_UNDEFINED | RE_FATAL, szTmp, NULL);
        else
          strcpy (rgszLex[i+j-iLB], szTmp);
      }

      /* get the separating comma */
      if ((bErr = !GetPunct (pibIn, szTmp, ',')))
        goto Exit_GetNDoses;

      break; /* get out of the for i loop */

    } /* end else */

  } /* for i */

  /* Try to get a list of n T0's, numeric or symbolic */
  for (i = iDoseArg / 2; i < iDoseArg && bOK; i++) {

    rgiTypes[i] = LX_INTEGER | LX_FLOAT | LX_IDENTIFIER;

    /* read the T0's, numeric or symbolic */
    NextLex (pibIn, szLex, &iType);
    if (!(bOK &= (iType & rgiTypes[i]) > 0))
      ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, vrgszLexTypes[rgiTypes[i]],
                  szLex);

    /* check if it is a scalar or an array and act accordingly */
    iLB = iUB = -1;
    if (GetPunct (pibIn, szTmp, '[')) /* array found, read bounds */
      GetArrayBounds (pibIn, &iLB, &iUB);

    if (iUB == -1) { /* scalar, copy to rgszLex and continue */
      strcpy (rgszLex[i], szLex);
    }
    else { /* array */

      if (2 * (iUB - iLB) != iDoseArg)
        ReportError (pibIn, RE_TOOMANYPVARS | RE_FATAL, "GetNDoses", NULL);

      for (j = iLB; j < iUB; j++) {
        sprintf (szTmp, "%s_%ld", szLex, j); /* create names */

        if ((bErr = !(hvar = GetVarHandle (szTmp))))
          ReportError (pibIn, RE_UNDEFINED | RE_FATAL, szTmp, NULL);
        else
          strcpy (rgszLex[i+j-iLB], szTmp);
      } /* for j */

      /* get the final parenthesis */
      if ((bErr = EGetPunct (pibIn, szTmp, CH_RPAREN)))
        goto Exit_GetNDoses;

      break; /* get out of the for i loop */

    } /* end else */

  } /* for i */

  if ((bErr = (szTmp[0] != CH_RPAREN)))
    goto Exit_GetNDoses;

  /* We have read in the list and stored it in rgszLex.
     Try to get each parm value or handle to show all errors */

  bOK = TRUE;

  /* magnitudes */
  for (i = 0; i < pifn->nDoses; i++)
    bOK &= DefDepParm (rgszLex[i], pifn->rgMags + i, pifn->rghMags + i);

  /* starting times */
  for (i = 0; i < pifn->nDoses; i++)
    bOK &= DefDepParm (rgszLex[i+pifn->nDoses], pifn->rgT0s+i, pifn->rghT0s+i);

  /* final time */
  i = pifn->nDoses;
  pifn->rgT0s[i] = DBL_MAX;
  pifn->rghT0s[i] = 0;

  if (!bOK) ReportError (pibIn, RE_EXPECTED | RE_FATAL, "input-spec", NULL);

Exit_GetNDoses:

  if (bErr)
    printf ("Syntax: NDoses (nInputs, <n Magnitudes>, <n T0's>)\n\n");

  return (!bErr);

} /* GetNDoses */


/* -----------------------------------------------------------------------------
   GetEvents

   Reads the arguments for the Events() "input" type (in fact will affect a 
   state variable. Return TRUE if the structure is defined, FALSE on error.
*/
BOOL GetEvents (PINPUTBUF pibIn, PSTR szLex, PIFN pifn)
{
  PSTRLEX *rgszLex;
  int     iType, *rgiTypes;
  int     i, iDoseArg;
  BOOL    bOK = TRUE;
  BOOL    bErr = FALSE;    /* Return value flags error condition */

  if ((bErr = EGetPunct (pibIn, szLex, CH_LPAREN)))
    goto Exit_GetEvents;

  // Get target state ID
  NextLex (pibIn, szLex, &iType);
  if (!(bOK &= (iType & (LX_IDENTIFIER)) > 0))
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, vrgszLexTypes[LX_IDENTIFIER],
                szLex);

  if (!(pifn->target_state = (HANDLE) GetParmHandle(szLex))) {
    ReportError (NULL, RE_UNDEFINED | RE_FATAL, szLex, NULL);
  }

  // Get punctuation
  if (!(bOK = GetPunct (pibIn, szLex, ','))) {
    ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, ",", szLex);
  }

  // Get positive integer number of spikes

  if ((bErr = ENextLex (pibIn, szLex, LX_INTEGER)))
    goto Exit_GetEvents;

  pifn->nDoses = atoi(szLex);

  if ((bErr = (pifn->nDoses <= 0))) {
    ReportError (pibIn, RE_LEXEXPECTED | RE_FATAL, "positive-integer", szLex);
    goto Exit_GetEvents;
  }

  /* Try to get list: n times, n operations, n values */
  /* iDoseArg is the total number of following arguments (times, operation, 
     value) in Events after the number of discontinuities */
  iDoseArg = 3 * pifn->nDoses;

  if ( !(rgiTypes = InitiVector (iDoseArg)))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetEvents", NULL);

  if ( !(rgszLex = (PSTRLEX *) malloc (iDoseArg * sizeof(PSTRLEX))))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetEvents", NULL);

  for (i = 0; i < iDoseArg && bOK; i++) {

    rgiTypes[i] = LX_INTEGER | LX_FLOAT | LX_IDENTIFIER;

    if (!(bOK = GetOptPunct (pibIn, rgszLex[i], ','))) {
      *(rgszLex[i] + 1)  = ',';
      ReportError(pibIn, RE_EXPECTED | RE_FATAL, rgszLex[i], NULL);
    }

    NextLex (pibIn, rgszLex[i], &iType);
    if (!(bOK &= (iType & rgiTypes[i]) > 0))
      ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, vrgszLexTypes[rgiTypes[i]],
                  rgszLex[i]);
  } /* for */

  if ((bErr = EGetPunct (pibIn, szLex, CH_RPAREN)))
    goto Exit_GetEvents;

  /* Try to get each parm to show all errors */

  if ( !(pifn->rgT0s  = InitdVector (pifn->nDoses)) ||
       !(pifn->rgOper = InitiVector (pifn->nDoses)) ||
       !(pifn->rgMags = InitdVector (pifn->nDoses)))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetEvents", NULL);

  if (!(pifn->rghT0s  =
       (HANDLE *) malloc ((pifn->nDoses) * sizeof(HANDLE))) || 
      !(pifn->rghMags =
       (HANDLE *) malloc (pifn->nDoses * sizeof(HANDLE))))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetEvents", NULL);

  bOK = TRUE;

  /* times */
  for (i = 0; i < pifn->nDoses; i++)
    bOK &= DefDepParm (rgszLex[i], pifn->rgT0s+i, pifn->rghT0s+i);

  /* operations */
  for (i = 0; i < pifn->nDoses; i++) {
    pifn->rgOper[i] = GetKeywordCode (rgszLex[pifn->nDoses + i], NULL);
    if (!(pifn->rgOper[i])) 
      ReportError (pibIn, RE_LEXEXPECTED | RE_FATAL, 
                   "Replace, Add or Multiply operation",
                   rgszLex[pifn->nDoses + i]);
  }

  /* magnitudes */
  for (i = 0; i < pifn->nDoses; i++)
    bOK &= DefDepParm (rgszLex[2 * pifn->nDoses + i], 
                       pifn->rgMags+i, pifn->rghMags+i);

  if (!bOK) ReportError (pibIn, RE_EXPECTED | RE_FATAL, "input-spec", NULL);

Exit_GetEvents:

  if (bErr)
    printf ("Syntax: Events (State, nEvents, <n Times>, <n Operations>, "
            "<n Values>)\n\n");

  return (!bErr);

} /* GetEvents */


/* -----------------------------------------------------------------------------
   GetSpikes

   Reads the arguments for the Spikes() input type.  Return TRUE if
   the structure is defined, FALSE on error.
*/
BOOL GetSpikes (PINPUTBUF pibIn, PSTR szLex, PIFN pifn)
{
  PSTRLEX *rgszLex;
  int *rgiTypes, iType;
  int i, iDoseArg;
  BOOL bOK = TRUE;
  BOOL bErr = FALSE;    /* Return value flags error condition */

  if ((bErr = EGetPunct (pibIn, szLex, CH_LPAREN)))
    goto Exit_GetSpikes;

  /* Get positive integer number of spikes */

  if ((bErr = ENextLex (pibIn, szLex, LX_INTEGER)))
    goto Exit_GetSpikes;

  pifn->nDoses = atoi(szLex);

  if ((bErr = (pifn->nDoses <= 0))) {
    ReportError (pibIn, RE_LEXEXPECTED | RE_FATAL, "positive-integer", szLex);
    goto Exit_GetSpikes;
  }

  /* iDoseArg is the number of arguments (doses and starting times)
     in Spikes after the first integer, number of doses */
  iDoseArg = 2 * pifn->nDoses;

  if ( !(rgiTypes = InitiVector (iDoseArg)))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetSpikes", NULL);

  if ( !(rgszLex = (PSTRLEX *) malloc (iDoseArg * sizeof(PSTRLEX))))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetSpikes", NULL);

  if ( !(pifn->rgT0s = InitdVector (pifn->nDoses)) ||
       !(pifn->rgMags = InitdVector (pifn->nDoses)))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetSpikes", NULL);

  if (!(pifn->rghT0s =
       (HANDLE *) malloc ((pifn->nDoses) * sizeof(HANDLE))) || 
      !(pifn->rghMags =
       (HANDLE *) malloc (pifn->nDoses * sizeof(HANDLE))))
    ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "GetSpikes", NULL);

  /* Try to get list: n Mag's, n T0's */

  for (i = 0; i < iDoseArg && bOK; i++) {

    rgiTypes[i] = LX_INTEGER | LX_FLOAT | LX_IDENTIFIER;

    if (!(bOK = GetOptPunct (pibIn, rgszLex[i], ','))) {
      *(rgszLex[i] + 1)  = ',';
      ReportError(pibIn, RE_EXPECTED | RE_FATAL, rgszLex[i], NULL);
      break; /* Error: Stop getting args */
    }

    NextLex (pibIn, rgszLex[i], &iType);
    if (!(bOK &= (iType & rgiTypes[i]) > 0))
      ReportError(pibIn, RE_LEXEXPECTED | RE_FATAL, vrgszLexTypes[rgiTypes[i]],
                  rgszLex[i]);
  } /* for */

  if ((bErr = EGetPunct (pibIn, szLex, CH_RPAREN)))
    goto Exit_GetSpikes;

  /* Try to get each parm to show all errors */

  bOK = TRUE;

  /* magnitudes */
  for (i = 0; i < pifn->nDoses; i++)
    bOK &= DefDepParm (rgszLex[i], pifn->rgMags + i, pifn->rghMags + i);

  /* times */
  for (i = 0; i < pifn->nDoses; i++)
    bOK &= DefDepParm (rgszLex[i+pifn->nDoses], pifn->rgT0s+i, pifn->rghT0s+i);

  if (!bOK) ReportError (pibIn, RE_EXPECTED | RE_FATAL, "input-spec", NULL);

Exit_GetSpikes:

  if (bErr)
    printf ("Syntax: Spikes (nInputs, <n Magnitudes>, <n Times>)\n\n");

  return (!bErr);

} /* GetSpikes */


/* -----------------------------------------------------------------------------
   GetInputFn

   Attempts to define an IFN structure pifn according to the
   input spec in sz if sz is non-NULL, or pibIn if sz is NULL.
   Returns TRUE if the structure is defined.
*/
BOOL GetInputFn (PINPUTBUF pibIn, PSTR sz, PIFN pifn)
{
  INPUTBUF ibDummy;
  PINPUTBUF pibDum = &ibDummy;
  PSTRLEX szLex;
  int iType;
  BOOL bReturn = FALSE;

  if (!pibIn || !pifn)
    return (FALSE);

  if (sz)
    MakeStringBuffer (pibIn, pibDum, sz);
  else
    pibDum = pibIn;

  NextLex (pibDum, szLex, &iType);
  switch (iType) {

  default:
  case LX_NULL:
  case LX_PUNCT:
    ReportError (pibIn, RE_LEXEXPECTED, "input-spec", NULL);
    break;

  case LX_FLOAT:
  case LX_INTEGER:
  case LX_IDENTIFIER:
    InitIFN (pifn);

    if (iType == LX_IDENTIFIER) {
      pifn->iType = GetFnType (szLex);
      switch (pifn->iType) {

      case IFN_NDOSES:
        bReturn = GetNDoses (pibDum, szLex, pifn);
        break;

      case IFN_SPIKES:
        bReturn = GetSpikes (pibDum, szLex, pifn);
        break;

      case IFN_EVENTS:
        bReturn = GetEvents (pibDum, szLex, pifn);
        break;

      default:
        pifn->iType = IFN_NULL;
        ReportError (pibIn, RE_LEXEXPECTED, "input-spec", szLex);
        break;

      case IFN_PERDOSE:
      case IFN_PEREXP:
        bReturn = GetInputArgs (pibDum, pifn, 4); /* 4 arguments */
        break;

      case IFN_PERTRANS:
        bReturn = GetInputArgs (pibDum, pifn, 5); /* 5 arguments */
        break;

      } /* switch */
    } /* if identifier */

    else {
      pifn->iType = IFN_CONSTANT;
      pifn->dMag = pifn->dVal = atof (szLex);
      pifn->bOn = TRUE;
      bReturn = TRUE;
    } /* else */
    break;

  } /* switch */

  return (bReturn);
} /* GetInputFn */
