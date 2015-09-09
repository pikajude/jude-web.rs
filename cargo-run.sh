#!/bin/sh

k () {
  killall jude-web 2>/dev/null
  killall rustc 2>/dev/null
}

trap k SIGINT SIGTERM

if [ "$1" = "child" ]; then
  k
  cargo run &
else
  fswatch -e '.*' -i '\.rs$' -0 . | xargs -0 -n 1 -I {} "$0" child
fi
