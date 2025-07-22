void
Fnc()
{
  int i, j, k;
  int Sum = 0;

#ifdef t1
  for (i = 0; i < 10; i++)
  {
    for (j = 0; j < 10; j++)
      for (k = 0; k < 10; k++)
        Sum += i * 100 + j * 10 + k;
#elif defined(t2)
    for (i = 0; i < 10; i++)
    {
      for (j = 0; j < 10; j++)
        Sum += i * 10 + j;
#else
  for (i = 0; i < 10; i++)
  {
    Sum += i;
#endif
  }

  printf("%d\n", Sum);
}

