#!/bin/sh

# puzzle-2 will be created manually from puzzle-1

PWD=pwd
[ -z ${1} ] && exit

DAYDIR="day-${1}"
mkdir -p $DAYDIR/puzzle-1
cd $DAYDIR/
echo "all:\n\nrun: all" > puzzle-1/Makefile
touch input.txt 
cd $PWD