#include <stdio.h>
int main()
{
char *format = "\
%c %c %c
%c %c %c
%c %c %c\n";

printf(format, '1', '2', '3', '4', '5', '6', '7', '8', '9');
}

static inline void __generic_memcpy_fromfs(
        void * to, 
        const void * from, 
        unsigned long n)
{
    __asm__ volatile
	("	cld
		cmpl $3,%0
		jbe 1f
		movl %%edi,%%ecx
		negl %%ecx
		andl $3,%%ecx
		subl %%ecx,%0
		fs; rep; movsb
		movl %0,%%ecx
		shrl $2,%%ecx
		fs; rep; movsl
		andl $3,%0
	1:	movl %0,%%ecx
		fs; rep; movsb");
}
static inline void __generic_memcpy_fromfs(void * to, const void * from, unsigned long n)
{
    __asm__ volatile
	("	cld
		cmpl $3,%0
		jbe 1f
		movl %%edi,%%ecx
		negl %%ecx
		andl $3,%%ecx
		subl %%ecx,%0
		fs; rep; movsb
		movl %0,%%ecx
		shrl $2,%%ecx
		fs; rep; movsl
		andl $3,%0
	1:	movl %0,%%ecx
		fs; rep; movsb"
	:"=abd" (n)
	:"0" (n),"D" ((long) to),"S" ((long) from)
	:"cx","di","si", "memory");
}


