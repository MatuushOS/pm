#!/bin/sh
to_copy={pm,shapkg,mkrepo}
dirs={bin}
mkdir -p /mtos/$dirs
cp target/$(arch)-unknown-linux-musl/release/$to_copy /mtos/bin