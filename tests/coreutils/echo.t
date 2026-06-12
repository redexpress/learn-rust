Test the echo command.

  $ BIN="$BIN_DIR/echo"

Basic output with newline:

  $ $BIN hello world
  hello world

No newline with -n:

  $ $BIN -n hi
  hi (no-eol)

No args prints just a newline:

  $ $BIN | wc -c
  1

Escape enabled with -e (interprets backslash escapes):

  $ $BIN -e 'a\nb'
  a
  b

Escape explicitly disabled with -E (outputs literally):

  $ $BIN -E 'a\nb'
  a\nb

Combined -eE disables escape:

  $ $BIN -eE 'a\nb'
  a\nb

Last flag wins: -E overrides -e:

  $ $BIN -e -E 'a\nb'
  a\nb

Last flag wins: -e overrides -E:

  $ $BIN -E -e 'a\nb'
  a
  b

No trailing space on last arg:

  $ $BIN -n a b c
  a b c (no-eol)

Empty arg still prints newline:

  $ $BIN '' | wc -c
  1

Exit code is 0:

  $ $BIN >/dev/null; echo $?
  0

Unknown flag stops option parsing:

  $ $BIN -x hello world
  -x hello world
