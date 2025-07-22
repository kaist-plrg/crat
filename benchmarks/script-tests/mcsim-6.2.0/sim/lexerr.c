/* lexerr.c

   Written by Don Maszle
   15 September 1991

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

   Reports errors and exits program if fatal.
*/

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdarg.h>

#include "lexerr.h"
#include "simi.h"


/* -----------------------------------------------------------------------------
   ReportError

   Reports error iCode to terminal (one of RE_) and optional
   szMessage. If iSeverity is set to RE_FATAL, exits program.
*/

void ReportError (PINPUTBUF pibIn, WORD wCode, PSTR szMsg, PSTR szAltMsg)
{
  char cNull = '\0';
  BOOL bFatal   = wCode & RE_FATAL;
  BOOL bWarning = wCode & RE_WARNING;

  wCode &= ~(RE_FATAL | RE_WARNING);

  if (!szMsg)
    szMsg = &cNull;

  if (wCode) {
    if (bWarning)
      printf ("Warning: ");
    else {
      printf ("Error: ");
      bFatal |= (pibIn && (pibIn->cErrors++ > MAX_ERRORS));
    }
  } /* if */

  if (pibIn) {
    if (pibIn->pfileIn || pibIn->iLNPrev) { /* Line number is valid */
      printf ("line %d: ", pibIn->iLineNum);
    }
    else {
      if (wCode != RE_FILENOTFOUND) { /* Dummy pibIn, show buffer */
        PSTRLEX szTmp;
        szTmp[MAX_LEX-1] = '\0';
        printf ("'%s'...\n  ", strncpy (szTmp, pibIn->pbufOrg, MAX_LEX-1));
      } /* if */
    }
  }

  switch (wCode) {

  case 0:
    break;

  default:
    printf ("Unknown error code %x: %s", wCode, szMsg);

  case RE_INIT:
    printf ("Initialization error.");
    break;

  case RE_FILENOTFOUND:
    printf ("File not found \"%s\".", szMsg);
    break;

  case RE_CANNOTOPEN:
    printf ("Cannot open file \"%s\".", szMsg);
    break;

  case RE_OUTOFMEM:
    printf ("Out of memory in %s().", szMsg);
    break;

  case RE_READERROR:
    printf ("%s file cannot be read\n", szMsg);
    break;

  case RE_UNEXPECTED:
    printf ("Unexpected character '%c' in input file.", *szMsg);
    break;

  case RE_UNEXPNUMBER:
    printf ("Unexpected number %s in input file.", szMsg);
    break;

  case RE_EXPECTED:
    printf ("Expected '%c' before '%c'.", szMsg[1], szMsg[0]);
    break;

  case RE_LEXEXPECTED:
    printf ("Expected <%s>", szMsg);
    if (szAltMsg)
      printf (" before '%s'", szAltMsg);
    break;

  case RE_SYNTAXERR:
    printf("Syntax error %s", szMsg);
    break;

  case RE_BADCONTEXT:
    printf ("'%s' used in invalid context.", szMsg);
    break;

  case RE_TOOMANYLEVELS:
    printf("Too many levels");
    break;

  case RE_TOOMANYINST:
    printf("Too many instances at level %s", szMsg);
    break;

  case RE_OPENLEVEL:
    printf("Unclosed level statement");
    break;

  case RE_LEVINEXPT:
    printf("Level statement enclosed in Simulation (Experiment) statement");
    break;

  case RE_TOOMANYPVARS:
    printf("Too many variables in 'Print(...)' statement");
    break;

  case RE_EQNTOOLONG:
    printf ("Equation is too long.  Possibly missing terminator.");
    break;

  case RE_UNDEFINED:
    printf ("Undefined identifier '%s'.", szMsg);
    break;

  case RE_TYPENOTMCMC:
    printf ("The level statement is permitted only in MCMC simulations.\n");
    break;

  /* Simulation input errors */

  case RE_ERRORSINEXP:
    printf ("Bad definition of experiment %d\n", *(PINT)szMsg);
    break;

  case RE_NOOUTPUTS:
    printf ("Simulation (Experiment) %d has no outputs specified\n", 
            *(PINT)szMsg);
    break;

  case RE_POSITIVE:
    printf ("Positive number expected.");
    break;

  case RE_SPECERR:
    printf ("in specification: %s", szMsg);
    break;

  case RE_INSUF_POINTS:
    printf ("Insufficient points in file \"%s\"\n", szMsg);
    break;

  case RE_MAXMIN_RANGE:
    printf ("Max is less than min\n");
    break;

  case RE_OUTISRESTART:
    printf ("Output and restart files have the same name\n");
    break;

  } /* switch */

  printf ("\n");
  if (szAltMsg && wCode != RE_LEXEXPECTED)
    printf ("%s\n", szAltMsg);

  if (bFatal) {
    if ((pibIn != NULL) && (pibIn->pInfo != NULL))
      FreeLevels((PANALYSIS)pibIn->pInfo);

    printf ("\nFatal errors.  Exiting.\n\n");

    exit (wCode);

  } /* if bFatal */

} /* ReportError */


/* -----------------------------------------------------------------------------
   ReportRunTimeError

   Reports error iCode to terminal (one of RE_) and optional
   szMessage. If iSeverity is set to RE_FATAL, exits program.
*/

void ReportRunTimeError (PANALYSIS panal, WORD wCode, ...)
{
  va_list ap;
  PSTR szMsg1, szMsg2, szMsg3;
  BOOL bFatal   = wCode & RE_FATAL;
  BOOL bWarning = wCode & RE_WARNING;

  wCode &= ~(RE_FATAL | RE_WARNING);

  if(wCode)
    bWarning ? (printf("Warning: ")) : (printf ("Fatal error: "));

  va_start(ap, wCode);

  switch (wCode) {

  case 0:
    break;

  default:
    printf("Unknown error code %x", wCode);
    break;

  case RE_OUTOFMEM:
    szMsg1 = va_arg(ap, PSTR);
    printf ("Out of memory in %s().", szMsg1); 
    break;

  case RE_CANNOTOPEN:
    szMsg1 = va_arg(ap, PSTR);
    szMsg2 = va_arg(ap, PSTR);
    printf ("Cannot open file \"%s\" in %s().", szMsg1, szMsg2);
    break;

  case RE_BADNORMALSD:
    szMsg1 = va_arg(ap, PSTR);
    szMsg2 = va_arg(ap, PSTR);
    szMsg3 = va_arg(ap, PSTR);
    printf ("SD of normal variate %s = %s in %s().",
            szMsg1, szMsg2, szMsg3);
    break;

  case RE_BADLOGNORMALSD:
    szMsg1 = va_arg(ap, PSTR);
    szMsg2 = va_arg(ap, PSTR);
    szMsg3 = va_arg(ap, PSTR);
    printf ("SD of lognormal variate %s = %s in %s().",
            szMsg1, szMsg2, szMsg3);
    break;

  case RE_BADLOGNORMALMEAN:
    szMsg1 = va_arg(ap, PSTR);
    szMsg2 = va_arg(ap, PSTR);
    szMsg3 = va_arg(ap, PSTR);
    printf ("Mean of lognormal variate %s = %s in %s().",
            szMsg1, szMsg2, szMsg3);
    break;

  case RE_BADUNIFORMDIST:
    szMsg1 = va_arg(ap, PSTR);
    szMsg2 = va_arg(ap, PSTR);
    printf ("Max and min of uniform variate %s are equal or inverted in %s().",
            szMsg1, szMsg2);
    break;

  case RE_UNKNOWNDIST:
    szMsg1 = va_arg(ap, PSTR);
    printf ("Unknown distribution in %s().", szMsg1);
    break;

  case RE_BADMODEL:
    printf ("Bad value in output; model is not computable.\n");
    break;

  case RE_DUPVARINEXPRT:
    szMsg1 = va_arg(ap, PSTR);
    szMsg2 = va_arg(ap, PSTR);
    printf ("Variable %s appears in two or more '%s' statements.\n", 
            szMsg1, szMsg2);
    break;

  } /* switch */

  printf ("\n");

  va_end(ap);

  if (bFatal) {
    if (panal != NULL)
      FreeLevels(panal);

    printf ("\nFatal errors.  Exiting.\n\n");

    exit (wCode);
  }

} /* ReportRunTimeError */

/* End. */

