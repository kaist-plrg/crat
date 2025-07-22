function eq(left, right)
{
        return left == right
}

function ne(left, right)
{
        return left != right
}

function lt(left, right)
{
        return left <  right
}

function le(left, right)
{
        return left <= right
}

function gt(left, right)
{
        return left >  right
}

function ge(left, right)
{
        return left >= right
}

BEGIN {
	nan = sqrt(-1)
	inf = -log(0)
        split("== != < <= > >=", names)
	names[3] = names[3] " "
	names[5] = names[5] " "
        split("eq ne lt le gt ge", funcs)

	compare[1] =              2.0
        compare[2] = values[1] = -sqrt(-1.0)	# nan
        compare[3] = values[2] =  sqrt(-1.0)	# -nan
        compare[4] = values[3] = -log(0.0)	# inf
        compare[5] = values[4] =  log(0.0)	# -inf

	for (i = 1; i in values; i++) {
		for (j = 1; j in compare; j++) {
			for (k = 1; k in names; k++) {
				the_func = funcs[k]
				printf("%g %s %g -> %s\n",
                                                values[i],
						names[k],
						compare[j],
					        @the_func(values[i], compare[j]) ?
                                                        "True" : "False");
			}
			printf("\n");
		}
	}
}
