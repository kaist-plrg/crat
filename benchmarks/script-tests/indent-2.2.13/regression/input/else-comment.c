void foo()
{
  if (0)
    i = 0;
  else /* This is a dirty place to put a comment */ if (1)
    i = 0;
  else			/* comment behind else */
      /* comment on new line, after else */
    i = 0;

  if (0)
 	/* Lets put a comment here too */
    i = 0;
  else /* This is a dirty place
          to put a multi-line comment */ if (1)
    i = 0;
  else			/* multi-line comment
                           behind else */
    i = 0;

  if (0)	/* Lets put a comment here too */
    i = 0;
  else /* This is a dirty place to put a comment */ if (1)
    i = 0;
  else			/* comment behind else */
      /* comment on new line, after else */
    i = 0;

  if (0)
    i = 0;
  else if (1) /* comment after if */
    i = 0;
  else
    /* multi-line
       comment */
    i = 0;

  if (0)
  {
    i = 0;
  }
  else /* This is a dirty place to put a comment */ if (1)
  {
    i = 0;
  }
  else			/* comment behind else */
  {    /* comment on new line, after else */
    i = 0;
  }
  if (0)
  { 	/* Lets put a comment here too */
    i = 0;
  }
  else /* This is a dirty place
          to put a multi-line comment */ if (1)
  {
    i = 0;
  }
  else			/* multi-line comment
                           behind else */
  {
    i = 0;
  }

  if (0)	/* Lets put a comment here too */
  {
    i = 0;
  }
  else /* This is a dirty place to put a comment */ if (1)
  {
    i = 0;
  }
  else			/* comment behind else */
     /* comment on new line, after else */
  {
    i = 0;
  }

  if (0)
  {
    i = 0;
  }
  else if (1) /* comment after if */
  {
    i = 0;
  }
  else
  { /* multi-line
       comment */
    i = 0;
  }
}
