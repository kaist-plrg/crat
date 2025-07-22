/* btdt.c --- been there done that

   Copyright (C) 2010-2022 Thien-Thi Nguyen

   This file is part of GNU RCS.

   GNU RCS is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU RCS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
   See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#include "base.h"
#include <string.h>
#include <stdlib.h>
#include "stat-time.h"
#include "timespec.h"
#include "b-complain.h"
#include "b-divvy.h"
#include "b-esds.h"
#include "b-feph.h"
#include "b-fro.h"
#include "b-grok.h"

/* This program serves as a collection of test-support commands
   (to be invoked from the t??? files) for various components of
   the RCS library.  */

struct top *top;

exiting void
bad_args (char const *argv0)
{
  fprintf (stderr, "%s: bad args (try %s --help)\n",
           argv0, PROGRAM (invoke));
  _Exit (EXIT_FAILURE);
}

#define MORE "\n\t\t"


/* ‘getoldkeys’ */

/* Print the keyword values found.  */

char const getoldkeys_usage[] =
  "WORKING-FILE";

void
getoldkeys_spew (char const *what, char *s)
{
  if (s)
    printf ("%s: %zu \"%s\"\n", what, strlen (s), s);
}

int
getoldkeys_do_it (int argc, char *argv[VLA_ELEMS (argc)])
{
  if (2 > argc)
    bad_args (argv[0]);

  MANI (filename) = argv[1];
  getoldkeys (NULL);
  printf ("valid: %s\n", PREV (valid) ? "true" : "false");
  getoldkeys_spew ("revno", PREV (rev));
  getoldkeys_spew ("date", PREV (date));
  getoldkeys_spew ("author", PREV (author));
  getoldkeys_spew ("name", PREV (name));
  getoldkeys_spew ("state", PREV (state));
  return EXIT_SUCCESS;
}


/* ‘grok_all’ */

/* Parse an RCS file and display different aspects of the result.  */

char const grok_usage[] =
  "RCS-FILE [ASPECT...]"
  MORE "where ASPECT is one of:"
  MORE "  edits-order";

int
grok_do_it (int argc, char *argv[VLA_ELEMS (argc)])
{
  int i;
  struct fro *f;

  REPO (filename) = argv[1];            /* FIXME: for ‘RERR’ */
  if (! (f = fro_open (argv[1], "r", NULL)))
    RERR ("cannot open %s", argv[1]);
  if (! (REPO (r) = grok_all (SINGLE, f)))
    RERR ("grok_all failed for %s", argv[1]);

  for (i = 2; i < argc; i++)
    {
      struct delta *d;
      char const *aspect = argv[i];

      printf ("%s:\n", aspect);
      if (STR_SAME ("edits-order", aspect))
        for (struct wlink *ls = GROK (deltas); ls; ls = ls->next)
          {
            d = ls->entry;
            printf ("%s\n", d->num);
          }
      else
        bad_args (argv[0]);
    }

  return EXIT_SUCCESS;
}


/* xorlf */

/* XOR stdin with LF (aka '\n', 012, 0xA) to stdout.  */

char const xorlf_usage[] =
  "";

int
xorlf_do_it (int argc, RCS_UNUSED char *argv[VLA_ELEMS (argc)])
{
  int c;

  while (EOF != (c = getchar ()))
    putchar (c ^ 012);
  return EXIT_SUCCESS;
}


/* mtimecmp */

/* Consider mtime of both FILE1 and FILE2 as M1 and M2.
   If M1 < M2, display "-1" to stdout.
   If M1 = M2, display "0" to stdout.
   If M1 > M2, display "1" to stdout.  */

char const mtimecmp_usage[] =
  "FILE1 FILE2";

struct timespec
mtimecmp_grok (char const *filename)
{
  struct stat st;

  if (PROB (stat (filename, &st)))
    {
      fprintf (stderr, "mtimecmp: could not stat %s\n",
               filename);
      _Exit (EXIT_FAILURE);
    }

  return get_stat_mtime (&st);
}

int
mtimecmp_do_it (int argc, char *argv[VLA_ELEMS (argc)])
{
  struct timespec m1, m2;
  long int sign;

  if (3 > argc)
    bad_args (argv[0]);

  m1 = mtimecmp_grok (argv[1]);
  m2 = mtimecmp_grok (argv[2]);

  sign = timespec_cmp (m1, m2);
  if (0 > sign) sign = -1;
  if (0 < sign) sign =  1;

  printf ("%ld\n", sign);
  return EXIT_SUCCESS;
}


typedef int (main_t) (int argc, char *argv[VLA_ELEMS (argc)]);

struct yeah
{
  char const *component;
  char const *usage;
  main_t *whatever;
  bool scram;
};

#define YEAH(comp,out)  { #comp, comp ## _usage, comp ## _do_it, out }

struct yeah yeah[] =
  {
    YEAH (getoldkeys,   true),
    YEAH (grok,         true),
    YEAH (xorlf,        true),
    YEAH (mtimecmp,     true),
  };

#define NYEAH  (sizeof (yeah) / sizeof (struct yeah))

int
main (int argc, char *argv[VLA_ELEMS (argc)])
{
  char const *me = "btdt";

  if (STR_SAME ("--version", argv[1]))
    {
      printf ("btdt (%s) %s\n", PACKAGE_NAME, PACKAGE_VERSION);
      printf ("Copyright (C) 2010-2022 Thien-Thi Nguyen\n");
      printf ("License GPLv3+; GNU GPL version 3 or later"
              " <http://gnu.org/licenses/gpl.html>\n\n");
      argv[1] = "--help";
      /* fall through */
    }

  if (2 > argc || STR_SAME ("--help", argv[1]))
    {
      printf ("Usage: %s COMPONENT [ARG...]\n", me);
      for (size_t i = 0; i < NYEAH; i++)
        printf ("- %-10s %s\n", yeah[i].component, yeah[i].usage);
      printf ("\n(Read the source for details.)\n");
      return EXIT_SUCCESS;
    }

  for (size_t i = 0; i < NYEAH; i++)
    if (STR_SAME (yeah[i].component, argv[1]))
      {
        int exitstatus;
        const struct program program =
          {
            .invoke = me,
            .name  = argv[1],
            .tyag = (yeah[i].scram
                     ? TYAG_IMMEDIATE
                     : BOG_ZONK)
          };

        gnurcs_init (&program);
        exitstatus = yeah[i].whatever (argc - 1, argv + 1);
        gnurcs_goodbye ();
        return exitstatus;
      }

  fprintf (stderr, "%s: bad component (try --help): %s\n", me, argv[1]);
  return EXIT_FAILURE;
}

/* btdt.c ends here */
