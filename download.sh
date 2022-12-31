#!/bin/sh
SESSION=$(cat session_id)

YEAR=2022
mkdir $YEAR
cd $YEAR

for DAY in {1..25}
do
    printf -v PDAY "%02d" $DAY
    mkdir $PDAY
    cd $PDAY
    curl "https://adventofcode.com/$YEAR/day/$DAY" --cookie "session=$SESSION" -o "$PDAY.html"
    curl "https://adventofcode.com/$YEAR/day/$DAY/input" --cookie "session=$SESSION" -o "input.html"
    cd ..
done
