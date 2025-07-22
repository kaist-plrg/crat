/* strutil.h

   Originally written by Don Robert Maszle

   Copyright (c) 1992-2017 Free Software Foundation, Inc.

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

   Alternate string routines header file.  Routines for when NULL
   pointer dereferencing is a no-no.

   These handle NULL string pointer cases in a reasonable manner and
   ultimately call the standard library routines.
*/

#define  MyStrcpy(szDest, szSource) \
  ((szDest) && (szSource) ? strcpy((szDest), (szSource)) : NULL)

#define  MyStrlen(sz) \
  ((sz) ? strlen((sz)) : (int) 0)

#define  MyStrchr(sz, iChar) \
  ((sz) ? strchr((sz), (iChar)) : NULL)

#define  MyStrtok(sz, szToken) \
  ((sz) && (szToken) ? strtok((sz), (szToken)) : NULL)

/* ----------------------------------------------------------------------------
   Prototypes */

int MyStrcmp(const char* sz1, const char* sz2);

/* End */

