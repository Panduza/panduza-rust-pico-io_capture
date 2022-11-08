#! /bin/bash

# Helper function
is_sourced() {
if [ -n "$ZSH_VERSION" ]; then 
    case $ZSH_EVAL_CONTEXT in *:file:*) return 0;; esac
else  # Add additional POSIX-compatible shell names here, if needed.
    case ${0##*/} in dash|-dash|bash|-bash|ksh|-ksh|sh|-sh) return 0;; esac
fi
return 1  # NOT sourced.
}

is_sourced && sourced=1 || sourced=0

if [ $sourced -eq 0 ]; then
    echo "this script needs to be sourced!"
    exit -1
fi

SCRIPTPATH="$( cd "$(dirname "$1")" >/dev/null 2>&1 ; pwd -P )"

export PATH=$PATH:$SCRIPTPATH