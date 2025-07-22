/* matutilo.c

   Written by Don Robert Maszle
   18 September 1992

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

   Matrix utilitites.  Output routines.
*/

#include <stdio.h>
#include <math.h>

#include "matutilo.h"

/* -----------------------------------------------------------------------------
   WriteArray

   writes the elements of an array, tab separated, to a specified file.

   The trailing tab is not printed, nor is  carriage return.
*/

void WriteArray (FILE *pfile, long cElems, double *rg)
{
  register long i;
  register long cElems_minus_1 = cElems - 1;

  for (i = 0; i < cElems; i++) {
    fprintf(pfile, "%g", rg[i]);
    if (i < cElems_minus_1) fputc ('\t', pfile);
  } /* for */
} /* WriteArray */


void WriteArrayExp (FILE *pfile, long cElems, double *rg)
{
  register long i;
  register long cElems_minus_1 = cElems - 1;

  for (i = 0; i < cElems; i++) {
    fprintf(pfile, "%g", exp(rg[i]));
    if (i < cElems_minus_1) fputc ('\t', pfile);
  } /* for */
} /* WriteArrayExp */


void _walog (long cElems, double *rg)
{
  int i;
  double dSum = 0.0;
  printf ("{");
  for (i = 0; i < cElems; i++) {
    dSum += exp(rg[i]);
    printf("%s%g", (i ? ", " : ""), exp(rg[i]));
  } /* for */
  printf ("} => %g [%g]\n", dSum, 1.0-dSum);
} /* _walog */

