set -e
RUSTFLAGS=

DAY=$1
QUESTION=$2
INPUTFILE=./inputs/$DAY.txt

mkdir -p ./inputs/
if [ ! -f $INPUTFILE ]; then
    curl "https://adventofcode.com/2020/day/$DAY/input" -H "cookie: $(cat cookie.txt)" --compressed > $INPUTFILE
fi

cargo run -- $@
