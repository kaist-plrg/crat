int
foo ()
{
  if (long_function_name (a,
#if FOO				/* comment1 */
			  b))
#else	/* comment2 */
			  c, d))
#endif	/* comment3 */
    ++ i;
  return 0;
}
