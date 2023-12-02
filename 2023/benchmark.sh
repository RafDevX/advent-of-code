#!/bin/sh

set -e

test -d $1 || (echo "Pass day directory as argument" && exit 1)

cd $1

[[ $(grep -o -- '-- example <-' ./part*.hs | wc -l) = 2 ]] || (echo "Comment out example part" && exit 2)

ghc --make ./part1
ghc --make ./part2

hyperfine "./part1 && ./part2" --warmup 250 --shell "/bin/sh"
