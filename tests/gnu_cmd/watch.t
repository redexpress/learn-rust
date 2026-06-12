Test the watch command.

  $ BIN="$BIN_DIR/watch"

Help output:

  $ $BIN -h
  Usage: watch [options] command
  \s* (re)
  Options:
    -n, --interval SECS     seconds between updates (default 2)
    -d, --differences       highlight changed lines
    -c, --color             interpret ANSI color sequences
    -t, --no-title          suppress the header
    -b, --beep              beep if command has a non-zero exit
    -e, --errexit           exit if command has a non-zero exit
    -g, --chgexit           exit when output changes
    -h, --help              this help

  $ $BIN --help
  Usage: watch [options] command
  \s* (re)
  Options:
    -n, --interval SECS     seconds between updates (default 2)
    -d, --differences       highlight changed lines
    -c, --color             interpret ANSI color sequences
    -t, --no-title          suppress the header
    -b, --beep              beep if command has a non-zero exit
    -e, --errexit           exit if command has a non-zero exit
    -g, --chgexit           exit when output changes
    -h, --help              this help

No command given:

  $ $BIN 2>&1; echo "exit=$?"
  watch: no command given
  exit=2

Invalid interval:

  $ $BIN -n abc echo hi 2>&1; echo "exit=$?"
  watch: invalid interval 'abc'
  exit=2

  $ $BIN --interval=0 echo hi 2>&1; echo "exit=$?"
  watch: invalid interval '0'
  exit=2

  $ $BIN -n 0 echo hi 2>&1; echo "exit=$?"
  watch: invalid interval '0'
  exit=2

Interval requires argument:

  $ $BIN -n 2>&1; echo "exit=$?"
  watch: option '-n' requires an argument
  exit=2

  $ $BIN --interval 2>&1; echo "exit=$?"
  watch: option '--interval' requires an argument
  exit=2

--interval=X with -t runs command:

  $ $BIN --interval=0.5 -t echo ok 2>&1 | head -n 1
  \x1b[2J\x1b[Hok (esc)

Basic execution with -t:

  $ timeout 1 $BIN -n 0.1 -t echo hello 2>&1 | head -n 1
  \x1b[2J\x1b[Hhello (esc)

Errexit with failing command:

  $ $BIN -n 0.1 -t -e false 2>&1; echo "exit=$?"
  \x1b[2J\x1b[H (esc)
  [exit 1] (glob)
  exit=1
