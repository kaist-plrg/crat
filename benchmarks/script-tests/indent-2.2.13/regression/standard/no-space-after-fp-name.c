int (*fp())();

register void (*GCForward)() asm("r16");	/* GC Forwarding code */
char **(*get_string_array)(const char *s1, const char *s2);

main()
{
  switch (fork())
    {
    default:
      break;
    }
}

int *
function_2(var)
     int var;
{
  int *var_1_0;

  var_1_0 = (int *) make_shape_1d(var);

  return (var_1_0);
}
