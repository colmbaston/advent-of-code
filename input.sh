#!/bin/bash

if [[ ! -v AOC_SESSION ]]
then
  echo "AOC_SESSION variable is unset"
  exit 1
fi

export TZ="America/New_York"
year=$(date +%Y)
month=$(date +%m)
day=$(date +%d)

if (( 10#$month < 12 || 10#$day > 25 ))
then
  echo "no AoC event currently running"
  exit 1
fi

file="/home/colm/git/advent-of-code/$year/puzzle$day/input.txt"

if [[ ! -f $file ]]
then
  if [[ ! -w $(dirname $file) ]]
  then
    echo "cannot write to filepath"
    exit 1
  fi

  echo "attempting to download $file"
  url="https://adventofcode.com/$year/day/$((10#$day))/input"
  curl $url --cookie "session=$AOC_SESSION" > $file
fi

if [[ -v EDITOR ]]
then
  exec $EDITOR $file
fi
