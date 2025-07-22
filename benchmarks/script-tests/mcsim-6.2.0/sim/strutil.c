/* strutil.c

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

   The following routines are provided as alternates to the standard
   library routines.  They handle NULL string pointer cases in a
   reasonable manner and ultimately call the standard library routines
   for non-NULL cases.  Some routines are implemented as macros.

   Use the header file "strutil.h"

   Currently supported are:

   MyStrlen(), MyStrcpy(), MyStrcmp(), MyStrchr(), MyStrtok()
*/

#include <string.h>

#include "strutil.h"

int MyStrcmp(const char* sz1, const char* sz2)
{
  if (!sz1) {
    if (sz2)
      return (-1);  /* NULL comes before the -something- */
    else
      return (0);   /* Two NULL strings compare equal */
  } /* if */

  else { /* assert (sz1) */
    if (sz2)
      return (strcmp(sz1, sz2));  /* Normal comparison */
    else
      return (1);   /* -Something- comes after the NULL */
  } /* else */

  /* Prevent compiler complaints */
  return 0; /* Never reached! */

} /* MyStrcmp */


/* End */

