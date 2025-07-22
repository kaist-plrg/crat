/* lex.c

   Written by Don Maszle
   10 October 1991

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

   Contains routines for lexical parsing of input.  Provides types
   INPUTBUF and *PINPUTBUF for maintaining information about an input
   buffer.  The buffer must be initialized with the name of a file.

   All Lex routines take the input buffer as an argument.  Input is
   read from the buffer and the buffer position is updated.  A
   routine MakeStringBuffer() creates a temporary buffer from a
   string, copying information about the original buffer if it is
   provided.

   A pointer is provided in the INPUTBUF structure to hold user information.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "lex.h"
#include "lexerr.h"

#ifndef SEEK_CUR
#define SEEK_CUR 1
#endif

PSTR vrgszLexTypes[] = {
  "null-type??",             /* 00 */
  "identifier",              /* 01 */
  "integer",                 /* 02 */
  "integer-or-id",           /* 03 */
  "floating-point",          /* 04 */
  "float-or-id",             /* 05 */
  "number",                  /* 06 */
  "number-or-id",            /* 07 */
  "punctuation",             /* 08 */
  "invalid lexical type",    /* 09 */
  "invalid lexical type",    /* 0a */
  "invalid lexical type",    /* 0b */
  "invalid lexical type",    /* 0c */
  "invalid lexical type",    /* 0d */
  "invalid lexical type",    /* 0e */
  "invalid lexical type",    /* 0f */
  "quoted-string",           /* 10 */

  "" /* End flag */
}; /* vrgszLexTypes[] = */


/* routines */

/* -----------------------------------------------------------------------------
   PreventLexSplit

   Prevents the buffer from splitting a lexical element by
   "backing up" the EOB pointer and adjusting the file pointer
   to just after a carriage return.
*/
void PreventLexSplit (PINPUTBUF pibIn, int iOffset)
{
  long lDelta;
  PBUF pbufEOB = pibIn->pbufOrg + iOffset;
  PBUF pbufEOBOld;

  if (!EOB(pibIn)        /* If not EOB, use all of input, otherwise... */
      || (iOffset == BUFFER_SIZE)) {    /* No room for NULL */

    pbufEOBOld = pbufEOB;        /* Save EOB */
    while (*(--pbufEOB) != CH_EOLN)
      ;                /* Move EOB to last EOLN */
    *pbufEOB = '\0';        /* Overwrite EOLN with NULL */

    if ((lDelta = (long) (pbufEOB - pbufEOBOld)))
      fseek (pibIn->pfileIn, lDelta, SEEK_CUR);    /* Backup file ptr */
  } /* if */

  else *pbufEOB = '\0'; /* Append NULL */

} /* PreventLexSplit */


/* -----------------------------------------------------------------------------
   FillBuffer

   Fills the initialized input buffer and sets all buffer pointers.

   Return 0 on error, non-zero on success or EOF if at end of file.
*/
int FillBuffer (PINPUTBUF pibIn)
{
  int iReturn = 0;
  int iOffset;

  if (pibIn && pibIn->pfileIn && pibIn->pbufOrg) {

    if ((iOffset = fread (pibIn->pbufOrg, 1, BUFFER_SIZE, pibIn->pfileIn))) {
      iReturn = (int) iOffset;
      PreventLexSplit (pibIn, iOffset);
      pibIn->pbufCur = pibIn->pbufOrg;
    } /* if */

    else
      if (feof(pibIn->pfileIn))
    iReturn = EOF;
      else
    ReportError (pibIn, RE_FATAL, "Unexpected end of file.", NULL);
  } /* if */

  return (iReturn);

} /* FillBuffer */


/* -----------------------------------------------------------------------------
   InitBuffer

   Initializes the input buffer whose address is pibIn with the
   file given by szFullPathname, and fills the buffer array with the
   first data to be processed.

   Returns 0 on error, non-zero on success.
*/
BOOL InitBuffer (PINPUTBUF pibIn, PSTR szFullPathname)
{
  BOOL bReturn = 0;

  if (!pibIn)
    return FALSE;

  pibIn->iLineNum = 1;
  pibIn->iLNPrev = 0;
  pibIn->cErrors = 0;
  pibIn->pInfo = NULL;
  pibIn->pbufCur = NULL;

  if ((pibIn->pfileIn = fopen (szFullPathname, "r"))) {
    if ((pibIn->pbufOrg = (PBUF) malloc (BUFFER_SIZE)))
      bReturn = FillBuffer (pibIn);
    else
      ReportError (pibIn, RE_OUTOFMEM | RE_FATAL, "InitBuffer", NULL);
  }
  else
    ReportError (pibIn, RE_FILENOTFOUND | RE_FATAL, szFullPathname, NULL);

  return (bReturn);

} /* InitBuffer */


/* -----------------------------------------------------------------------------
   MakeStringBuffer

   Makes a string buffer from a string.
*/
void MakeStringBuffer (PINPUTBUF pBuf, PINPUTBUF pbufStr, PSTR sz)
{
  pbufStr->pfileIn = NULL;        /* Flags that is not file buffer */
  pbufStr->pbufCur = pbufStr->pbufOrg = sz;
  pbufStr->iLineNum = 0;        /* Multiline eqn formatting in modo */
  pbufStr->iLNPrev  = 0;
  pbufStr->pInfo = (pBuf ? pBuf->pInfo : NULL);

  if (pBuf) {
    pbufStr->iLineNum = pBuf->iLineNum;    /* For error reporting  */
    pbufStr->iLNPrev = TRUE;        /* Flag: Use iLineNum in ReportError */
  } /* if */

} /* MakeStringBuffer */


void FlushBuffer (PINPUTBUF pibIn)
{
  PBUF pbuf = pibIn->pbufOrg;

  while (*pbuf)
    printf ("%c", *pbuf++);
  printf ("");
} /* FlushBuffer */


/* -----------------------------------------------------------------------------
   SkipWhitespace

   Skips over whitespace of input buffer.  Returns non-zero if something
   has been skipped.
*/
int SkipWhitespace (PINPUTBUF pibIn)
{
  char c;
  int fSkipped = 0;

  if (!pibIn)
    return 0;

  if (!*pibIn->pbufCur && pibIn->pfileIn)
    FillBuffer (pibIn);

  /* Skip Spaces, Tabs and Newlines */

  while (isspace(c = *pibIn->pbufCur) || c == CH_COMMENT) {
    fSkipped = 1;
    if (c == CH_COMMENT)
      SkipComment (pibIn);

    else {
      if (c == '\n')
    pibIn->iLineNum++;

      if (!*(++pibIn->pbufCur) && pibIn->pfileIn)
    if (FillBuffer (pibIn) == EOF)
      break;
    } /* else */
  } /* while */

  return (fSkipped);

} /* SkipWhitespace */


/* ---------------------------------------------------------------------------
   GetArrayBounds
   return the lower (LB) upper (UB) bounds of an array given between []. 
   GetArrayBounds must be called after finding [. It reads up to ], included.
   Errors are generated if LB < 0 or UB < LB. 
   Syntax for bounds:
   [i]:   bounds returned are i to i+1
   [i-j]: bounds returned are i to j+1
   where i and j are long integers
*/
void GetArrayBounds (PINPUTBUF pibIn, PLONG piLB, PLONG piUB)
{
  PSTRLEX szTmp;

  if (ENextLex (pibIn, szTmp, LX_INTEGER)) {
    ReportError (pibIn, RE_INIT | RE_FATAL, NULL, NULL);
  }
  else {
    *piLB = atol(szTmp);
    if (*piLB < 0) 
      ReportError (pibIn, RE_POSITIVE | RE_FATAL, szTmp, NULL);

    if (NextChar (pibIn) == '-') { /* get eventual hyphen */
      pibIn->pbufCur++; /* advance */
      if (ENextLex (pibIn, szTmp, LX_INTEGER)) {
        ReportError (pibIn, RE_INIT | RE_FATAL, NULL, NULL);
      }
      else {
      *piUB = atol(szTmp) + 1;
        if (*piUB <= *piLB) 
         ReportError (pibIn, RE_UNKNOWN | RE_FATAL, "", 
                      "Upper bound must be higher than lower bound");
      }
      if (!GetPunct (pibIn, szTmp, ']')) { /* get closing bracket */
        ReportError (pibIn, RE_LEXEXPECTED | RE_FATAL, "]", NULL);
      }
    }
    else {
      if (!GetPunct (pibIn, szTmp, ']')) { /* get closing bracket */
        ReportError (pibIn, RE_LEXEXPECTED | RE_FATAL, "]", NULL);
      }
      else { /* a number is an index, the upper bound is set at LB+1 */
        *piUB = *piLB + 1;
      }
    }
  }
} /* GetArrayBounds */


/* -----------------------------------------------------------------------------
   GetaString

   Copies the quoted string from buffer to szLex.
*/
void GetaString (PINPUTBUF pibIn, PSTR szLex)
{
  int i = 0;

  if (!pibIn || !szLex)
    return;

  if (IsString ((PSTR) pibIn->pbufCur)) {
    do
      szLex[i++] = *++pibIn->pbufCur; /* Copy string */

    while ((*pibIn->pbufCur)
           && (*pibIn->pbufCur != CH_STRDELIM)
           && (i < MAX_LEX-1));
  } /* if */

  if (*pibIn->pbufCur == CH_STRDELIM) {
    pibIn->pbufCur++; /* Increment past closing delim */
    i--;
  }

  szLex[i] = '\0'; /* Overwrite closing delim with '\0' */

} /* GetaString */


/* -----------------------------------------------------------------------------
   GetIdentifier

   Copies identifier from buffer to szLex.  A valid id begins with a
   letter or '_' and is followed by alphanumerics or '_'.  MAX_LEX is
   the length of the longest permitable id.
*/
void GetIdentifier (PINPUTBUF pibIn, PSTR szLex)
{
  int i = 0;

  if (!pibIn || !szLex)
    return;

  if (isalpha(*pibIn->pbufCur) || IsUnderscore(*pibIn->pbufCur)) {
    do
    szLex[i++] = *pibIn->pbufCur++; /* Copy identifier */

    while ((*pibIn->pbufCur)
           && (isalnum(*pibIn->pbufCur) || IsUnderscore(*pibIn->pbufCur))
           && (i < MAX_LEX-1));
  } /* if */

  szLex[i] = '\0';

} /* GetIdentifier */


/* -----------------------------------------------------------------------------
   GetNumber

   Gets the next lexical element as either a floating-point or
   integer number from the input buffer.  piLexType is set accordingly.

   An integer is defined as a sequence of digits.

   A floating-point ::= (digits | dotted-digits) {exponent}?
   where
       dotted-digits ::= (ddd. | ddd.ddd | .ddd)
       {exponent}    ::= ('e' | 'E') {'+' | '-'}? digits

   Leading signs to the numbers are handled separately as unary
   or binary operators.

   The routine processes by using a state transition definition of
   parsing the number:
           ddd1 [[.[ddd2]] [E[+]ddd3]]
*/
void GetNumber (PINPUTBUF pibIn, PSTR szLex, PINT piLexType)
{
  int i = 0;
  char c;
  BOOL bHasSign = FALSE;
  BOOL bLeadingDigits = FALSE;
  enum States
   {Start, Digits1, Point, Digits2, Exp, ExpSign, Digits3, End} eState;

  if (!pibIn || !szLex || !piLexType)
    return;

  eState = Start;
  *piLexType = LX_NULL;
  while ((c = *pibIn->pbufCur)
         && i < MAX_LEX-1
     && eState != End) {

    switch (eState) {
      case Start:
    if (c == '.')
      eState = Point;
    else if (!bHasSign && IsSign(c))
      bHasSign = TRUE;
    else if (isdigit(*pibIn->pbufCur)) {
      bLeadingDigits = *piLexType = LX_INTEGER;
      eState = Digits1;
    } /* else */
    else
      eState = End;
    break;

      case Digits1:
    if (c == '.')
      eState = Point;
    else if (c == 'e' || c == 'E')
      eState = Exp;
    else if (!isdigit(c))
      eState = End;
    break;

      case Point:
    *piLexType = LX_FLOAT;
    if (bLeadingDigits && (c == 'e' || c == 'E'))
      eState = Exp;
    else if (isdigit(c))
      eState = Digits2;
    else {
      if (!bLeadingDigits)        /* Error, point only */
        *piLexType = LX_NULL;
      eState = End;
    } /* else */
    break;

      case Digits2:
    if (c == 'e' || c == 'E')
      eState = Exp;
    else if (!isdigit(c))
      eState = End;
    break;

      case Exp:
    *piLexType = LX_FLOAT;
    if (IsSign(c)) {
      eState = ExpSign;
      break;
    } /* if */
        /* Fall through! */

      case ExpSign:
    if (isdigit(c))
      eState = Digits3;
    else {
      *piLexType = LX_NULL;
      eState = End;
    }
    break;

      case Digits3:
    if (!isdigit(c))
      eState = End;
        break;

      case End:
    break;
    } /* switch */

    if (eState != End)
      szLex[i++] = *pibIn->pbufCur++;
  } /* while */
  szLex[i] = '\0';

} /* GetNumber */


/* -----------------------------------------------------------------------------
   NextLex

   Skips over leading whitespace and copies the next lexical element
   into szLex.
*/
void NextLex (PINPUTBUF pibIn, PSTRLEX szLex, PINT piLexType)
{
  char c;
  BOOL fDone = FALSE;

  *piLexType = LX_NULL;
  if (!pibIn || !szLex || !piLexType || !pibIn->pbufCur)
    return;

  while (!fDone) {
    fDone = TRUE;
    SkipWhitespace (pibIn);

    if (!EOB(pibIn)) {
      c = *pibIn->pbufCur;

      if (c == CH_COMMENT) { /* Comments can appear anywhere */
        fDone = FALSE; /* Continue until you get a lex */
        SkipComment (pibIn);
      } /* if */

      else
      if (isalpha(c) || IsUnderscore(c)) { /* Take one identifier */
        *piLexType = LX_IDENTIFIER;
        GetIdentifier (pibIn, szLex);
      } /* if */

      else
      if (isdigit(c) || c == '.' || IsSign(c)) { /* Take one number */
        GetNumber (pibIn, szLex, piLexType);
        if (IsSign(c) && !*piLexType) { /* Unary +/- for identifier */
          szLex[0] = c;
          szLex[1] = '\0';
          *piLexType = LX_PUNCT;
        } /* if */
      } /* if */

      else
      if (c == CH_STRDELIM) {
        *piLexType = LX_STRING;
        GetaString (pibIn, szLex);
      } /* if */

      else { /* Is other punctuation -- Take one char */
        *piLexType = LX_PUNCT;
        szLex[0] = *pibIn->pbufCur++;
        szLex[1] = '\0';
      } /* else */
    } /* if */
  } /* while */

} /* NextLex */


/* -----------------------------------------------------------------------------
   ENextLex

   Sames as NextLex but looks for iType.  Reports errors.  Returns
   0 if ok, non-zero if error.
*/
BOOL ENextLex (PINPUTBUF pibIn, PSTRLEX szLex, int iType)
{
  int iLex, iErr;

  NextLex (pibIn, szLex, &iLex);

  if ((iErr = !(iType & iLex)))
    ReportError (pibIn, RE_LEXEXPECTED, vrgszLexTypes[iType], szLex);

  return (iErr);

} /* ENextLex */


/* -----------------------------------------------------------------------------
   SkipComment

   Skips over the comment in the input buffer, the leading delimiter
   of which has already been stripped.
*/
void SkipComment (PINPUTBUF pibIn)
{
  if (!pibIn)
    return;

  if (!*pibIn->pbufCur)
    FillBuffer (pibIn);

  while (*pibIn->pbufCur++ != CH_EOLN) /* Eat 1 line comment */
    if (!*pibIn->pbufCur)
      if (FillBuffer (pibIn) == EOF)
    break;

  pibIn->iLineNum++;

  if (!*pibIn->pbufCur)
    FillBuffer (pibIn);

} /* SkipComment */


/* -----------------------------------------------------------------------------
   NextChar

   Returns the next character in the input buffer without advancing
   over it.
*/
char NextChar (PINPUTBUF pibIn)
{
  if (!pibIn
     || (!*pibIn->pbufCur
         && FillBuffer (pibIn) == EOF))
    return (0);

  else
    return (*pibIn->pbufCur);

} /* NextChar */


/* -----------------------------------------------------------------------------
   GetOptPunct

   Advances over the optional punctuation chPunct.  Allows syntax where either
   the punctuation or whitespace is valid.

   e.g. 'x 5;'  -or-  'x = 5;'
*/
int GetOptPunct (PINPUTBUF pibIn, PSTR szLex, char chPunct)
{
  int iReturn, iType;

  iReturn = SkipWhitespace (pibIn);
  if (NextChar (pibIn) == chPunct) {
    iReturn = TRUE;
    NextLex (pibIn, szLex, &iType);
  }
  return (iReturn);

} /* GetOptPunct */


/* -----------------------------------------------------------------------------
   GetPunct

   Tries to get the given punctuation from the input buffer.
   Returns TRUE if the next lexical item was the chPunct, else FALSE .
*/
int GetPunct (PINPUTBUF pibIn, PSTR szLex, char chPunct)
{
  int iType;

  NextLex (pibIn, szLex, &iType);
  return ((iType == LX_PUNCT) && szLex[0] == chPunct);

} /* GetPunct */


/* -----------------------------------------------------------------------------
   EGetPunct

   Tries to get the given punctuation from the input buffer.
   Returns 0 if next lexical item was the chPunct, non-zero if error.
   Reports Errors.
*/
int EGetPunct (PINPUTBUF pibIn, PSTR szLex, char chPunct)
{
  int iReturn;

  iReturn = !GetPunct (pibIn, szLex, chPunct);
  if (iReturn) {
    szLex[1] = chPunct;
    ReportError (pibIn, RE_EXPECTED, szLex, NULL);
  }

  return (iReturn);

} /* EGetPunct */


/* -----------------------------------------------------------------------------
   EatStatement

   Eats buffer to the statement terminator.
*/
void EatStatement (PINPUTBUF pib)
{
  char c;

  if (!pib)
    return;

  while ((c = NextChar (pib)) && (c != CH_STMTTERM)) {
    if (c == CH_EOLN)
      pib->iLineNum++;
    pib->pbufCur++;    /* Eat to ... */
  } /* while */

  if (c)
    pib->pbufCur++; /* ... and including statement terminator */
} /* EatStatement */


/* -----------------------------------------------------------------------------
   GetStatement

   Gets the next statement from the input buffer.  The buffer is
   read until a statement terminator ';' is found.  The statement is
   unprocessed.  Syntactical validity will be checked later.

   The buffer szStmt is assumed to be of type PSTREQN of size MAX_EQN.
*/
void GetStatement (PINPUTBUF pibIn, PSTR szStmt)
{
  int i, fDone = 0;

  if (!pibIn || !szStmt)
    return;

  SkipWhitespace (pibIn);

  i = 0;
  if (!EOB(pibIn)) {
    while (!fDone) {
      if (*pibIn->pbufCur) {
        if (!(fDone = (NextChar (pibIn) == CH_STMTTERM))) {
          if (i < MAX_EQN-2) {
            if ((szStmt[i++] = *pibIn->pbufCur++) == CH_EOLN)
              pibIn->iLineNum++;
          } /* if */
          else {
            ReportError (pibIn, RE_EQNTOOLONG | RE_FATAL, NULL, NULL);
          }
        } /* if !fDone... */
      } /* if pibIn->pbufCur */
      else {
        fDone = (FillBuffer (pibIn) == EOF);
      }
    } /* while */

    szStmt[i] = '\0';

  } /* if */

  if (!i)
    ReportError (pibIn,
                 RE_LEXEXPECTED | RE_FATAL, "rvalue to assignment", NULL);

} /* GetStatement */


/* -----------------------------------------------------------------------------
   NextListItem

   Copies the next list item to szLex if it is a lexical element
   indicated in the bit flags bIdTypes.  If the fItemNum flag is
   non-zero, a separator must appear in the list.

   Returns positive if a valid item is found.
   Returns negative if szLex is an invalid item.

   Will not eat list terminator.
*/
int NextListItem (PINPUTBUF pibIn, PSTR szLex,
                  int bIdTypes, int fItemNum, char cListTerm)
{
  int iType, iReturn = 0;

  if (!fItemNum
      || GetOptPunct (pibIn, szLex, ',')) {
    if (NextChar (pibIn) != cListTerm) {
      NextLex (pibIn, szLex, &iType);
      if (iType & bIdTypes)
    iReturn = 1;
      else
    iReturn = -1;
    } /* if */
  } /* if */

  return (iReturn);

} /* NextListItem */


/* -----------------------------------------------------------------------------
   GetFuncArgs

   Gets nArgs arguments to a "function" from pibIn. The argument
   list must be in parentheses.

   rgiArgTypes[] is a profile specifying which type each argument must be.
   rgszArgs[] is an array of PSTRLEX buffers to hold the arguments.

   Returns TRUE on success, FALSE on an error. Errors are reported,
   but the statement is not flushed.
*/
BOOL GetFuncArgs (PINPUTBUF pibIn,
                  int nArgs, int rgiArgTypes[], PSTR szArgs)
{
  BOOL bOK = TRUE;
  int i, iType;
  PSTRLEX szPunct;

  if (!(bOK = GetPunct(pibIn, szPunct, CH_LPAREN))) {
    szPunct[1] = CH_LPAREN;
    ReportError(pibIn, RE_EXPECTED, szPunct, NULL);
  } /* if */

  for (i = 0; i < nArgs && bOK; i++, szArgs += MAX_LEX) {
    if (i)
      if (!(bOK = GetOptPunct(pibIn, szArgs, ','))) {
        *(szArgs+1) = ',';
        ReportError(pibIn, RE_EXPECTED, szArgs, NULL);
        return(bOK); /* Error: Stop getting args */
      }

    NextLex (pibIn, szArgs, &iType);
    if (!(bOK &= (iType & rgiArgTypes[i]) > 0)) {
      ReportError(pibIn, RE_LEXEXPECTED,
                  vrgszLexTypes[rgiArgTypes[i]], szArgs);
      return(bOK); /* Error: Stop getting args */
    }
  } /* for */

  if (!(bOK = GetPunct (pibIn, szPunct, CH_RPAREN))) {
    szPunct[1] = CH_RPAREN;
    ReportError(pibIn, RE_EXPECTED, szPunct, NULL);
  } /* if */

  return(bOK);

} /* GetFuncArgs */


/* ---------------------------------------------------------------------------
   UnrollEquation

   Copy szEqn in szEqnU, replacing bracketed expressions evaluating in
   <number> by _number. Expressions can be composed of integers, the 
   4 basic arithmetic operators, parentheses and 'i' which stands for the
   argument index passed to the routine.
   Examples:
   y[0] -> y_0
   y[1 + 1] -> y_2
   y[i * 2] -> y_4 if index = 2
*/
void UnrollEquation (PINPUTBUF pibIn, long index, PSTR szEqn, PSTR szEqnU)
{
  int j = 0, k = 0, m;
  BOOL bExpress = FALSE;
  PSTRLEX szExpression;

  while ((szEqn[j] != '\0') && (k < MAX_EQN - 1)) {
    if (bExpress) { /* bracketed expressions found: scan it up to ] included */
      /* copy the expression to a temporary string */
      m = 0;
      while ((szEqn[j] != '\0') && (szEqn[j] != ']') && (m < MAX_EQN - 1)) {
        szExpression[m] = szEqn[j]; 
        j++;
        m++;
      }
      if (szEqn[j] == ']') { /* skip and exit expression parsing mode */
        j++;
        bExpress = FALSE;
      }
      if ((szEqn[j] != '\0') && (m == MAX_EQN - 1))
        ReportError (pibIn, RE_EQNTOOLONG | RE_FATAL, NULL, 
                     "(Occured while unrolling a loop)");
      szExpression[m] = '\0'; /* terminate szExpression */

      /* compute expression and put back the result in szExpression
      sprintf (szExpression, "%ld", 
               EvaluateExpression (pibIn, index, szExpression)); */

      /* copy szExpression into szEqnU */
      m = 0;
      while ((szExpression[m] != '\0') && (m < MAX_EQN - 1)) {
        szEqnU[k] = szExpression[m]; 
        k++;
        m++;
      }
    } /* end if bExpress */
    else switch (szEqn[j]) {
      case '[': /* replace by _ and enter expression parsing mode */
        szEqnU[k] = '_';
        j++;
        k++;
        bExpress = TRUE;
        break;

      case ']': /* should have been eaten in expression parsing mode */
        ReportError (pibIn, RE_UNEXPECTED | RE_FATAL, "]", 
                     "(Could be nested brackets)");

      default: /* copy and advance */
        szEqnU[k] = szEqn[j]; 
        j++;
        k++;
        break;
    }
  } /* while */
  if ((szEqn[j] != '\0') && (k == MAX_EQN - 1))
    ReportError (pibIn, RE_EQNTOOLONG | RE_FATAL, NULL, 
                 "(Occured in UnrollEquation)");
    
  /* terminate szEqnU */
  szEqnU[k] = '\0';

} /* UnrollEquation */


/* End */

