Test the cat command.

  $ BIN="$BIN_DIR/cat"

v1: Basic file output:

  $ echo "hello" > $CRAMTMP/hello.txt
  $ $BIN $CRAMTMP/hello.txt
  hello

v1: Multiple files:

  $ echo "a" > $CRAMTMP/a.txt
  $ echo "b" > $CRAMTMP/b.txt
  $ $BIN $CRAMTMP/a.txt $CRAMTMP/b.txt
  a
  b

v1: Read from stdin when no args:

  $ echo "stdin" | $BIN
  stdin

v1: Read from stdin with -:

  $ echo "stdin" | $BIN -
  stdin

v1: File not found error to stderr, exit code 1:

  $ $BIN nonexistent_file 2>&1; echo "exit=$?"
  cat: nonexistent_file: * (glob)
  exit=1

v1: Mixed valid and invalid files:

  $ echo "ok" > $CRAMTMP/ok.txt
  $ $BIN $CRAMTMP/ok.txt nonexistent_file 2>&1; echo "exit=$?"
  ok
  cat: nonexistent_file: * (glob)
  exit=1

v2: -n numbers all lines:

  $ printf "a\n\nb\n" > $CRAMTMP/nums.txt
  $ $BIN -n $CRAMTMP/nums.txt
       1\ta (esc)
       2\t (esc)
       3\tb (esc)

v2: -b numbers only non-blank lines:

  $ $BIN -b $CRAMTMP/nums.txt
       1\ta (esc)
  \s* (re)
       2\tb (esc)

v2: -b overrides -n:

  $ $BIN -nb $CRAMTMP/nums.txt
       1\ta (esc)
  \s* (re)
       2\tb (esc)

v3: -s squeezes consecutive blank lines:

  $ printf "a\n\n\nb\n" > $CRAMTMP/squeeze.txt
  $ $BIN -s $CRAMTMP/squeeze.txt
  a
  
  b

v3: -s with -n:

  $ $BIN -ns $CRAMTMP/squeeze.txt
       1\ta (esc)
       2\t (esc)
       3\tb (esc)

v4: -E shows line ends with $:

  $ echo "hello" > $CRAMTMP/ends.txt
  $ $BIN -E $CRAMTMP/ends.txt
  hello$

v4: -T shows tabs as ^I:

  $ printf "a\tb\n" > $CRAMTMP/tabs.txt
  $ $BIN -T $CRAMTMP/tabs.txt
  a^Ib

v4: -v shows non-printing characters:

  $ printf "\x01\x02\n" > $CRAMTMP/ctrl.txt
  $ $BIN -v $CRAMTMP/ctrl.txt
  \x01\x02

v4: -A equals -vET:

  $ printf "a\tb\n" > $CRAMTMP/all.txt
  $ $BIN -A $CRAMTMP/all.txt
  a^Ib$

v4: -e equals -vE:

  $ $BIN -e $CRAMTMP/all.txt
  a\tb$ (esc)

v4: -t equals -vT:

  $ $BIN -t $CRAMTMP/all.txt
  a^Ib

v4: Combined -b -E -s:

  $ printf "a\n\n\nb\n" > $CRAMTMP/combo.txt
  $ $BIN -b -E -s $CRAMTMP/combo.txt
       1\ta$ (esc)
  $
       2\tb$ (esc)

Exit code 0 on success:

  $ echo "ok" > $CRAMTMP/exit.txt
  $ $BIN $CRAMTMP/exit.txt >/dev/null; echo $?
  0

Invalid option:

  $ $BIN -x 2>&1; echo "exit=$?"
  cat: invalid option -- 'x'
  exit=1

-- stops option parsing:

  $ $BIN -- -n 2>&1; echo $?
  cat: -n: * (glob)
  1
