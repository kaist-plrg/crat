int i = MACRO (int);
struct X x = { MACRO (int) };
void
foo ()
{
  MACRO (int);
}
