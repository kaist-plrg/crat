void
foo ()
{
  if (mask && ((mask[0] == '\0') ||
	       (mask[1] == '\0'
		&& ((mask[0] == '0')
		    || (mask[0] ==
			'*')))))
    mask = NULL;
  if ((acptr = FindUser (nick)) &&
      ((!(bitsel & WHOSELECT_OPER))
       || IsAnOper (acptr))
      && Process (acptr)
      && SHOW_MORE (sptr, counter))
    mask = NULL;
}
