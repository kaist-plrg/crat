void foo()
{
  if ((
#if 1
	/*
	 * REMOVE THIS after all servers upgraded to 2.10.01 and
	 * Uworld uses a numeric too
	 */
	 (strlen(parv[1]) != 1 && !(acptr = FindClient(parv[1]))))
       || (strlen(parv[1]) == 1 &&
#endif
	 1))
  {
  }
  if ((!IsServer(cptr)) && isNickJuped(nick))
  {
    sendto_one(sptr, err_str(ERR_NICKNAMEINUSE), me.name,
        /* parv[0] is empty when connecting */
	BadPtr(parv[0]) ? "*" : parv[0], nick);
    return 0;                   /* NICK message ignored */
  }
}
