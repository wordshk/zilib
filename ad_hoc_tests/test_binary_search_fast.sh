#!/bin/bash

set -e
set -o pipefail

GREP=rg
BSEARCH=../target/debug/binary_search_file
tmpfile=$(mktemp)
echo "[DEBUG] tmpfile: $tmpfile"

# Program does a binary search on a text file with lines as records.
# Tests

function check {
    diff <($BSEARCH $tmpfile "$1" ',') <($GREP "^$1(\$|,)" $tmpfile) # empty file
}

# Simple test
check x # empty file
check xxxxx # empty file

cat /dev/null > $tmpfile
echo "1" >> $tmpfile
check 0
check 1
check 2

echo "2" >> $tmpfile
echo "3" >> $tmpfile
check 0
check 1
check 2
check 3
check 4

cat /dev/null > $tmpfile
echo "1" >> $tmpfile
echo "22" >> $tmpfile
echo "4444" >> $tmpfile
echo "88888888" >> $tmpfile
echo "aaaaaaaaaaaaaaaa" >> $tmpfile
check 0
check 1
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b


cat /dev/null > $tmpfile
echo "1111111111111111" >> $tmpfile
echo "22" >> $tmpfile
echo "4444" >> $tmpfile
echo "88888888" >> $tmpfile
echo "aaaaaaaaaaaaaaaa" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b

echo '' > $tmpfile  # empty lines leading
echo "1111111111111111" >> $tmpfile
echo "22" >> $tmpfile
echo "4444" >> $tmpfile
echo "88888888" >> $tmpfile
echo "aaaaaaaaaaaaaaaa" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b

echo '' > $tmpfile  # empty lines leading
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo "1111111111111111" >> $tmpfile
echo "22" >> $tmpfile
echo "4444" >> $tmpfile
echo "88888888" >> $tmpfile
echo "aaaaaaaaaaaaaaaa" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b


echo '' > $tmpfile  # empty lines leading
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo "a" >> $tmpfile
echo "b" >> $tmpfile
echo "c" >> $tmpfile
check 0
check aaaaaaaaaaaaaaaa
check b
check c
check d


# Now with fields

cat /dev/null > $tmpfile
echo "1,hello" >> $tmpfile
check 0
check 1
check 2

echo "2" >> $tmpfile
echo "3,world" >> $tmpfile
check 0
check 1
check 2
check 3
check 4

cat /dev/null > $tmpfile
echo "1,a" >> $tmpfile
echo "22,bb" >> $tmpfile
echo "4444,cc" >> $tmpfile
echo "88888888,dddddddd" >> $tmpfile
echo "aaaaaaaaaaaaaaaa,xxxxxxxxxxxxxxxx" >> $tmpfile
check 0
check 1
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b


cat /dev/null > $tmpfile
echo "1111111111111111,aaaaaaaaaaaaaaaa" >> $tmpfile
echo "22,bb" >> $tmpfile
echo "4444,cccc" >> $tmpfile
echo "88888888,dddddddd" >> $tmpfile
echo "aaaaaaaaaaaaaaaa,xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b

echo '' > $tmpfile  # empty lines leading
echo "1111111111111111,aaaaaaaaaaaaaaaa" >> $tmpfile
echo "22,bb" >> $tmpfile
echo "4444,cccc" >> $tmpfile
echo "88888888,dddddddd" >> $tmpfile
echo "aaaaaaaaaaaaaaaa,xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b

echo ',' > $tmpfile  # empty lines leading
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo "1111111111111111,aaaaaaaaaaaaaaaa" >> $tmpfile
echo "22,bb" >> $tmpfile
echo "4444,cccc" >> $tmpfile
echo "88888888,dddddddd" >> $tmpfile
echo "aaaaaaaaaaaaaaaa,xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b


echo '' > $tmpfile  # empty lines leading
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ",a," >> $tmpfile
echo "b" >> $tmpfile
echo "c" >> $tmpfile
check 0
check aaaaaaaaaaaaaaaa
check b
check c
check d

cat /dev/null > $tmpfile
echo "1,,,,,," >> $tmpfile
check 0
check 1
check 2

echo "2" >> $tmpfile
echo "3,,,,,," >> $tmpfile
check 0
check 1
check 2
check 3
check 4

cat /dev/null > $tmpfile
echo "1,," >> $tmpfile
echo "22,,," >> $tmpfile
echo "4444,,," >> $tmpfile
echo "88888888,,,,,,,,," >> $tmpfile
echo "aaaaaaaaaaaaaaaa,,,x,,,,,,,,,,x,," >> $tmpfile
check 0
check 1
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b


cat /dev/null > $tmpfile
echo "1111111111111111,a,,a,,,,,,,,,,,a" >> $tmpfile
echo "22,b," >> $tmpfile
echo "4444,c,,," >> $tmpfile
echo "88888888,d,d,d,d," >> $tmpfile
echo "aaaaaaaaaaaaaaaa,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check aaaaaaaaaaaaaaaa
check b
check x

echo '' > $tmpfile  # empty lines leading
echo "1111111111111111,a,a,a,a,a,a,a,a," >> $tmpfile
echo "22,bb" >> $tmpfile
echo "4444,cccc" >> $tmpfile
echo "88888888,dddddddd" >> $tmpfile
echo "aaaaaaaaaaaaaaaa,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check a
check aaaaaaaaaaaaaaaa
check b
check x

echo ',' > $tmpfile  # empty lines leading
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo '' >> $tmpfile
echo "1111111111111111,a,a,a,a,a,a,a,a," >> $tmpfile
echo "22,bb" >> $tmpfile
echo "4444,cccc" >> $tmpfile
echo "88888888,dddddddd" >> $tmpfile
echo "aaaaaaaaaaaaaaaa,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x" >> $tmpfile
check 0
check 1
check 1111111111111111
check 2
check 22
check 222
check 4
check 44
check 4444
check 444444
check 88888888
check a
check aaaaaaaaaaaaaaaa
check b
check x


echo '' > $tmpfile  # empty lines leading
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo '' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ',' >> $tmpfile
echo ",a," >> $tmpfile
echo "b" >> $tmpfile
echo "c" >> $tmpfile
check 0
check aaaaaaaaaaaaaaaa
check b
check c
check d
rm -f $tmpfile
echo "[DEBUG] Tests in $0 pass"
