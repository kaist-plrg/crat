/* Copyright (C) 1992 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it without restriction.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  */

#include <stdio.h>

#ifdef DEBUG
extern int debug;
#endif

#ifdef __STDC__
#define INLINE inline
#else
#define INLINE
#endif

struct file_buffer
{
  char *name;
  int size;
  char *data;
};

extern struct file_buffer *read_file (), *read_stdin ();

/* Standard memory allocation routines.  */
char *malloc ();
char *realloc ();

/* Similar, but abort with an error if out of memory (see globs.c).  */
char *xmalloc ();
char *xrealloc ();
