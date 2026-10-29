# d-press

CLI en Rust para descompresión masiva de archivos comprimidos, con modo
interactivo, reanudación de corridas y registro de historial.

## Características

- Recorre recursivamente la carpeta de entrada y detecta el formato por firma
  (no por extensión).
- Barra de progreso con estado por archivo (OK / ERR / skip).
- Dos modos de salida: `flat` (todo al primer nivel) y `preserve` (conserva la
  estructura de carpetas del origen).
- Evita colisiones renombrando la carpeta destino con sufijos `_1`, `_2`, …
- Reanudación: guarda el estado por corrida y omite archivos ya extraídos.
- Historial de corridas en JSON Lines.
- Banner ASCII configurable (temas, colores, animación).

## Uso rápido

```bash
./start.sh
```

La primera ejecución compila el binario estático; las siguientes reutilizan el
binario cacheado en `bin/`.

### Modo no interactivo

```bash
./start.sh -i ./input -o ./output --mode preserve --yes
```

### Opciones

| Opción | Descripción | Valor por defecto |
| --- | --- | --- |
| `-i, --input <DIR>` | Carpeta de entrada | — |
| `-o, --output <DIR>` | Carpeta de salida | — |
| `-m, --mode <MODE>` | `flat` o `preserve` | `flat` |
| `--yes` | No pedir confirmación | `false` |
| `--no-banner` | Ocultar banner | `false` |
| `--simple-banner` | Banner simple | `false` |
| `--compact-banner` | Banner compacto | `false` |
| `--ascii-only` | Banner solo ASCII | `false` |
| `--no-color` | Sin colores | `false` |
| `--no-animation` | Sin animación | `false` |
| `--fast-banner` | Banner rápido | `false` |
| `--cinematic-banner` | Banner cinematográfico | `false` |
| `--banner-theme <THEME>` | Tema del banner | — |

Si se pasan `--input` y `--output`, se ejecuta el modo no interactivo; en caso
contrario se abre el menú interactivo (procesar `./input`, ver historial,
reanudar, salir). La confirmación final es **sí** por defecto: pulsa Enter para
iniciar la extracción.

## Modos de salida

- **flat**: cada archivo se extrae a `output/<nombre-del-archivo-sin-extensión>/`.
- **preserve**: se replica la ruta relativa dentro de `input/`, es decir
  `output/<subcarpetas>/<nombre-del-archivo-sin-extensión>/`.

Ejemplo:

```text
input/  manga/tomo1.zip
output/ manga/tomo1/...        # preserve
output/ tomo1/...              # flat
```

## Compatibilidad especial

El comportamiento predeterminado no cambia: los nombres se usan tal cual salvo
que no sean portables. En ese caso se aplica un ajuste **solo a ese archivo** y
se avisa de forma explícita antes y durante la extracción.

Se corrige:

- Puntos (`.`) o espacios al final de carpetas y entradas, que Windows,
  FAT/exFAT y muchos recursos SMB eliminan o rechazan.
- Caracteres no válidos (`< > : " | ? *` y controles).
- Nombres reservados de Windows (`CON`, `NUL`, `COM1`, `LPT1`, …).

Ejemplo: una carpeta generada como `Boku no Kaa-chan｜Mi Mamá es...` pasa a
`Boku no Kaa-chan｜Mi Mamá es` y la app muestra:

```text
╭─ Compatibilidad especial ──────────────────────────────╮
│ Boku no Kaa-chan｜Mi Mamá es....zip                    │
│   carpeta «Boku no Kaa-chan｜Mi Mamá es» · punto final │
│ Solo estos archivos · no es el modo normal             │
╰────────────────────────────────────────────────────────╯
```

El diagnóstico previo indica cuántos archivos requieren este modo y cuáles son.
Si el motor nativo de ZIP/RAR falla, la app intenta un motor externo disponible
(`7zz`, `7z`, `7za` o `unzip` para ZIP) y también lo registra como
compatibilidad especial. Los ajustes quedan guardados en el estado de la corrida
(campo `compatibility`).

## Estructura del proyecto

```text
input/                 archivos comprimidos de origen
output/s+extracciones limpias
registros/state/          estado de cada corrida (<hash>.json)
registros/logs/           historial.jsonl
bin/                   binarios estáticos (x86_64 / aarch64 musl)
src/cli/               argumentos, menús, banner, diagnóstico, historial
src/core/              escaneo, planificación y orquestación de extracción
src/engines/           detección por firma y motores ZIP / RAR
src/state/             estado de corrida e historial
```

## Formatos

| Formato | Detección | Extracción |
| --- | --- | --- |
| ZIP | Sí | Sí |
| RAR (1.5–4 y 5) | Sí | Sí |
| 7z | Sí | Pendiente |
| TAR | Sí | Pendiente |
| GZ / BZ2 / XZ (sueltos) | Sí | Pendiente |

Los archivos detectados pero aún no soportados cuentan como compatibles en el
diagnóstico previo y se omiten durante la extracción.

## Estado y reanudación

Cada corrida se identifica con un hash de las rutas de entrada y salida y se
guarda en `registros/state/<hash>.json`. Al repetir la misma corrida, los archivos
marcados como `Extracted` se omiten. Los estados posibles son `Pending`,
`Extracted`, `Error` y `Skipped`.

El historial se añade una sola vez por corrida en `registros/logs/historial.jsonl`
con el formato `{id, input, output, ts}`.

## Requisitos

- Linux x86_64 o aarch64.
- Rust toolchain solo para la primera compilación (opcional si ya existe un
  binario en `bin/`).
- `start.sh` compila con `cargo build --release --target *-unknown-linux-musl`.

## Solución de problemas

- **No hay binario y falta Rust**: copia el proyecto a una máquina con toolchain
  y ejecuta `./start.sh`, o coloca el binario compilado en
  `bin/d-press-<target>-musl`.
- **«No compatibles» en el diagnóstico**: el archivo no es un formato
  reconocido o está dañado; revisa la firma real del archivo.
- **Reanudar no reprocesa errores**: el estado solo omite los `Extracted`; los
  `Error` se vuelven a intentar en la siguiente corrida.
