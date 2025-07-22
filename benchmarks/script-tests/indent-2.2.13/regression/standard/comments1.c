/* Copyright (c) 1992, Free Software Foundation, Inc.  All rights reserved.

   Copyright (c) 1985 Sun Microsystems, Inc. Copyright (c) 1980 The Regents
   of the University of California. Copyright (c) 1976 Board of Trustees of
   the University of Illinois. All rights reserved.

   Redistribution and use in source and binary forms are permitted
   provided that
   the above copyright notice and this paragraph are duplicated in all such
   forms and that any documentation, advertising materials, and other
   materials related to such distribution and use acknowledge that the
   software was developed by the University of California, Berkeley, the
   University of Illinois, Urbana, and Sun Microsystems, Inc.  The name of
   either University or Sun Microsystems may not be used to endorse or
   promote products derived from this software without specific prior written
   permission. THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
   IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES
   OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE. */



#define label_offset 2		/* number of levels a label is placed to left
				   of code */

int c;				/* And here is another comment which
				   occupies more than two lines. */

/* profile types */
enum profile
{
  PRO_BOOL,			/* boolean */
  PRO_INT,			/* integer */
  PRO_FONT,			/* troff font */
  PRO_IGN,			/* ignore it */
  PRO_KEY,			/* -T switch */
  PRO_SETTINGS,			/* bundled set of settings */
  PRO_PRSTRING			/* Print string and exit */
};

/* profile specials for booleans */
enum on_or_off
{
  ONOFF_NA,			/* Not applicable.  Used in table for
				   non-booleans.  */
  OFF,				/* This option turns on the boolean variable
				   in question.  */
  ON				/* it turns it off */
};



void
parse (tk)
     enum codes tk;		/* the code for the construct scanned */
{
  int i;

  while (parser_state_tos->p_stack[parser_state_tos->tos] == ifhead
	 && tk != elselit)
    {
      /* true if we have an if without an else */

      /* apply the if(..) stmt ::= stmt reduction */
      parser_state_tos->p_stack[parser_state_tos->tos] = stmt;
      reduce ();		/* see if this allows any reduction */
    }


  switch (tk)
    {				/* go on and figure out what to do with the
				   input */

    case decl:			/* scanned a declaration word */
      parser_state_tos->search_brace = btype_2;
      /* indicate that following brace should be on same line */
      if (parser_state_tos->p_stack[parser_state_tos->tos] != decl)
	{			/* only put one declaration onto stack */
	  break_comma = true;	/* while in declaration, newline should be
				   forced after comma */
	  inc_pstack ();
	}

    default:
      break;
    }
}
