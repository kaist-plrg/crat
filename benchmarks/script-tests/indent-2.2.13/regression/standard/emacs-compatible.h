struct token_data
{
  enum token_data_type type;
  union
  {
    struct
    {
      char *text;
    }
    u_t;
    struct
    {
      builtin_func *func;
      boolean traced;
    }
    u_f;
  }
  u;
};

foo ()
{
  while (1)
    {
      foobar ();
    }

  do
    {
      ralph ();
    }
  while (nigel ());

  switch (x)
    {
    case 33:
      break;

    case 49:
      printf ("foobar");
      {
	int i;
	i = 23;
	f (i);
      }

    default:
      {
	int x;
	abort ();
      }
      ;
    }
}
