Test the false command.

  $ BIN="$BIN_DIR/false"

Exit code is 1 with no args:

  $ $BIN; echo $?
  1

Exit code is 1 with arbitrary args:

  $ $BIN --help; echo $?
  1

  $ $BIN foo bar baz; echo $?
  1

  $ $BIN -n; echo $?
  1

Produces no stdout:

  $ $BIN | wc -c
  0
