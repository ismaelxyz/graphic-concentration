#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
PROFILE="${PROFILE:-debug}"  # debug | release
PORT="${PORT:-8000}"

DIST_DIR="$ROOT_DIR/dist/emscripten-$PROFILE"
if [[ ! -d "$DIST_DIR" ]]; then
  echo "ERROR: no existe $DIST_DIR" >&2
  echo "Primero corré: ./scripts/build_emscripten.sh" >&2
  exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "ERROR: no encuentro python3 en PATH" >&2
  exit 1
fi

cd "$DIST_DIR"
echo "Sirviendo $DIST_DIR en http://localhost:$PORT/" >&2
echo "(Corta con Ctrl+C)" >&2
python3 -m http.server "$PORT"
