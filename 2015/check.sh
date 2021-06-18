#!/bin/bash

cd "`dirname \"$0\"`"

TARGET_DIR=target

COLOUR1="\e[1;31m"
COLOUR2="\e[1;32m"

rm -f output.txt
cargo build --release --target-dir $TARGET_DIR

if [ $? -eq 0 ];
then
  time (for PUZZLE in $(ls | grep puzzle)
  do
    printf $COLOUR1
    $TARGET_DIR/release/$PUZZLE | tee -a output.txt
    TEMP=$COLOUR1
    COLOUR1=$COLOUR2
    COLOUR2=$TEMP
  done
  printf "\e[0m")
fi

diff -w --color answers.txt output.txt
rm -f output.txt