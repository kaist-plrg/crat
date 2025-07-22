extern EXETOKEN savetkn;	/* Last input token found.  */
extern EXETOKEN NOTOKEN;	/* Empty token              */

word
FilOpnTmp (uword maxr, word creat)	/* i: 0 - copies existing mastfil to tmpfil, 1- creates
					   tmpfil */
{
  here_is_an_incredibly_long_variable = my * but * this * (is + a) /
    lengthy * expression;

  for (mastfil_dta = 0; mastfil_dta < cnfils && (strcmp (mastfil->fname,
							 dtafilarr
							 [mastfil_dta]->
							 fname) != 0);
       mastfil_dta++);

  for (mastfil_dta = 0; mastfil_dta < cnfils; mastfil_dta++);
}
