int aa(int b)
{
  int a = 1;;
  if (a == 1) /* Cuddle? */ {
    a = 2;
  } /* if (a == 1 ... */
  /* look some more */
  if (a == 2) {
    a = 3;
  } /* if (a == 2 ... */
  else /* a != 2 */ {
    a = 4;
    }
  /* call check a */
  if (!check(a)) {
    a = 0;
  }
}

