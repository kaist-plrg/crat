
void
bar (int foo)
{
  switch (foo)
    {
    case 0:
      abort ();
      break;

    case 1:
      fuckme ();
      break;

    default:
      abort ();
    }
}
