void
main ()
{
  if (1 == 1)
    {
      G_fatal_error (xx
		     ("A big long string that is treated as a function argument ends up breaking away from its function name."));
    }
  else if (2 == 2)
    {
      G_fatal_error
	(_("When the big long string is in the _() from gettext, however, it's better to treat the _ as a string delimiter rather than a function name."));
    }
  else
    {
      G_fatal_error
	(N_("When the big long string is in the N_() from gettext, however, it's better to treat the N_ as a string delimiter rather than a function name."));
    }
}
