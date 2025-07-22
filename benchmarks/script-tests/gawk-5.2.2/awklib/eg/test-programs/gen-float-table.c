#include <stdio.h>
#include <math.h>
#include <stdbool.h>

#define def_func(name, op) \
    bool name(double left, double right) { \
        return left op right; \
    }

def_func(eq, ==)
def_func(ne, !=)
def_func(lt, <)
def_func(le, <=)
def_func(gt, >)
def_func(ge, >=)

struct {
    const char *name;
    bool (*func)(double left, double right);
} functions[] = {
    { "==", eq },
    { "!=", ne },
    { "< ", lt },
    { "<=", le },
    { "> ", gt },
    { ">=", ge },
    { 0, 0 }
};

int main()
{
    double values[] = {
        -sqrt(-1),     // nan
        sqrt(-1),      // -nan
        -log(0.0),     // inf
        log(0.0)       // -inf
    };
    double compare[] = { 2.0,
        -sqrt(-1),     // nan
        sqrt(-1),      // -nan
        -log(0.0),     // inf
        log(0.0)       // -inf
    };

    int i, j, k;

    for (i = 0; i < 4; i++) {
        for (j = 0; j < 5; j++) {
            for (k = 0; functions[k].name != NULL; k++) {
                printf("%g %s %g -> %s\n", values[i],
                                functions[k].name,
                                compare[j],
                    functions[k].func(values[i], compare[j]) ? "True" : "False");
            }
            printf("\n");
        }
    }

    return 0;
}
