# demo drivers — one recipe per iTerm pane (restore your saved window
# arrangement, one pane per role). For a cross-language table, open a
# pane in the sibling ../ttt-qt and use ITS demo recipes: the wire is
# the spec.

demo_port := "4747"

# the authority
[group('demo')]
demo-server: build
    target/debug/server {{demo_port}}

# a seat — piped through cat, so the move history scrolls
[group('demo')]
demo-player: build
    target/debug/player 127.0.0.1:{{demo_port}} | cat

# a seat that plays by itself — same socket, no keyboard
[group('demo')]
demo-robot: build
    target/debug/player 127.0.0.1:{{demo_port}} robot | cat

# the wire, naked — HELLO · MOVE <n> · STATE · BYE
[group('demo')]
demo-wire:
    @echo "you are the client — type:  HELLO   then  MOVE <n> · STATE · BYE"
    nc 127.0.0.1 {{demo_port}}

# kill this repo's demo processes
[group('demo')]
demo-stop:
    -pkill -f 'target/debug/(server|player)'

[group('demo')]
wire-traffic:
    #!/usr/bin/env sh
    echo "Running as sudo, provide password when prompted"
    sudo tshark -Q -i lo0 -n -l \
        -f "tcp portrange 40000-65535" \
        -T fields \
        -E separator='|' \
        -e _ws.col.Source \
        -e tcp.srcport \
        -e _ws.col.Destination \
        -e tcp.dstport \
        -e tcp.payload \
    | while IFS='|' read -r src sport dst dport hex; do
        if [ -n "$hex" ]; then
            ascii=$(
                printf '%s' "$hex" \
                | xxd -r -p \
                | perl -pe 's/\r/\\r/g; s/\n/\\n/g; s/\t/\\t/g; s/[^\x20-\x7e]//g'
            )

            printf '%s:%s -> %s:%s | %s\n' \
                "$src" "$sport" "$dst" "$dport" "$ascii"
        fi
    done
