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


#include "sys.h"
#include "indent.h"
#include <ctype.h>

/* POSIX says that <fcntl.h> should exist.  Some systems might need to use
   <sys/fcntl.h> or <sys/file.h> instead.  */
#include <fcntl.h>

#include <sys/types.h>
#include <sys/stat.h>

/* number of levels a label is placed to left of code */
#define LABEL_OFFSET 2


/* Stuff that needs to be shared with the rest of indent. Documented in
   indent.h.  */
char *in_prog;
char *in_prog_pos;
char *cur_line;
unsigned int in_prog_size;
FILE *output;
char *buf_ptr;
char *buf_end;
int had_eof;
int out_lines;
int com_lines;

int suppress_blanklines = 0;
int comment_open;

int paren_target;

/* Use `perror' to print the system error message
   caused by OFFENDER. */

static char *errbuf;

void
sys_error (offender)
     char *offender;
{
  int size = strlen (offender);
  static int buffer_size;

  if (errbuf == 0)
    {
      buffer_size = size + 10;	/* Extra for random unix lossage */
      errbuf = (char *) xmalloc (buffer_size);
    }
  else if (size + 10 > buffer_size)
    {
      buffer_size = size + 10;
      errbuf = xrealloc (errbuf, buffer_size);
    }
  sprintf (errbuf, "indent: %s", offender);
  perror (errbuf);
  exit (1);
}

/* true if INDENT OFF is in effect */
static int inhibit_formatting;

void
dump_line ()
{				/* dump_line is the routine that actually
				   effects the printing of the new source. It
				   prints the label section, followed by the
				   code section with the appropriate nesting
				   level, followed by any comments */
  register int cur_col;
  register int target_col = 0;
  static not_first_line;

  if (parser_state_tos->procname[0])
    {
      if (troff)
	{
	  if (comment_open)
	    {
	      comment_open = 0;
	      fprintf (output, ".*/\n");
	    }
	  fprintf (output, ".Pr \"%.*s\"\n", parser_state_tos->procname_end - parser_state_tos->procname,
		   parser_state_tos->procname);
	}
      parser_state_tos->ind_level = 0;
      parser_state_tos->procname = "\0";
    }

  /* A blank line */
  if (s_code == e_code && s_lab == e_lab && s_com == e_com)
    {
      /* If we have a formfeed on a blank line, we should just output it,
         rather than treat it as a normal blank line.  */
      if (parser_state_tos->use_ff)
	{
	  putc ('\014', output);
	  parser_state_tos->use_ff = false;
	}
      else
	{
	  if (suppress_blanklines > 0)
	    suppress_blanklines--;
	  else
	    {
	      parser_state_tos->bl_line = true;
	      n_real_blanklines++;
	    }
	}
    }
  else if (!inhibit_formatting)
    {
      suppress_blanklines = 0;
      parser_state_tos->bl_line = false;
      if (prefix_blankline_requested && not_first_line)
	{
	  if (swallow_optional_blanklines && n_real_blanklines > 1)
	    n_real_blanklines = 1;
	  else if (n_real_blanklines == 0)
	    n_real_blanklines = 1;
	}

      while (--n_real_blanklines >= 0)
	putc ('\n', output);
      n_real_blanklines = 0;
      if (parser_state_tos->ind_level == 0)
	parser_state_tos->ind_stmt = 0;	/* this is a class A kludge. dont do
					   additional statement indentation
					   if we are at bracket level 0 */

      if (e_lab != s_lab || e_code != s_code)
	++code_lines;		/* keep count of lines with code */


      if (e_lab != s_lab)
	{			/* print lab, if any */
	  if (comment_open)
	    {
	      comment_open = 0;
	      fprintf (output, ".*/\n");
	    }
	  while (e_lab > s_lab && (e_lab[-1] == ' ' || e_lab[-1] == '\t'))
	    e_lab--;
	  cur_col = pad_output (1, compute_label_target ());
	  if (s_lab[0] == '#' && (strncmp (s_lab, "#else", 5) == 0
				  || strncmp (s_lab, "#endif", 6) == 0))
	    {
	      /* Treat #else and #endif as a special case because any text
	         after #else or #endif should be converted to a comment.  */
	      register char *s = s_lab;
	      if (e_lab[-1] == '\n')
		e_lab--;
	      do
		putc (*s++, output);
	      while (s < e_lab && 'a' <= *s && *s <= 'z');
	      while ((*s == ' ' || *s == '\t') && s < e_lab)
		s++;
	      if (s < e_lab)
		fprintf (output, s[0] == '/' && s[1] == '*' ? "\t%.*s" : "\t/* %.*s */",
			 e_lab - s, s);
	    }
	  else
	    fprintf (output, "%.*s", e_lab - s_lab, s_lab);
	  cur_col = count_spaces (cur_col, s_lab);
	}
      else
	cur_col = 1;		/* there is no label section */

      parser_state_tos->pcase = false;

      if (s_code != e_code)
	{			/* print code section, if any */
	  register char *p;

	  if (comment_open)
	    {
	      comment_open = 0;
	      fprintf (output, ".*/\n");
	    }
	  target_col = compute_code_target ();
	  /* If a line ends in an lparen character, the following line should
	     not line up with the parenthesis, but should be indented by the
	     usual amount.  */
	  if (parser_state_tos->last_token == lparen)
	    {
	      parser_state_tos->paren_indents[parser_state_tos->p_l_follow - 1]
		+= ind_size - 1;
	    }
	  {
	    register i;

	    for (i = 0; i < parser_state_tos->p_l_follow; i++)
	      if (parser_state_tos->paren_indents[i] >= 0)
		parser_state_tos->paren_indents[i]
		  = -(parser_state_tos->paren_indents[i] + target_col);
	  }
	  cur_col = pad_output (cur_col, target_col);
	  for (p = s_code; p < e_code; p++)
	    if (*p == (char) 0200)
	      fprintf (output, "%d", target_col * 7);
	    else
	      putc (*p, output);
	  cur_col = count_spaces (cur_col, s_code);
	}

      if (s_com != e_com)
	{
	  if (troff)
	    {
	      int all_here = 0;
	      register char *p;

	      if (e_com[-1] == '/' && e_com[-2] == '*')
		e_com -= 2, all_here++;
	      while (e_com > s_com && e_com[-1] == ' ')
		e_com--;
	      *e_com = 0;
	      p = s_com;
	      while (*p == ' ')
		p++;
	      if (p[0] == '/' && p[1] == '*')
		p += 2, all_here++;
	      else if (p[0] == '*')
		p += p[1] == '/' ? 2 : 1;
	      while (*p == ' ')
		p++;
	      if (*p == 0)
		goto inhibit_newline;
	      if (comment_open < 2 && parser_state_tos->box_com)
		{
		  comment_open = 0;
		  fprintf (output, ".*/\n");
		}
	      if (comment_open == 0)
		{
		  if ('a' <= *p && *p <= 'z')
		    *p = *p + 'A' - 'a';
		  if (e_com - p < 50 && all_here == 2)
		    {
		      register char *follow = p;
		      fprintf (output, "\n.nr C! \\w\1");
		      while (follow < e_com)
			{
			  switch (*follow)
			    {
			    case '\n':
			      putc (' ', output);
			    case 1:
			      break;
			    case '\\':
			      putc ('\\', output);
			    default:
			      putc (*follow, output);
			    }
			  follow++;
			}
		      putc (1, output);
		    }
		  fprintf (output, "\n./* %dp %d %dp\n",
			   parser_state_tos->com_col * 7,
			   (s_code != e_code || s_lab != e_lab) - parser_state_tos->box_com,
			   target_col * 7);
		}
	      comment_open = 1 + parser_state_tos->box_com;
	      while (*p)
		{
		  if (*p == BACKSLASH)
		    putc (BACKSLASH, output);
		  putc (*p++, output);
		}
	    }
	  else
	    {			/* print comment, if any */
	      register target = parser_state_tos->com_col;
	      register char *com_st = s_com;

	      target += parser_state_tos->comment_delta;
	      while (*com_st == '\t')
		com_st++, target += tabsize;

	      while (target <= 0)
		if (*com_st == ' ')
		  target++, com_st++;
		else if (*com_st == '\t')
		  {
		    target = ((target - 1) & ~(tabsize - 1)) + (tabsize + 1);
		    com_st++;
		  }
		else
		  target = 1;
	      if (cur_col > target)
		{		/* if comment cant fit on this line, put it
				   on next line */
		  putc ('\n', output);
		  cur_col = 1;
		  ++out_lines;
		}
	      while (e_com > com_st && isspace (e_com[-1]))
		e_com--;
	      cur_col = pad_output (cur_col, target);
	      if (!parser_state_tos->box_com)
		{
		  if (star_comment_cont
		      && (com_st[1] != '*' || e_com <= com_st + 1))
		    if (com_st[1] == ' '
			&& com_st[0] == ' ' && e_com > com_st + 1)
		      com_st[1] = '*';
		    else
		      fwrite (" * ", (com_st[0] == '\t'
				      ? 2 : (com_st[0] == '*' ? 1 : 3)),
			      1, output);
		}
	      fwrite (com_st, e_com - com_st, 1, output);
	      parser_state_tos->comment_delta
		= parser_state_tos->n_comment_delta;
	      cur_col = count_spaces (cur_col, com_st);
	      ++com_lines;	/* count lines with comments */
	    }
	}

      if (parser_state_tos->use_ff)
	{
	  putc ('\014', output);
	  parser_state_tos->use_ff = false;
	}
      else
	putc ('\n', output);
    inhibit_newline:
      ++out_lines;
      if (parser_state_tos->just_saw_decl == 1
	  && blanklines_after_declarations)
	{
	  prefix_blankline_requested = 1;
	  parser_state_tos->just_saw_decl = 0;
	}
      else
	prefix_blankline_requested = postfix_blankline_requested;
      postfix_blankline_requested = 0;
    }

  /* if we are in the middle of a declaration, remember that fact
     for proper comment indentation */
  parser_state_tos->decl_on_line = parser_state_tos->in_decl;

  /* next line should be indented if we have not completed this
     stmt and if we are not in the middle of a declaration */
  parser_state_tos->ind_stmt = (parser_state_tos->in_stmt
				& ~parser_state_tos->in_decl);

  parser_state_tos->dumped_decl_indent = 0;
  *(e_lab = s_lab) = '\0';	/* reset buffers */
  *(e_code = s_code) = '\0';
  *(e_com = s_com) = '\0';
  parser_state_tos->ind_level = parser_state_tos->i_l_follow;
  parser_state_tos->paren_level = parser_state_tos->p_l_follow;
  if (parser_state_tos->paren_level > 0)
    paren_target
      = -parser_state_tos->paren_indents[parser_state_tos->paren_level - 1];
  else
    paren_target = 0;
  not_first_line = 1;
  return;
}

/* Figure out where we should put the code in codebuf. Return the column
   number in spaces.  */

INLINE int
compute_code_target ()
{
  register target_col = parser_state_tos->ind_level + 1;

  if (parser_state_tos->paren_level)
    if (!lineup_to_parens)
      target_col += continuation_indent * parser_state_tos->paren_level;
    else
      {
	register w;
	register t = paren_target;

	if ((w = count_spaces (t, s_code) - max_col) > 0
	    && count_spaces (target_col, s_code) <= max_col)
	  {
	    t -= w + 1;
	    if (t > target_col)
	      target_col = t;
	  }
	else
	  target_col = t;
      }
  else if (parser_state_tos->ind_stmt)
    target_col += continuation_indent;
  return target_col;
}

INLINE int
compute_label_target ()
{
  return
  parser_state_tos->pcase ? case_ind + 1
  : *s_lab == '#' ? 1
  : parser_state_tos->ind_level - LABEL_OFFSET + 1;
}

/* Read file FILENAME into a `fileptr' structure, and return a pointer to
   that structure. */

static struct file_buffer fileptr;

struct file_buffer *
read_file (filename)
     char *filename;
{
  int fd;
  struct stat file_stats;
  int namelen = strlen (filename);

  fd = open (filename, O_RDONLY, 0777);
  if (fd < 0)
    sys_error (filename);

  if (fstat (fd, &file_stats) < 0)
    sys_error (filename);

  if (fileptr.data != 0)
    free (fileptr.data);
  fileptr.size = file_stats.st_size;
  fileptr.data = (char *) xmalloc (file_stats.st_size + 1);

  if (read (fd, fileptr.data, fileptr.size) < 0)
    sys_error (filename);

  if (close (fd) < 0)
    sys_error (filename);

  fileptr.name = (char *) xmalloc (namelen + 1);
  memcpy (fileptr.name, filename, namelen);
  fileptr.name[namelen] = '\0';

  fileptr.data[fileptr.size] = '\0';

  return &fileptr;
}

/* This should come from stdio.h and be some system-optimal number */
#ifndef BUFSIZ
#define BUFSIZ 1024
#endif

/* Suck the standard input into a file_buffer structure, and
   return a pointer to that structure. */

struct file_buffer stdinptr;

struct file_buffer *
read_stdin ()
{
  unsigned int size = 15 * BUFSIZ;
  int ch;
  register char *p;

  if (stdinptr.data != 0)
    free (stdinptr.data);

  stdinptr.data = (char *) xmalloc (size + 1);
  stdinptr.size = 0;
  p = stdinptr.data;
  do
    {
      while (stdinptr.size < size)
	{
	  ch = getc (stdin);
	  if (ch == EOF)
	    break;

	  *p++ = ch;
	  stdinptr.size++;
	}

      if (ch != EOF)
	{
	  size += (2 * BUFSIZ);
	  stdinptr.data = xrealloc (stdinptr.data, size);
	}
    }
  while (ch != EOF);

  stdinptr.name = "Standard Input";

  stdinptr.data[stdinptr.size] = '\0';

  return &stdinptr;
}

/* Advance buf_ptr so that it points to the next line of input.  Skip over
   indent errors (comments beginning with *INDENT**), ignoring them.  Process
   INDENT ON and INDENT OFF. (Note: the name of this function is a historical
   artifact from before the time that indent kept the whole source file in
   memory). */

INLINE void
fill_buffer ()
{
  /* Point various places in the buffer.  */
  register char *p;

  /* Character P points to. */
  register char c;

  /* Have we found INDENT ON or INDENT OFF ? */
  enum
    {
      None, Indent_on, Indent_off
    } com;

  /* indent() may be saving the text between "if (...)" and the following
     statement.  To do so, it uses another buffer (`save_com').  Switch
     back to the previous buffer here. */
  if (bp_save != 0)
    {
      buf_ptr = bp_save;
      buf_end = be_save;
      bp_save = be_save = 0;

      /* only return if there is really something in this buffer */
      if (buf_ptr < buf_end)
	return;
    }

fill_it:

  cur_line = in_prog_pos;
  buf_ptr = in_prog_pos;
  if (*buf_ptr == '\0')
    {
      had_eof = true;
      return;
    }

  p = buf_ptr;
  do
    {
      c = *p;
      p++;
    }
  while (c != '\0' && c != '\n');
  buf_end = p;

  p = buf_ptr;
  in_prog_pos = buf_end;

  while (*p == ' ' || *p == '\t')
    p++;
  if (*p == '/' && p[1] == '*')
    {
      p += 2;
      if (p[1] == 'I' && strncmp (p, "*INDENT**", 9) == 0)
	goto fill_it;
      while (*p == ' ' || *p == '\t')
	p++;
      com = None;
      if (p[0] == 'I' && p[1] == 'N' && p[2] == 'D' && p[3] == 'E'
	  && p[4] == 'N' && p[5] == 'T')
	{
	  p += 6;
	  while (*p == ' ' || *p == '\t')
	    p++;
	  if (*p == '*')
	    com = Indent_on;
	  else if (*p == 'O')
	    if (*++p == 'N')
	      p++, com = Indent_on;
	    else if (*p == 'F' && *++p == 'F')
	      p++, com = Indent_off;
	  while (*p == ' ' || *p == '\t')
	    p++;
	  if (p[0] == '*' && p[1] == '/' && p[2] == '\n' && com)
	    {
	      if (s_com != e_com || s_lab != e_lab || s_code != e_code)
		dump_line ();
	      if (!(inhibit_formatting = (int) com - 1))
		{
		  n_real_blanklines = 0;
		  postfix_blankline_requested = 0;
		  prefix_blankline_requested = 0;
		  suppress_blanklines = 1;
		}
	    }
	}
    }
  if (inhibit_formatting)
    {
      p = buf_ptr;
      do
	putc (*p, output);
      while (*p++ != '\n');
    }

}


/* Copyright (C) 1976 by the Board of Trustees of the University of Illinois

All rights reserved


NAME: pad_output

FUNCTION: Writes tabs and spaces to move the current column up to the desired
   position.

ALGORITHM: Put tabs and/or blanks into pobuf, then write pobuf.

PARAMETERS: current		integer		The current column target
   nteger		The desired column

RETURNS: Integer value of the new column.  (If current >= target, no action
   is taken, and current is returned.

GLOBALS: None

CALLS: write (sys)

CALLED BY: dump_line

HISTORY: initial coding 	November 1976	D A Willcox of CAC */

INLINE int
pad_output (current, target)	/* writes tabs and blanks (if necessary) to
				   get the current output position up to the
				   target column */
     int current;		/* the current column value */
     int target;		/* position we want it at */
{
  register int curr;		/* internal column pointer */
  register int tcur;

  if (troff)
    fprintf (output, "\\h'|%dp'", (target - 1) * 7);
  else
    {
      if (current >= target)
	return (current);	/* line is already long enough */
      curr = current;
      while ((tcur = curr + tabsize - (curr - 1) % tabsize) <= target)
	{
	  putc ('\t', output);
	  curr = tcur;
	}
      while (curr++ < target)
	putc (' ', output);	/* pad with final blanks */
    }
  return (target);
}

/* Copyright (C) 1976 by the Board of Trustees of the University of Illinois

All rights reserved


NAME: count_spaces

FUNCTION: Find out where printing of a given string will leave the current
   character position on output.

ALGORITHM: Run thru input string and add appropriate values to current
   position.

RETURNS: Integer value of position after printing "buffer" starting in column
   "current".

HISTORY: initial coding 	November 1976	D A Willcox of CAC */

INLINE int
count_spaces (current, buffer)
     /* this routine figures out where the character position will be after
        printing the text in buffer starting at column "current" */
     int current;
     char *buffer;
{
  register char *buf;		/* used to look thru buffer */
  register int cur;		/* current character counter */

  cur = current;

  for (buf = buffer; *buf != '\0'; ++buf)
    {
      switch (*buf)
	{

	case '\n':
	case 014:		/* form feed */
	  cur = 1;
	  break;

	case '\t':
	  cur = cur + tabsize - (cur - 1) % tabsize;
	  break;

	case 010:		/* backspace */
	  --cur;
	  break;

	default:
	  ++cur;
	  break;
	}			/* end of switch */
    }				/* end of for loop */
  return (cur);
}

/* Nonzero if we have found an error (not a warning).  */
int found_err;

/* Signal an error.  LEVEL is nonzero if it is an error (as opposed to a
   warning.  MSG is a printf-style format string.  Additional arguments are
   additional arguments for printf.  */
/* VARARGS2 */
diag (level, msg, a, b)
     int level;
     unsigned int a, b;
     char *msg;
{
  if (level)
    found_err = 1;
  if (output == stdout)
    {
      fprintf (stdout, "/**INDENT** %s@%d: ", level == 0 ? "Warning" : "Error", line_no);
      fprintf (stdout, msg, a, b);
      fprintf (stdout, " */\n");
    }
  else
    {
      fprintf (stderr, "%s: %d: ", in_name, line_no);
      fprintf (stderr, msg, a, b);
      fprintf (stderr, "\n");
    }
}

writefdef (f, nm)
     register struct fstate *f;
     unsigned int nm;
{
  fprintf (output, ".ds f%c %s\n.nr s%c %d\n",
	   nm, f->font, nm, f->size);
}

/* Write characters starting at S to change the font from OF to NF.  Return a
   pointer to the character after the last character written. For troff mode
   only.  */
char *
chfont (of, nf, s)
     register struct fstate *of, *nf;
     char *s;
{
  if (of->font[0] != nf->font[0]
      || of->font[1] != nf->font[1])
    {
      *s++ = '\\';
      *s++ = 'f';
      if (nf->font[1])
	{
	  *s++ = '(';
	  *s++ = nf->font[0];
	  *s++ = nf->font[1];
	}
      else
	*s++ = nf->font[0];
    }
  if (nf->size != of->size)
    {
      *s++ = '\\';
      *s++ = 's';
      if (nf->size < of->size)
	{
	  *s++ = '-';
	  *s++ = '0' + of->size - nf->size;
	}
      else
	{
	  *s++ = '+';
	  *s++ = '0' + nf->size - of->size;
	}
    }
  return s;
}

void
parsefont (f, s0)
     register struct fstate *f;
     char *s0;
{
  register char *s = s0;
  int sizedelta = 0;
  int i;

  f->size = 0;
  f->allcaps = 1;
  for (i = 0; i < 4; i++)
    f->font[i] = 0;

  while (*s)
    {
      if (isdigit (*s))
	f->size = f->size * 10 + *s - '0';
      else if (isupper (*s))
	if (f->font[0])
	  f->font[1] = *s;
	else
	  f->font[0] = *s;
      else if (*s == 'c')
	f->allcaps = 1;
      else if (*s == '+')
	sizedelta++;
      else if (*s == '-')
	sizedelta--;
      else
	{
	  fprintf (stderr, "indent: bad font specification: %s\n", s0);
	  exit (1);
	}
      s++;
    }
  if (f->font[0] == 0)
    f->font[0] = 'R';
  if (bodyf.size == 0)
    bodyf.size = 11;
  if (f->size == 0)
    f->size = bodyf.size + sizedelta;
  else if (sizedelta > 0)
    f->size += bodyf.size;
  else
    f->size = bodyf.size - f->size;
}

#ifdef DEBUG
void
dump_debug_line ()
{
  fprintf (output, "\n*** Debug output marker line ***\n");
}

#endif
