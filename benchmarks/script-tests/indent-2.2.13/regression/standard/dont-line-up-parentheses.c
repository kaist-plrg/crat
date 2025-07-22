void
t ()
{
    if ((really_long_variable_name_like_this =
	    some_funct_group_create (mixer, SND_MIXER_GRP_IGAIN, igain_group_control,
		parameter5, parameter6, parameter7)) == NULL)
	error++;
}
