#!/bin/bash

GREP=rg
BSEARCH=../target/debug/binary_search_file
DATA_FILE=../lists/Unihan_Readings.txt
tmpfile=$(mktemp)

LC_ALL=C sort $DATA_FILE > "$tmpfile"
echo $tmpfile

for ((i=13310;i<=204884+2;i++)); do
    i_hex=$(printf "%04X" $i)
    echo "Testing $i_hex..."
    diff <($BSEARCH $tmpfile "U+$i_hex" '	' ) <($GREP "^U\\+$i_hex	" $tmpfile)
    if [ $? -ne 0 ]; then
        echo "Test failed for $i_hex"
        exit 1
    fi

done
