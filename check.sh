#!/bin/bash

if [ $# -ne 1 ]
then
  echo "expected argument \$YEAR, \"all\", or \"clean\""
  exit 1
fi

cd "`dirname \"$0\"`"

if [ $1 == "clean" ]
then
  for YEAR in $(ls | grep "^20")
  do
    (cd $YEAR; cargo clean --target-dir target)
  done
  exit
fi

check()
{
  if [ ! -d $1 ]
  then
    echo "$1 is not a directory"
    exit 1
  fi

  cd $1

  if [ ! -f "answers.txt" ]
  then
    echo "could not find file answers.txt in directory $1"
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

  diff -Z -U 999 --color answers.txt output.txt
}

if [ $1 == "all" ]
then
  for year in $(ls | grep "^20")
  do
    (check $year)
  done
else
  (check $1)
fi
