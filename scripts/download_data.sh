#!/usr/bin/env sh

CWD=$(basename "$(pwd)")

WARN=$(cat << _EOF_
Please execute this at the root of this git repository:
Usage:
  ./scripts/fetch_data.sh
_EOF_
)

if [ "$CWD" = "scripts" ]
then
  echo "$WARN"
  exit 1
fi

git submodule add git@github.com:mbuffa/hellorust-data.git data
mkdir -p target/debug target/release
ln -s ../../data target/debug/data
ln -s ../../data target/release/data
git submodule init
cd data || edit
git pull
