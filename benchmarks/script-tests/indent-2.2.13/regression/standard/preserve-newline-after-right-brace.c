int
aa (int b)
{
  int a = 1;
  if (a == 1) {
    a = 1;
  }
  if (a == 2) {
    a = 2;
  } else {
    a = 3;
  }

  /* test */
  if (!check (a)) {
    a = 0;
  }
}
