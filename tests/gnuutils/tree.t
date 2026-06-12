Test the tree command.

  $ BIN="$BIN_DIR/tree"

Help output:

  $ $BIN -h
  Usage: tree [options] [directory]
  \s* (re)
  Options:
    -a            show hidden files
    -d            list directories only
    -L LEVEL      max display depth
    -h, --help    this help

  $ $BIN --help
  Usage: tree [options] [directory]
  \s* (re)
  Options:
    -a            show hidden files
    -d            list directories only
    -L LEVEL      max display depth
    -h, --help    this help

Basic tree of a directory:

  $ mkdir -p $CRAMTMP/tree_test/sub
  $ touch $CRAMTMP/tree_test/a.txt
  $ touch $CRAMTMP/tree_test/b.txt
  $ touch $CRAMTMP/tree_test/sub/c.txt
  $ $BIN $CRAMTMP/tree_test
  \xe2\x94\x9c\xe2\x94\x80\xe2\x94\x80 a.txt (esc)
  \xe2\x94\x9c\xe2\x94\x80\xe2\x94\x80 b.txt (esc)
  \xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 sub (esc)
      \xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 c.txt (esc)

Directories only (-d):

  $ $BIN -d $CRAMTMP/tree_test
  \xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 sub (esc)

Show hidden files (-a):

  $ touch $CRAMTMP/tree_test/.hidden
  $ $BIN $CRAMTMP/tree_test
  \xe2\x94\x9c\xe2\x94\x80\xe2\x94\x80 a.txt (esc)
  \xe2\x94\x9c\xe2\x94\x80\xe2\x94\x80 b.txt (esc)
  \xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 sub (esc)
      \xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 c.txt (esc)

  $ $BIN -a $CRAMTMP/tree_test
  \xe2\x94\x9c\xe2\x94\x80\xe2\x94\x80 a.txt (esc)
  \xe2\x94\x9c\xe2\x94\x80\xe2\x94\x80 b.txt (esc)
  \xe2\x94\x9c\xe2\x94\x80\xe2\x94\x80 sub (esc)
  \xe2\x94\x82   \xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 c.txt (esc)
  \xe2\x94\x94\xe2\x94\x80\xe2\x94\x80 .hidden (esc)

Invalid level:

  $ $BIN -L abc 2>&1; echo "exit=$?"
  tree: invalid level 'abc'
  exit=2

Level requires argument:

  $ $BIN -L 2>&1; echo "exit=$?"
  tree: option '-L' requires an argument
  exit=2

Non-existent directory:

  $ $BIN nonexistent 2>&1
  nonexistent
