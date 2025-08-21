ssort
=====

`ssort` is a CLI sort utility for inverse lexicographic (suffix) sorting.

An inverse/suffix sort order looks at the last character first, and
works backwards towards the first.

`ssort` is designed to be fast. Typically it can process 1M lines of text
in a matter of seconds. Some options may make it somewhat slower
(e.g. `--stable`).

ssort -h
--------
Usage: ssort [OPTIONS] [FILE]...

Arguments:
  [FILE]...  input files (use '-' for stdin, default if no files provided)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

Sorting Options:
  -i, --ignore-case       ignore case when sorting
  -l, --line              use entire line for sorting instead of first word
  -d, --dictionary-order  dictionary order: ignore non-alphabetic characters when finding first word
  -r, --reverse           reverse the sort order
  -s, --stable            stable sort (maintains original order of equal elements)

Output:
  -a, --right-align      right-align output by adding leading spaces
  -x, --exclude-no-word  exclude lines without words
  -w, --word-only        output only the word used for sorting (excludes the remainder of lines)

Basic behavior
--------------
The CLI tool feels very much like the standard `sort` utility, with the
only notable exception that the short option for `--ignore-case` is not
`-f` but the more intuitive `-i`.

If given an argument, `ssort` will take it as a file path:

	$ ssort tests/test1.txt
	a
	aa
	ba
	za
	b
	ab
	ac
	bc
	abc
	z
	az
	bz
	zz
	zzz


Without arguments, ssort takes input from stdin.
The above is therefore equivalent to:

	$ cat tests/test1.txt | ssort
	...

Options
-------

### `--right-align` / `-a`

In order to make the comparisons easier on the eye, use the `-a` option,
which right-aligns the results:

	$ ssort -a tests/test1.txt
	  a
	 aa
	 ba
	 za
	  b
	 ab
	 ac
	 bc
	abc
	  z
	 az
	 bz
	 zz
	zzz

By default, `ssort` uses the first word on the line for sorting and ignores
the rest of the line:

	$ cat tests/test2.txt
	a zzz
	aa bbb
	ab xxxxx
	b aaa
	za -

	$ ssort -a tests/test2.txt
	 a zzz
	aa bbb
	za -
	 b aaa
	ab xxxxx

### `--line` / `-l`

With the `-l`/`--line` option, the text is sorted using entire lines:

	$ ssort -al tests/test2.txt
		za -
	   b aaa
	  aa bbb
	ab xxxxx
	   a zzz

### `--dictionary-order` / `-d`

The `-d` option ignores any non-alphabetic characters in identifying the
first word. It has no effect in combination with the `--line` option.

### `--word-only` / `-w`

The `-w` option outputs only the word used in the sort, dropping the
remainder of lines. It has no effect in combination with `--line`.

### `--exclude-no-word` / `-x`

The `-x` option removes empty lines and lines without any alphanumeric
characters from the output. It has no effect in combination with `-l`.
