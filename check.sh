#!/bin/bash

if [ $# -ne 1 ]
then
  echo "provide \$YEAR or \"clean\" as only argument"
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

COLOUR1="\e[1;31m"
COLOUR2="\e[1;32m"

rm -f output.txt
cargo build --release --target-dir target

if [ $? -eq 0 ];
then
  time (for PUZZLE in $(ls | grep puzzle)
  do
    printf $COLOUR1
    target/release/$PUZZLE | tee -a output.txt
    TEMP=$COLOUR1
    COLOUR1=$COLOUR2
    COLOUR2=$TEMP
  done
  printf "\e[0m")
fi

diff -w --color answers.txt output.txt
rm -f output.txt
