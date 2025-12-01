#!/usr/bin/bash -eE

year="$1"
day="$2"

if [ -z "$year" ] || [ -z "$day" ]; then
    echo "run.sh <year> <day>"
fi

cd "$year/$day"
dune exec --display quiet --no-print-directory "$day"
