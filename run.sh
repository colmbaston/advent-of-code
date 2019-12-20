#!/bin/bash

cd "`dirname \"$0\"`"

cargo build --release

COLOUR1="\e[1;31m"
COLOUR2="\e[1;32m"

time if [ $? -eq 0 ];
then
  for puzzle in $(ls | grep puzzle)
  do
    printf $COLOUR1
    ./target/release/$puzzle
    TEMP=$COLOUR1
    COLOUR1=$COLOUR2
    COLOUR2=$TEMP
  done
  printf "\e[0m"
fi
