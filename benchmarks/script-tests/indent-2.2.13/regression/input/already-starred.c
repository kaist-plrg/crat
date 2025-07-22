 
/*
 * Instance XV_KEY_DATA key.  An instance is a set of related
 * user interface objects.  A pointer to an object's instance
 * is stored under this key in every object.  This must be a
 * global variable.
 */

    /*
     * Another comment.
     */
 
main(int argc, char **argv)
{
        /*
         * Initialize XView.
         */
        xv_init(XV_INIT_ARGC_PTR_ARGV, &argc, argv, NULL);
        INSTANCE = xv_unique_key();
}
