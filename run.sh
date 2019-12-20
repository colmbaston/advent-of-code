#!/bin/bash

cd "`dirname \"$0\"`"

COLOUR1="\e[1;31m"
COLOUR2="\e[1;32m"

cargo build --release

if [ $? -eq 0 ];
then
  time (for puzzle in $(ls | grep puzzle)
  do
    printf $COLOUR1
    ./target/release/$puzzle
    TEMP=$COLOUR1
    COLOUR1=$COLOUR2
    COLOUR2=$TEMP
  done
  printf "\e[0m")
fi

cargo clean
