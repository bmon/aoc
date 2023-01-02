#!/bin/sh

# usage ./submit.sh --year 2022 --day 1 <answer>
SESSION=$(cat session_id)

DAY=1
LEVEL=1
YEAR=2022


while [[ $# -gt 0 ]]; do
  case $1 in
    -d|--day)
      DAY="$2"
      shift # past argument
      shift # past value
      ;;
    -y|--year)
      YEAR="$2"
      shift # past argument
      shift # past value
      ;;
    -l|--level)
      LEVEL="$2"
      shift # past argument
      shift # past value
      ;;
    -*|--*)
      echo "Unknown option $1"
      echo "Usage: ./submit.sh --year 2022 --day 1 <answer>"
      exit 1
      ;;
    *)
      ANSWER="$1" # save positional arg
      shift # past argument
      ;;
  esac
done

echo "Submitting answer for $YEAR/$DAY lv.$LEVEL: $ANSWER"

curl "https://adventofcode.com/$YEAR/day/$DAY/answer" --cookie "session=$SESSION" -d "level=$LEVEL&answer=$ANSWER" | rg article

