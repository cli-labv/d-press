#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

mkdir -p input

ARCH=$(uname -m)
case "$ARCH" in
    x86_64) BIN="bin/d-press-x86_64-unknown-linux-musl" ;;
    aarch64) BIN="bin/d-press-aarch64-unknown-linux-musl" ;;
    *) echo "Arquitectura no soportada: $ARCH" >&2; exit 1 ;;
esac

if [[ -x "$BIN" ]]; then
    exec "$BIN" "$@"
fi

if command -v cargo >/dev/null 2>&1 && command -v rustc >/dev/null 2>&1; then
    echo "Binario no encontrado. Compilando por primera vez..."
    cargo build --release --target x86_64-unknown-linux-musl
    # Copiar binario al lugar esperado
    mkdir -p bin
    cp target/x86_64-unknown-linux-musl/release/d-press "$BIN"
    chmod +x "$BIN"
    exec "$BIN" "$@"
else
    cat <<EOF
Falta toolchain Rust y no existe binario compilado.

Para usar d-press:
1) Copia este directorio a una PC con Rust y ejecuta ./start.sh
   -> Se compilará la primera vez.
2) O copia el binario ya compilado a bin/d-press-x86_64-unknown-linux-musl
EOF
    exit 1
fi
