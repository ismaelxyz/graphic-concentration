#!/usr/bin/env bash
set -euo pipefail

# Build NeHe Lesson 11 (C/GLUT) from this folder.
# Output: ./lesson11_c

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$script_dir"

if ! command -v gcc >/dev/null 2>&1; then
  echo "error: gcc not found" >&2
  exit 127
fi

# usleep() is a POSIX API and may require feature-test macros on newer glibc.
cflags="-O2 -Wall -Wextra -std=c99 -D_DEFAULT_SOURCE -D_XOPEN_SOURCE=600"
libs=""

# Prefer pkg-config when available (different distros expose different .pc names)
if command -v pkg-config >/dev/null 2>&1; then
  if pkg-config --exists freeglut; then
    libs="$(pkg-config --libs freeglut) -lGL -lGLU"
  elif pkg-config --exists glut; then
    libs="$(pkg-config --libs glut) -lGL -lGLU"
  fi
fi

# Fallback if pkg-config wasn't able to resolve GLUT
if [[ -z "$libs" ]]; then
  libs="-lglut -lGL -lGLU"
fi

# libm is required for sin/cos on many Linux distros
libs="$libs -lm"

# The original tutorial code loads the texture via a relative path:
#   Data/lesson11/tim.bmp
# Stage that layout so you can run the binary from this folder.
mkdir -p Data/lesson11
if [[ -f "tim.bmp" ]]; then
  cp -f "tim.bmp" "Data/lesson11/tim.bmp"
else
  echo "warning: tim.bmp not found next to build_c.sh" >&2
fi

out="lesson11_c"

echo "gcc $cflags -o $out lesson11.c $libs"
gcc $cflags -o "$out" lesson11.c $libs

echo "built: $script_dir/$out"
