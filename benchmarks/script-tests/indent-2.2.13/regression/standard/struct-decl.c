enum
{
  t1,
  t2
} barfoo;

struct foobar
{
  int ch;
  char f;
} xyz;

main ()
{
  enum
  {
    None, Indent_on, Indent_off
  }
  com;

  struct foobar
  {
    int ch;
    char f;
  }
  xyz;
}
