# d-press

CLI en Rust para descompresión masiva, totalmente aislada.

## Uso rápido
```bash
./start.sh --help
```

Primera ejecución compila el binario estático. Ejecuciones siguientes usan binario cacheado.

Carpetas:
- `input/` → archivos comprimidos
- `output/` → extracciones
- `logs/` y `state/` persisten dentro del directorio de la app.

## Requisitos
- Rust toolchain solo para la primera compilación (opcional si el binario ya está en `bin/`).
- Ubuntu / Linux x86_64.

## Formatos
ZIP, RAR4/RAR5 (rars), 7Z, TAR + compresiones GZ/BZ2/XZ, detección por firma.
