#!/bin/sh


UNAME=$(uname)

if [ "$UNAME" == "Linux" ] ; then
	echo "Linux"
elif [ "$UNAME" == "Darwin" ] ; then
	echo "Darwin"
elif [[ "$UNAME" == CYGWIN* || "$UNAME" == MINGW* ]] ; then
	echo "Windows"
fi
