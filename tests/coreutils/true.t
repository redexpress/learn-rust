Test the true command.

  $ BIN="$BIN_DIR/true"

Exit code is 0 with no args:

  $ $BIN; echo $?
  0

Exit code is 0 with arbitrary args:

  $ $BIN --help; echo $?
  0

  $ $BIN foo bar baz; echo $?
  0

  $ $BIN -n; echo $?
  0

Produces no stdout:

  $ $BIN | wc -c
  0
