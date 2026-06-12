Test the pwd command.

  $ BIN="$BIN_DIR/pwd"

Default prints current directory:

  $ $BIN
  * (glob)

Exit code is 0:

  $ $BIN >/dev/null; echo $?
  0

Explicit -L behaves like default:

  $ $BIN -L
  * (glob)

  $ $BIN --logical
  * (glob)

-P resolves symlinks:

  $ $BIN -P
  * (glob)

  $ $BIN --physical
  * (glob)

Unknown option fails with exit code 1:

  $ $BIN -X; echo $?
  pwd: invalid option '-X'
  1

  $ $BIN --bogus; echo $?
  pwd: invalid option '--bogus'
  1

Output ends with newline:

  $ $BIN | wc -c
  * (glob)
