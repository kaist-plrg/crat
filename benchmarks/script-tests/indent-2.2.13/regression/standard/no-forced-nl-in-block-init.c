void
foo ()
{
  struct_a arr[] = {
    {&a, sizeof (a)},
    {&b, sizeof (b)},
    {&c, sizeof (c)},
    {&d, sizeof (d)}
  };

  struct
  {
    int a;
    int b;
  } v = { 0 };
}
