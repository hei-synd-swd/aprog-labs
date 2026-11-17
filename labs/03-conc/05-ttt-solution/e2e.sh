#!/bin/sh
# E2E smoke test for the ttt workspace: two executables — server (the
# authority) and player (a keyboard).
#
# Covers: (a) scripted 2-player win, (b) addressing + refusals + peer-left,
# (d) a netcat probe of the server (SEAT, explicit REFUSED, STATE resync),
# (e) single-player tolerance in a RAW pipe, (f) no leftover processes.
#
# Pacing note: a SINGLE client needs no pacing at all — its requests
# serialize through its socket and the authority answers in order (case e is
# a raw pipe on purpose). TWO scripted keyboards are two independent
# processes: the sleeps in (a)/(b) simulate each human waiting for their
# prompt, they mask nothing in the code under test.
#
# Usage: sh e2e.sh   (from the ttt workspace root)

# A fresh port block per run (PID-derived): back-to-back runs never collide
# on a socket still in TIME_WAIT.
BASE=$((20000 + $$ % 30000))
S=target/debug/server
C=target/debug/player
OUT=/tmp/ttt-e2e
mkdir -p $OUT
rm -f $OUT/*.log

# Self-contained: build the workspace (harmless if already built).
cargo build -q || { echo "build failed"; exit 1; }

# ---- process bookkeeping -------------------------------------------------
PIDS=""
track() { PIDS="$PIDS $1"; }
cleanup() { for p in $PIDS; do kill "$p" 2>/dev/null || true; done; }
trap cleanup EXIT INT TERM

# wait_line <logfile> <pattern> — block until <pattern> shows up (~5s cap).
wait_line() {
  i=0
  while [ $i -lt 50 ]; do
    grep -q "$2" "$1" 2>/dev/null && return 0
    sleep 0.1
    i=$((i + 1))
  done
  return 1
}

# wait_pid <pid> <seconds> — wait for a child, but kill it past the cap.
# The watchdog subshell clears inherited traps so tearing it down is quiet.
wait_pid() {
  ( trap - TERM INT EXIT; sleep "$2"; kill -TERM "$1" 2>/dev/null ) & wd=$!
  wait "$1"; code=$?
  kill "$wd" 2>/dev/null || true
  wait "$wd" 2>/dev/null || true
  return $code
}

# ---- assertions ----------------------------------------------------------
ok=0; ko=0
check() { # check <file> <must|mustnot> <pattern> <label>
  if [ "$2" = must ]; then
    if grep -qE "$3" "$OUT/$1"; then echo "OK   $4"; ok=$((ok+1)); else echo "FAIL $4"; ko=$((ko+1)); fi
  else
    if grep -qE "$3" "$OUT/$1"; then echo "FAIL $4"; ko=$((ko+1)); else echo "OK   $4"; ok=$((ok+1)); fi
  fi
}
count_is() { # count_is <file> <pattern> <n> <label>
  c=$(grep -cE "$2" "$OUT/$1" 2>/dev/null || true)
  if [ "$c" = "$3" ]; then echo "OK   $4 (x$c)"; ok=$((ok+1)); else echo "FAIL $4 (got $c, want $3)"; ko=$((ko+1)); fi
}
exit0() { # exit0 <code> <label>
  if [ "$1" = 0 ]; then echo "OK   $2 (exit 0)"; ok=$((ok+1)); else echo "FAIL $2 (exit $1)"; ko=$((ko+1)); fi
}

echo "=== running scenarios ==="

# (a) scripted 2-client win: X takes 1-2-3 while O answers 4 then 5.
# O is launched only after X holds seat X (readiness gate, not a sleep);
# the interleaved sleeps then keep each keyboard behind its prompt.
PA=$((BASE + 1))
$S $PA > $OUT/a_server.log 2>&1 & AS=$!; track $AS
wait_line $OUT/a_server.log "listening on"
{ printf '1\n'; sleep 1.0; printf '2\n'; sleep 1.0; printf '3\n'; sleep 0.4; } \
  | $C 127.0.0.1:$PA > $OUT/a_x.log 2>&1 & AX=$!; track $AX
wait_line $OUT/a_x.log "you are X"
{ sleep 0.5; printf '4\n'; sleep 1.0; printf '5\n'; sleep 1.0; } \
  | $C 127.0.0.1:$PA > $OUT/a_o.log 2>&1 & AO=$!; track $AO
wait_pid $AX 10; ax=$?
wait_pid $AO 10; ao=$?
wait_pid $AS 5;  as=$?
check a_x.log must "you are X\." "(a) first client is seated X"
check a_o.log must "you are O\." "(a) second client is seated O"
check a_x.log must "X wins on 1-2-3!" "(a) X sees the verdict"
check a_o.log must "X wins on 1-2-3!" "(a) O sees the same verdict"
exit0 $ax "(a) client X"; exit0 $ao "(a) client O"; exit0 $as "(a) server exits after the game"

# (b) addressing + refusals: O misbehaves and is the ONLY one to hear
# about it; X quits, and only O reports "peer left." O's burst is a RAW
# pipe on purpose: one socket = FIFO = deterministic ('5' taken -> '9'
# accepted -> '8' out of turn). O is launched only once X's move is
# rendered, so the board O misplays against is already settled.
PB=$((BASE + 2))
$S $PB > $OUT/b_server.log 2>&1 & BS=$!; track $BS
wait_line $OUT/b_server.log "listening on"
{ printf '5\n'; sleep 2.0; printf 'q\n'; sleep 0.3; } \
  | $C 127.0.0.1:$PB > $OUT/b_x.log 2>&1 & BX=$!; track $BX
wait_line $OUT/b_x.log "4 | X | 6"
{ printf '5\n9\n8\n'; sleep 3.0; } \
  | $C 127.0.0.1:$PB > $OUT/b_o.log 2>&1 & BO=$!; track $BO
wait_pid $BX 10; bx=$?
wait_pid $BO 10; bo=$?
wait_pid $BS 5
check b_o.log must "that cell is already taken" "(b) O replays X's cell: refused, to O"
check b_o.log must "\(not your turn\)" "(b) O plays twice in a row: refused, to O"
check b_o.log must "7 \| 8 \| O" "(b) O's legal 9 landed in between"
check b_x.log mustnot "not your turn|already taken" "(b) X never hears O's refusals (addressed)"
check b_x.log mustnot "peer left" "(b) the quitter does not report a peer leaving"
check b_o.log must "peer left\." "(b) the peer DOES see the departure"
exit0 $bx "(b) client X"; exit0 $bo "(b) client O"

# (d) netcat probe of the server: the dialect, observed raw. The probe is
# seated X, so its first MOVE is accepted (broadcast board) and its second
# is answered REFUSED turn; STATE returns the current board again.
PD=$((BASE + 4))
$S $PD > $OUT/d_server.log 2>&1 & DS=$!; track $DS
wait_line $OUT/d_server.log "listening on"
{ printf 'GARBAGE\n'; sleep 0.3; printf 'HELLO\n'; sleep 0.3; printf 'MOVE 5\n'; sleep 0.3; \
  printf 'MOVE 5\n'; sleep 0.3; printf 'STATE\n'; sleep 0.5; } \
  | nc 127.0.0.1 $PD > $OUT/d_probe.log 2>&1 & NC=$!; track $NC
sleep 2.5
kill $NC 2>/dev/null || true
wait_pid $DS 5
count_is d_probe.log "SEAT X" 1              "(d) probe: HELLO is answered with the seat"
count_is d_probe.log "STATE 123456789" 1     "(d) probe: ...then the fresh board"
count_is d_probe.log "STATE 1234X6789" 2     "(d) probe: accepted move + resync, nothing else"
count_is d_probe.log "REFUSED turn" 1        "(d) probe: out-of-turn MOVE refused explicitly"

# (e) single-client tolerance, RAW pipe (no pacing anywhere): a non-number
# gets a LOCAL hint; '99' decodes fine but the AUTHORITY refuses it; '5' is
# accepted twice-refused once; quitting is silent about peers.
PE=$((BASE + 5))
$S $PE > $OUT/e_server.log 2>&1 & ES=$!; track $ES
wait_line $OUT/e_server.log "listening on"
printf 'garbage\n99\n5\n5\nq\n' | $C 127.0.0.1:$PE > $OUT/e_client.log 2>&1
ee=$?
wait_pid $ES 5
check e_client.log must "type a cell number 1-9"      "(e) garbage line prints the local hint"
check e_client.log must "pick a cell between 1 and 9" "(e) '99' comes back as an authority refusal"
check e_client.log must "4 \| X \| 6"                 "(e) '5' is accepted and rendered"
check e_client.log must "\(not your turn\)"           "(e) replaying while it is O's turn is refused"
check e_client.log mustnot "peer left"                "(e) quitting alone says nothing about a peer"
exit0 $ee "(e) raw-piped session"

# (f) no leftover processes.
cleanup
sleep 0.5
# scoped to THIS run's ports: a live demo session elsewhere must not trip it
left=$(pgrep -f "target/debug/(server|player).*($PA|$PB|$PD|$PE)$" 2>/dev/null | wc -l | tr -d ' ')
if [ "$left" = 0 ]; then echo "OK   (f) no leftover processes"; ok=$((ok+1)); else echo "FAIL (f) $left leftover process(es)"; ko=$((ko+1)); fi

echo "=== result: $ok OK, $ko FAIL ==="
[ $ko -eq 0 ]
