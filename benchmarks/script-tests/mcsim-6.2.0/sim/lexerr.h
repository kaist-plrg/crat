/* lexerr.h

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

   Header file for error reporting routine of lexerr.c
*/

#ifndef LEXERR_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions  */

#include "hungtype.h"
#include "lex.h"
#include "sim.h"

/* ----------------------------------------------------------------------------
   Prototypes */

void ReportError (PINPUTBUF, WORD, PSTR, PSTR);
void ReportRunTimeError (PANALYSIS, WORD, ...);

#define LEXERR_H_DEFINED
#endif

/* End */


