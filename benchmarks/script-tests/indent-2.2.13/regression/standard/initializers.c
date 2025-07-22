void
foo ()
{
  char c[8] = {
    0, 0, 0, 0,
    1, 1, 1, 1
  };
  char c[8] = {			/* comment */
    0, 0, 0, 0,
    1, 1, 1, 1
  };
  char c[8] = {			/* mult-line
				   comment */
    0, 0, 0, 0,
    1, 1, 1, 1
  };

  static unsigned int current_flags[4] = { 0, 0, 1, 2 };
  static unsigned int current_flags[4] =
       { 0, CHFL_CHANOP | CHFL_VOICE, CHFL_VOICE, CHFL_CHANOP };
}
