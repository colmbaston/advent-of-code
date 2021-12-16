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

  diff -w -u --color answers.txt <(for PUZZLE in $(ls | grep "^puzzle")
  do
    (echo "$PUZZLE:"; target/release/$PUZZLE | sed 's/^/  /') | tee /dev/tty
  done)
}

if [ $1 == "all" ]
then
  for YEAR in $(ls | grep "20")
  do
    (check $YEAR)
  done
  exit
else
  check $1
fi
