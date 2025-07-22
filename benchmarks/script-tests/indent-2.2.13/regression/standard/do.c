foo (a)
     int a;
{
  int b;

  if (a == 1)
    do
      b = getchar ();
    while (b != 0x1a);
  else
    do
      b = getchar ();
    while (b != 0xff);

  do
    {
      b = getchar ();
    }
  while (b != 0xff);
}
