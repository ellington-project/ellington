#!/bin/bash

directory=$1
library=${2:-"newlibrary.json"}

echo "Reading from directory: $directory" 
echo "Writing to library: $library"

case "$(uname -s)" in
    Darwin)
        export PATH=$PATH:/usr/local/bin/:/Users/adam/projects/bellson/bin/:/Users/adam/projects/ellington/target/release/
        ;;

    Linux)
        export PATH=$PATH:/usr/local/bin/:/home/adam/personal/bellson/bin/:/home/adam/personal/ellington/target/release/
        ;;
    *)
        echo "No idea what plaform we're running on. Failing."
        exit 1
        ;;
esac


if [ ! -f $library ]; then 
    echo "Default ellington library not present - initialising it first with 'ellington init directory $library --directory $directory'"
    ellington init directory "$library" --directory "$directory"
fi

# Set the separator to a newline
IFS='
'

for filename in `ellington dump $library`; do 
    echo $filename; 
    ellington query "$filename" "$library" -f -p
done