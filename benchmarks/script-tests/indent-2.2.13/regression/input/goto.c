int f(void)
{
	for (i = 0; i < 10; i++) {
		if (i > 5)
			goto two;
		i++;
      two:
	}
}

int main(void)
{
	if (f())
		goto out;
      out:
	return 0;
}
