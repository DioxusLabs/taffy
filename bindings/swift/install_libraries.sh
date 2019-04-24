#!/bin/sh

STRETCHKIT=../StretchKit

cd $(dirname $0)/StretchCore
cargo lipo --release
rm -rf $STRETCHKIT/Libraries $STRETCHKIT/Headers
mkdir $STRETCHKIT/Libraries $STRETCHKIT/Headers
cp target/universal/release/libstretch.a $STRETCHKIT/Libraries/libstretch.a
cbindgen . -o $STRETCHKIT/Headers/libstretch.h -l c