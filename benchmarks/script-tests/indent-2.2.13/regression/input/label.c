function()
{
    if (do_stuff1() == ERROR)
        goto cleanup1;

    if (do_stuff2() == ERROR)
        goto cleanup2;

    return SUCCESS;

  cleanup2:
    do_cleanup2();

  cleanup1:
    do_cleanup1();

    return ERROR;
}
