#!/bin/bash
# Script de arranque: inicializa el entorno de trabajo
# Crea la carpeta input si no existe

INPUT_DIR="./input"

if [ ! -d "$INPUT_DIR" ]; then
  echo "Creando carpeta de entrada: $INPUT_DIR"
  mkdir -p "$INPUT_DIR"
else
  echo "La carpeta de entrada ya existe: $INPUT_DIR"
fi

BIN_DIR="$(dirname "$0")/bin"
ARCH="$(uname -m)"
TARGETS=(
  "$BIN_DIR/d-press-${ARCH}-unknown-linux-musl"
  "$BIN_DIR/d-press-${ARCH}-unknown-linux-gnu"
  "$BIN_DIR/d-press"
)

BIN=""
for b in "${TARGETS[@]}"; do
  if [ -f "$b" ]; then BIN="$b"; break; fi
done

if [ -n "$BIN" ]; then
  chmod +x "$BIN" 2>/dev/null
  exec "$BIN" "$@"
fi

if command -v cargo >/dev/null 2>&1; then
  echo "Compilando d-press (primera vez)..."
  cargo build --release
  exec "$(dirname "$0")/target/release/d-press" "$@"
fi

echo "No se encontró un binario ni Rust. Copia el proyecto a un entorno con cargo o coloca el binario en bin/."
exit 1
