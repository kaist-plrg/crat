/* Copyright (C) 1986, 1989, 1992 Free Software Foundation, Inc. All rights
   reserved.

   Redistribution and use in source and binary forms are permitted
   provided that the above copyright notice and this paragraph are
   duplicated in all such forms and that any documentation, advertising
   materials, and other materials related to such distribution and use
   acknowledge that the software was developed by the University of
   California, Berkeley, the University of Illinois, Urbana, and Sun
   Microsystems, Inc.  The name of either University or Sun Microsystems
   may not be used to endorse or promote products derived from this
   software without specific prior written permission. THIS SOFTWARE IS
   PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES,
   INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF
   MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE. */


#include "sys.h"

/* Like malloc but get error if no storage available.  size really should be
   size_t, but not all systems have size_t, so I hope "unsigned" will work.
   It works for GNU style machines, where it is 32 bits, and works on MS-DOS.  */

char *
xmalloc (size)
     unsigned size;
{
  register char *val = (char *) malloc (size);
  if (!val)
    {
      fprintf (stderr, "indent: Virtual memory exhausted.\n");
      exit (1);
    }
#if defined (DEBUG)
  /* Fill it with garbage to detect code which depends on stuff being
     zero-filled.  */
  memset (val, 'x', size);
#endif
  return val;
}

/* Like realloc but get error if no storage available.  */

char *
xrealloc (ptr, size)
     char *ptr;
     unsigned size;
{
  register char *val = (char *) realloc (ptr, size);
  if (!val)
    {
      fprintf (stderr, "indent: Virtual memory exhausted.\n");
      exit (1);
    }
  return val;
}
