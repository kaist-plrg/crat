foo ()
{
  do
  {
    b = getchar ();
  } while (b != 0xff);

  do
    b = getchar ();
  while (b != 0xff);
}
