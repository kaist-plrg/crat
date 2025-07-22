/* hungtype.h

   Written by Don Robert Maszle
   10 September 1991

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

   Hungarian style type defs.
*/

#ifndef HUNGTYPE_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions
*/

#include <stdio.h>


/* ----------------------------------------------------------------------------
   Typedefs
*/

typedef int            BOOL;
typedef unsigned char  BYTE;
typedef unsigned int   WORD;
typedef unsigned long  DWORD;
typedef DWORD          HANDLE;

typedef char*          PSTR;
typedef BYTE*          PBYTE;
typedef int*           PINT;
typedef WORD*          PWORD;
typedef long*          PLONG;
typedef DWORD*         PDWORD;
typedef void*          PVOID;

typedef float*         PFLOAT;
typedef double*        PDOUBLE;

typedef FILE*          PFILE;

#ifndef TRUE
#define TRUE 1
#define FALSE 0
#endif

#define HUNGTYPE_H_DEFINED
#endif

/* End */

