/* simi.h

   Originally written by Frederic Bois

   Copyright (c) 1993-2008 Free Software Foundation, Inc.

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

   Header file for simi.c
*/

#ifndef SIMI_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions
*/

#include "sim.h"
#include "strutil.h"


#define INIT_KERNELSD 1.0  /* Initial value of MH std dev divisor */

/* ----------------------------------------------------------------------------
   Prototypes
*/

BOOL CheckDistribParam(PLIST plist, HVAR hvar1, HVAR hvar2);
void DListToArray (PLIST plist, PLONG pcDouble, PDOUBLE *ppDouble);
BOOL EndExperiment (PINPUTBUF pibIn, PANALYSIS panal);
BOOL EndLevel (PANALYSIS panal);
void FreeLevels (PANALYSIS panal);
int  FreeMCVar (PVOID pData, PVOID pUserInfo);
int  FreeDataRec (PVOID pData, PVOID pUserInfo);
int  FreePrintRec (PVOID pData, PVOID pUserInfo);
void FreeOneLevel (PLEVEL plevel);
BOOL GetData (PINPUTBUF pibIn, PSTR szLex, POUTSPEC pos);
BOOL GetMCMCSpec (PINPUTBUF pibIn, PEXPERIMENT pexp);
BOOL GetIntegrate (PINPUTBUF pibIn, PSTR szLex, PINTSPEC pis);
PSTR GetKeyword (int iKWCode);
int  GetKeywordCode (PSTR szKeyword, PINT pfContext);
BOOL GetListOfData (PINPUTBUF pibIn, PDATAREC pda, PSTR szLex);
BOOL GetListOfTimes (PINPUTBUF pibIn, int nRecs, PPRINTREC *ppr, PSTR szLex);
int  GetDistribParam(PINPUTBUF pibIn, PSTR szLex,
                     PLIST plist, int n, PMCVAR pmcvar);
int  GetDistribSpec (PINPUTBUF pibIn, PSTR szLex, PANALYSIS panal);
int  GetMonteCarloSpec (PINPUTBUF pibIn, PANALYSIS panal, PSTR szLex);
BOOL GetOptDSpec (PINPUTBUF pibIn, PANALYSIS  panal, PSTR szLex);
BOOL GetOutputFile (PINPUTBUF pibIn, PSTR szLex, PANALYSIS panal);
BOOL GetParmMod (PINPUTBUF pibIn, PSTRLEX szLex);
BOOL GetPrint (PINPUTBUF pibIn, PSTR szLex, POUTSPEC pos);
BOOL GetPrintStep (PINPUTBUF pibIn, PSTR szLex, POUTSPEC pos);
int  GetSetPointsSpec (PINPUTBUF pibIn, PANALYSIS  panal, PSTR szLex);
BOOL GetSimType (PINPUTBUF pibIn);
BOOL GetSimulate ();
BOOL GetStartTime (PINPUTBUF pibIn, PEXPERIMENT pexp);
BOOL GetStringArg (PINPUTBUF pibIn, PSTR *pszArg, PSTR szLex, BOOL bDelim);
BOOL GetInvTemperature (PINPUTBUF pibIn, PSTR szLex, PGIBBSDATA pgd);
int  GetTerminator (PINPUTBUF pibIn, PSTR szLex);
long ImFromLex (PSTR szLex);
int  McvFromLex (PSTR szLex);
void NewExperiment (PINPUTBUF pibIn);
int  OneDToArray (PVOID pData, PVOID pInfo);
void ProcessWord (PINPUTBUF pibIn, PSTR szLex, PSTR szEqn);
BOOL ReadAnalysis (PINPUTBUF);
int  SetLevel (PINPUTBUF pibIn);
BOOL YesNoFromLex (PSTR szLex);

#define SIMI_H_DEFINED
#endif  /* SIMI_H_DEFINED */

/* End */

