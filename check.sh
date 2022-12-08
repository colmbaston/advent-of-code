#!/bin/bash

if [ $# -ne 1 ]
then
  echo "error: expected exactly one argument: year, \"all\", or \"clean\""
  exit 1
fi

cd "`dirname \"$0\"`"

if [ $1 == "clean" ]
then
  for year in $(ls | grep "^20")
  do
    (cd $year && cargo clean --target-dir target)
  done
  exit
fi

check()
{
  if [ ! -d $year ]
  then
    echo "error: $year is not a directory"
    exit 1
  fi

  cd $year

  if [ ! -f "answers.txt" ]
  then
    echo "error: could not find file $year/answers.txt"
    exit 1
  fi

  cargo build --release --target-dir target

  if [ $? -ne 0 ]
  then
    exit 1
  fi

  (for puzzle in $(ls | grep "^puzzle")
  do
    echo "$puzzle:"
    target/release/$puzzle | sed '/Hello, world!/d; s/^/  /'
  done) | tee output.txt

  diff -Z -u --color answers.txt output.txt || (echo "error: there were differences from $year/answers.txt" && exit 1)
}

if [ $1 == "all" ]
then
  for year in $(ls | grep "^20")
  do
    (check) || break
  done
else
  (year=$1 check)
fi
