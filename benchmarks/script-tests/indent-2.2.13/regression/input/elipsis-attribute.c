extern void sendto_one(aClient *to, char *pattern, ...)
    __attribute__ ((format(printf, 2, 3)));
int f(char *m, ...) __attribute__ ((format(printf, 1, 2)));

void sendto_one(aClient *to, char *pattern, ...)
{
}
