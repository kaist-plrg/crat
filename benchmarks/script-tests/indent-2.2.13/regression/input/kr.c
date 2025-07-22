#include <stdio.h>

struct
{
  int x;
  int y;
} z;

main()
{
    struct {int x;int y;} z;
    z.x=1;
    z.y=2;
}
