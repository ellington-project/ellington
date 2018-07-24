#!/bin/sh
# Package up the project into a zip
# Get the crate root. 
crate=$1
# Make sure that it's been built. 
echo "Running 'cargo build --release' to generate release binaries"
pushd $crate
cargo build --release --verbose
popd

# Make a releases directory, if one doesn't exist
releases="$crate/releases"
echo "Creating release directory at '$releases'"
mkdir -p $releases

# Get some information to name the package, for now just the time and os
now=$(date "+%Y-%m-%d-%H-%M")

# Get the operating system name
if [[ $OSTYPE == darwin* ]]; then 
    echo "Running on OSX"
    osname="osx"
elif [[ $OSTYPE == linux* ]]; then
    echo "Running on Linux"
    osname="linux"
fi

# Define the package name
package="ellington-pre-alpha-$now-$osname"
echo "Defined package name: '$package'"

# Make a directory for the combination
pdir="$releases/$package"
mkdir -p "$releases/$package"

# Copy the relevant stuff into the package. 
cp $crate/target/release/ellington $pdir
cp $crate/README.md $pdir

# Zip it up! 
pushd $releases
zip -r $package $package
rm -rf $package
popd