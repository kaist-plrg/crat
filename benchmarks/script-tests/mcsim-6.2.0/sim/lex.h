/* lex.h

   Written by Don Maszle
   13 October 1991

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

   Header file for Lexical parsing routines.
*/

#ifndef LEX_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions  */

#include "hungtype.h"

/* ----------------------------------------------------------------------------
   Constants  */

#define BUFFER_SIZE    0x10000    // Size of input data buffer
#define MAX_LEX        255        // Max size of Lexical Element
#define MAX_EQN        0x03FF     // Max size of a string eqn

/* ----------------------------------------------------------------------------
   Lexical types */

#define LX_NULL       0x0000
#define LX_IDENTIFIER 0x0001
#define LX_INTEGER    0x0002
#define LX_FLOAT      0x0004
#define LX_NUMBER     (LX_INTEGER | LX_FLOAT)
#define LX_PUNCT      0x0008
#define LX_STRING     0x0010

/* To avoid unmatched delimeters confusions in editor */

#define CH_LPAREN       ('(')
#define CH_RPAREN       (')')    
#define CH_LBRACKET     ('[')
#define CH_RBRACKET     (']')
#define CH_LBRACE       ('{')
#define CH_RBRACE       ('}')

/* Character constants for convenience */

#define CH_EOLN        ('\n')   // End of line character
#define CH_COMMENT     ('#')    // One line Comment Char
#define CH_STRDELIM    ('\"')   // String delimiter
#define CH_STMTTERM    (';')    // Statement terminator

/* Report Error constants -- Lex errors */

#define MAX_ERRORS 0

#define RE_FATAL            0x8000 // Can be ORd to iCode to cause exit(1)
#define RE_WARNING          0x4000 // can be ORd to issue Warning instead

#define RE_UNKNOWN          0x0000 /* Unspecified error */
#define RE_INIT             0x0001 // Error during initialization
#define RE_FILENOTFOUND     0x0002 // Error opening file for I/O
#define RE_CANNOTOPEN       0x0003 // Cannot open file
#define RE_OUTOFMEM         0x0004 // Error allocating memory
#define RE_READERROR        0x0005 // General read file error 

#define RE_UNEXPECTED       0x0011 // Unexpected char in input
#define RE_UNEXPNUMBER      0x0012 // Unexpected number in input
#define RE_EXPECTED         0x0013 // Expected character szMsg[0]
#define RE_LEXEXPECTED      0x0014 // Expected szMsg lexical element
#define RE_SYNTAXERR        0x0015 // Let's make syntax errors fatal

#define RE_BADCONTEXT       0x0101 // Invalid context for identifier
#define RE_EQNTOOLONG       0x0104 // Eq. too long for buffer
#define RE_UNDEFINED        0x0106 // Undefined identifier
#define RE_TOOMANYLEVELS    0x0110 // Too many dependency levels
#define RE_TOOMANYINST      0x0111 // Too many instances in level
#define RE_OPENLEVEL        0x0112 // Unclosed level or simulation
#define RE_LEVINEXPT        0x0113 // Level enclosed in simulation
#define RE_TYPENOTMCMC      0x0116 // Level statement outside MCMC
#define RE_TOOMANYPVARS     0x0117 // Too many variables in Print statement
#define RE_DUPVARINEXPRT    0x0121 // Same var appears twice or more
#define RE_POSITIVE         0x0122 // Positive number expected

#define RE_ERRORSINEXP      0x0201 // Errors reported, skipping exp
#define RE_NOOUTPUTS        0x0202 // No outputs specified
#define RE_SPECERR          0x0205 // Errors in specification
#define RE_INSUF_POINTS     0x0208 // Insufficient forced points
#define RE_MAXMIN_RANGE     0x0209 // Max < min
#define RE_OUTISRESTART     0x0210 // Output and restart files have same name 

/* Run-time Errors */

#define RE_BADNORMALSD      0x0301
#define RE_BADLOGNORMALSD   0x0302
#define RE_BADLOGNORMALMEAN 0x0303
#define RE_BADUNIFORMDIST   0x0304
#define RE_UNKNOWNDIST      0x0305
#define RE_BADMODEL         0x0307

/* ----------------------------------------------------------------------------
   Typedefs */

/* The INPUTBUF structure which is used for file I/O buffering */

typedef PSTR PBUF;


typedef struct tagINPUTBUF {
  PFILE pfileIn;    /* DOS file pointer */
  PBUF  pbufOrg;    /* Pointers for buffer Origin */
  PBUF  pbufCur;    /* ... Current point */
  int   iLineNum;   /* Line number in file */
  int   iLNPrev;    /* Prev line num.  For formatting Dynamics eqns */
  int   cErrors;    /* Count of Errors */

  PVOID    pInfo; /* Pointer to private user information */

} INPUTBUF, * PINPUTBUF;


typedef char PSTRLEX[MAX_LEX]; /* String of a lexical element */
typedef char PSTREQN[MAX_EQN]; /* String of an equation */


/* ----------------------------------------------------------------------------
   Macros */

#define EOB(pib) (!(pib)\
                  || ((!(pib)->pbufCur || !*(pib)->pbufCur)\
              && (!(pib)->pfileIn || feof((pib)->pfileIn))))

#define IsUnderscore(c)    ((c) == '_')
#define IsSign(c)    ((c) == '+' || (c) == '-')
#define IsString(szLex) ((szLex) ? (*(szLex) == CH_STRDELIM) : (0) )

#define ErrorsReported(pib) ((pib)->cErrors)
#define ClearErrors(pib) ((pib) ? (pib)->cErrors = 0 : 0)


/* ----------------------------------------------------------------------------
   Prototypes */

void EatStatement (PINPUTBUF pib);
int  EGetPunct (PINPUTBUF pibIn, PSTR szLex, char chPunct);
int  ENextLex (PINPUTBUF, PSTRLEX, int);

int  FillBuffer (PINPUTBUF pibIn);
void FlushBuffer (PINPUTBUF pibIn);

void GetArrayBounds (PINPUTBUF pibIn, PLONG piLB, PLONG piUB);
BOOL GetFuncArgs (PINPUTBUF, int, PINT, PSTR);
void GetIdentifier (PINPUTBUF pibIn, PSTR szLex);
void GetNumber (PINPUTBUF pibIn, PSTR szLex, PINT piLexType);
int  GetOptPunct (PINPUTBUF, PSTR, char);
int  GetPunct (PINPUTBUF pibIn, PSTR szLex, char chPunct);
void GetStatement (PINPUTBUF pibIn, PSTR szStmt);
void GetaString (PINPUTBUF pibIn, PSTR szLex);

BOOL InitBuffer (PINPUTBUF pibIn, PSTR szFullPathname);

void MakeStringBuffer (PINPUTBUF pBuf, PINPUTBUF pStrBuf, PSTR sz);

char NextChar (PINPUTBUF pibIn);
void NextLex    (PINPUTBUF, PSTRLEX, PINT);
int  NextListItem (PINPUTBUF, PSTR, int, int, char);

void PreventLexSplit (PINPUTBUF pibIn, int iOffset);

void SkipComment (PINPUTBUF);
int  SkipWhitespace (PINPUTBUF pibIn);

void UnrollEquation (PINPUTBUF pibIn, long index, PSTR szEqn, PSTR szEqnU);

#define LEX_H_DEFINED
#endif

/* End */


