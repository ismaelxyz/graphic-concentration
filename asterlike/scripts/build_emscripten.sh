#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

EMSDK_DIR="${EMSDK_DIR:-/home/overlord/Documents/emsdk}"
EMSDK_ENV="$EMSDK_DIR/emsdk_env.sh"

if [[ ! -f "$EMSDK_ENV" ]]; then
  echo "ERROR: no encuentro emsdk_env.sh en: $EMSDK_ENV" >&2
  echo "Tip: export EMSDK_DIR=/ruta/a/emsdk" >&2
  exit 1
fi

PROFILE="${PROFILE:-debug}"  # debug | release
CARGO_PROFILE_FLAGS=()
TARGET_SUBDIR="debug"
if [[ "$PROFILE" == "release" ]]; then
  CARGO_PROFILE_FLAGS=("--release")
  TARGET_SUBDIR="release"
fi

export EMSDK_QUIET=1
# shellcheck source=/dev/null
source "$EMSDK_ENV" >/dev/null 2>&1 || source "$EMSDK_ENV"

TARGET="wasm32-unknown-emscripten"

# Asegura que el target exista (si ya está instalado, no hace nada).
rustup target add "$TARGET" >/dev/null 2>&1 || true

# Importante: `--features emscripten` compila SDL3 (+image/+ttf) desde source
# usando el toolchain de Emscripten, resolviendo el error de -lSDL3*.
(
  cd "$ROOT_DIR"
  cargo build --target "$TARGET" "${CARGO_PROFILE_FLAGS[@]}" --features emscripten
)

OUT_DIR="$ROOT_DIR/target/$TARGET/$TARGET_SUBDIR/deps"
JS="$OUT_DIR/asterlike.js"

if [[ ! -f "$JS" ]]; then
  echo "ERROR: no se generó $JS" >&2
  echo "Buscando asterlike.js en el target..." >&2
  FOUND=$(find "$ROOT_DIR/target/$TARGET/$TARGET_SUBDIR" -maxdepth 3 -name 'asterlike.js' -print -quit || true)
  if [[ -n "${FOUND:-}" ]]; then
    echo "Encontré: $FOUND" >&2
    JS="$FOUND"
  else
    exit 1
  fi
fi

BASE="${JS%.js}"

DIST_DIR="$ROOT_DIR/dist/emscripten-$PROFILE"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Copiamos y normalizamos nombres a `asterlike.*` para simplificar el HTML.
cp -f "$JS" "$DIST_DIR/asterlike.js"

for src in "$BASE.wasm" "$BASE.data" "$BASE.worker.js" "$BASE.wasm.map"; do
  if [[ -f "$src" ]]; then
    ext="${src#$BASE}"
    cp -f "$src" "$DIST_DIR/asterlike$ext"
  fi
done

cat > "$DIST_DIR/index.html" <<'HTML'
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>asterlike (emscripten)</title>
    <style>
      html, body { margin: 0; height: 100%; background: #000; }
      canvas { display: block; width: 100%; height: 100%; }
    </style>
  </head>
  <body>
    <canvas id="canvas" oncontextmenu="event.preventDefault()"></canvas>
    <script>
      window.addEventListener('error', (e) => console.error('window.error', e.error || e.message || e));
      window.addEventListener('unhandledrejection', (e) => console.error('unhandledrejection', e.reason || e));

      var Module = {
        canvas: document.getElementById('canvas'),
        print: (...args) => console.log('[stdout]', ...args),
        printErr: (...args) => console.error('[stderr]', ...args),
        onAbort: (what) => console.error('ABORT', what),
      };
    </script>
    <script src="asterlike.js"></script>
  </body>
</html>
HTML

echo "OK: build emscripten ($PROFILE)" >&2
echo "Salida: $DIST_DIR" >&2
echo "Siguiente: ./scripts/serve_emscripten.sh" >&2
