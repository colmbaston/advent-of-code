#!/bin/bash

if [[ ! -v AOC_SESSION ]]
then
  echo "AOC_SESSION variable is unset"
  exit 1
fi

export TZ="America/New_York"
repo="/home/colm/git/advent-of-code"
regex="^$repo/(20[0-9][0-9])/puzzle([0-9][0-9])"

if [[ $PWD =~ $regex ]]
then
  year=${BASH_REMATCH[1]}
  month=12
  day=${BASH_REMATCH[2]}
else
  year=$(date +%Y)
  month=$(date +%m)
  day=$(date +%d)
fi

date="$year/$month/$day"

if (( $year < 2015 || $month != 12 || 10#$day < 1 || 10#$day > 25 ))
then
  echo "invalid puzzle $date"
  exit 1
fi

if (( $(date +%s) < $(date -d $date +%s) ))
then
  echo "puzzle $date not yet released"
  exit 1
fi

path="$repo/$year/puzzle$day/input.txt"

if [[ ! -f $path ]]
then
  if [[ ! -w $(dirname $path) ]]
  then
    echo "cannot write to $path"
    exit 1
  fi

  echo "attempting to download $path"
  url="https://adventofcode.com/$year/day/$((10#$day))/input"
  curl $url --cookie "session=$AOC_SESSION" -o $path
fi

if [[ -v EDITOR ]]
then
  exec $EDITOR $path
fi
