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
    (cd $YEAR && cargo clean)
  done
  exit
fi

check()
{
  COLOUR1="\e[1;31m"
  COLOUR2="\e[1;32m"

  rm -f output.txt
  cargo build --release --target-dir target

  if [ $? -ne 0 ];
  then
    exit 1
  fi

  time (for PUZZLE in $(ls | grep "^puzzle")
  do
    printf $COLOUR1
    target/release/$PUZZLE | tee -a output.txt
    TEMP=$COLOUR1
    COLOUR1=$COLOUR2
    COLOUR2=$TEMP
  done
  printf "\e[0m")

  diff -w --color answers.txt output.txt
  rm -f output.txt
}

if [ $1 == "all" ]
then
  for YEAR in $(ls | grep "20")
  do
    (cd $YEAR && check)
  done
  exit
fi

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

check
