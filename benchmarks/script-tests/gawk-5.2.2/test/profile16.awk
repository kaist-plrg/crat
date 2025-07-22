BEGIN {
	foo::foo_bar()
	foofoo::xxx()
}

@namespace "foo"

function foo_bar()
{
	print "foo::foo_bar"
}

function foofoo::xxx()
{
	print "foofoo::xxx"
}
