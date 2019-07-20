#!/bin/sh
YEAR=2015
mkdir $YEAR
cd $YEAR

for DAY in {1..25}
do
    mkdir $DAY
    cd $DAY
    curl "https://adventofcode.com/$YEAR/day/$DAY" -o "$DAY.html"
    cd ..
done
