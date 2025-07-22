
cmd_tab (n)
     int n;
{
  int i, pos, col;

  if (tabch)
    {
      i = 0;
      ch = tabch;
      while (tabstop[i] <= vcol)
	i++;
      do
	{
	  pos = tabstop[i++];
	  col = vcol;
	  if (overwrite)
	    do
	      {
		FWD;
	      }
	    while (pos > vcol && vcol > col);
	  else
	    do
	      {
		ch = tabch;
		ch1 = tabch2;
		inst_ch (0);
	      }
	    while (pos > vcol && vcol > col);
	}
      while (--n > 0);
      return (kcol = 0);
    }
  if (overwrite)
    {
      do
	{
	  do
	    {
	      FWD;
	    }
	  while (vcol & TABCNT);
	}
      while (--n > 0);
      return (0);
    }
  return (inst_ch (n));
}
