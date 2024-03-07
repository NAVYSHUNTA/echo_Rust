#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "hello there" > $OUTDIR/hello1.txt
echo "hello"  "there" > $OUTDIR/hello2.txt
echo -n "hello  there" > $OUTDIR/hello1.n.txt
echo -n "hello"  "there" > $OUTDIR/hello2.n.txt