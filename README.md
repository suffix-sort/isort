isort
=====

`isort` is a CLI sort utility for inverse lexicographic (suffix) sorting.

An inverse/suffix sort order looks at the last character first, and
works backwards towards the first.

(While "inverse sort" is a common term in Computer Science, "suffix sort"
is more familiar to linguists.)

Basic behavior
--------------
The CLI tool feels very much like the standard `sort` utility, with the
only notable exception that the short option for `--ignore-case` is not
`-f` but the more intuitive `-i`.

If given an argument, `isort` will take it as a file path and spit out
the default result.

	$ isort tests/test1.txt
	a
	aa
	ba
	za
	b
	ab
	ac
	bc
	z
	az
	bz
	zz

In order to make the comparisons easier on the eye, use the `-a` option,
which right-aligns the results:

	$ isort -a tests/test1.txt
	 a
	aa
	ba
	za
	 b
	ab
	ac
	bc
	 z
	az
	bz
	zz

By default, `isort` uses the first word on the line for sorting and ignores
the rest of the line:

	$ cat tests/test2.txt
	a zzz
	aa bbb
	ab xxx
	b ignored
	za ---

	$ isort -a tests/test2.txt
	 a zzz
	aa bbb
	za ---
	 b ignored
	ab xxx

With the `-l`/`--line` option, the text is sorted using entire lines:

	$ isort -al tests/test2.txt
	   za ---
	   aa bbb
	b ignored
	   ab xxx
		a zzz
