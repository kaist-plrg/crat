char  *
frob (
       /* comment1 */
       struct foo *param1,
       /* comment 2
        * continued. */
       int param (int (*)(char, void *,	/* Really a
					 * `struct foo *' */
			  char ccccccc,	/* Extra param */
			  int *)), char *fooooooooooooooooo	/* comment3, also
								 * continued */ )
{
  int    aaaaaaaaaaaaaaaaaaaaa,	/* comment1 */
         bc,			/* comment2 */
         def;			/* comment3 */
  if (1)
    {
      char  *a,			/* comment1 */
            *bcccccccccccccccc,	/* comment2 */
             d;			/* comment3 */
    }
  if (strcmp (a /* comment a */ ,
	      b, /* comment b */ ))
    ;
  if (func_xxxxxxxxxxxxxxxx (a,	/* comment 1 */
			     b,	/* comment 2,
				 * continued. */
			     c,	/* comment 3 */
			     d (a + 11,	/* comment 4a */
				b - 11,	/* comment 4b
					 * continued. */
				11111),	/* comment 5 */
			     e))
    ;
}
