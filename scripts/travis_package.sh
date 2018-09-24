#!/bin/bash
# Package up the project into a zip
# Useless to go on when an error occurs
set -o errexit
# Get the crate root. 
buildroot="${TRAVIS_BUILD_DIR}"
# Get the OS from TRAVIS
osname="${TRAVIS_OS_NAME}"
# Get the Git tag from TRAVIS
tag="${TRAVIS_TAG:-master_prerelease}"

# Make sure that it's been built. 
echo "Running 'cargo build --release' to generate release binaries"
pushd $buildroot
    cargo build --release --verbose
popd

# Make a releases directory, if one doesn't exist
releases="$buildroot/releases"
echo "Creating release directory at '$releases'"
mkdir -p $releases

# Define the package name
package="ellington-$osname-$tag"
echo "Defined package name: '$package'"

# Generate a generic package
cargo script scripts/package.rs -- $buildroot $tag
zip -r $releases/$package.zip $package
rm -rf $package


# If we're on linux, build a debian package as well
if [[ $osname == linux ]]; then
    echo "Running on Linux"
    pushd $buildroot
        cargo deb --no-build --no-strip
    popd 
    cp $buildroot/target/debian/*.deb $releases
fi
