main ()
{
  int* x;
  x = malloc (sizeof (int));
  *x = 1;
}

func1 (int* p)
{
}

func2 (uint64_t* p)
{
  uint64_t* q;
  p = q * q;
}
