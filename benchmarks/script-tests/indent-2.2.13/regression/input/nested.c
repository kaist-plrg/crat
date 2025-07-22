#include <stdio.h>

void inner_function (int foo, int bar)
{
  int baz;

    printf ("Hi!\n");

  if (foo)
    printf ("Foo (%d)\n", foo);
  if (bar)
    printf ("Bar (%d)\n", bar);
}

void outer_function ()
{
  void
  inner_function (int foo, int bar)
  {
    int baz;

      printf ("Hi!\n");

    if (foo)
      printf ("Foo (%d)\n", foo);
    if (bar)
      printf ("Bar (%d)\n", bar);
  }

  inner_function (1, 1);
}

int
main (void)
{
  outer_function ();
  return 0;
}
