# d-press 🚀

> **Descompresión masiva en Rust** — menú interactivo, reanudación inteligente y registro de historial.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green)
![Platform](https://img.shields.io/badge/Platform-Linux%20x86_64%20%2F%20aarch64-blue)

---

## ✨ Características

- 🔍 Detección por firma, no por extensión
- 📊 Barra de progreso con estado por archivo `✓ / ✖ / omitido`
- 📁 Dos modos de salida: `flat` y `preserve`
- 🛡️ Evita colisiones renombrando automáticamente
- 🔁 Reanudación de corridas con estado persistente
- 📓 Historial en JSON Lines
- 🎨 Banner ASCII configurable con temas y animación
- ⚠️ Modo de compatibilidad especial solo cuando es necesario

---

## 🛠️ Uso rápido

```bash
./start.sh                # menú interactivo
./start.sh -i ./input -o ./output --mode preserve --yes
```

La primera ejecución compila el binario estático; las siguientes reutilizan `bin/`.

### Opciones

| Opción | Descripción | Default |
| --- | --- | --- |
| `-i, --input <DIR>` | Carpeta de entrada | — |
| `-o, --output <DIR>` | Carpeta de salida | — |
| `-m, --mode <MODE>` | `flat` o `preserve` | `flat` |
| `--yes` | No pedir confirmación | `false` |
| `--no-banner` | Ocultar banner | `false` |
| `--banner-theme <THEME>` | Tema del banner | `neon` |

> En modo interactivo la confirmación es **sí por defecto**: pulsa **Enter** para iniciar.

---

## 📁 Modos de salida

- **flat** → `output/<nombre-archivo>/`
- **preserve** → `output/<ruta-relativa>/<nombre-archivo>/`

```text
input/  manga/tomo1.zip
output/ manga/tomo1/...        # preserve
output/ tomo1/...              # flat
```

---

## ⚠️ Compatibilidad especial

El comportamiento por defecto no cambia. Solo cuando un nombre no es portable se aplica un ajuste **a ese archivo** y se avisa explícitamente.

Se corrige:
- Puntos/ espacios finales de carpetas
- Caracteres inválidos `< > : " | ? *`
- Nombres reservados Windows `CON`, `NUL`, `COM1`…

Ejemplo:

```text
╭─ Compatibilidad especial ──────────────────────────────╮
│ Boku no Kaa-chan｜Mi Mamá es....zip                    │
│   carpeta «Boku no Kaa-chan｜Mi Mamá es» · punto final │
│ Solo estos archivos · no es el modo normal             │
╰────────────────────────────────────────────────────────╯
```

Si el motor nativo falla, se intenta respaldo con `7zz/7z/7za` o `unzip`. Los ajustes quedan en `registros/state/<hash>.json`.

---

## 📂 Estructura del proyecto

```text
d-press/
├─ input/                 # archivos comprimidos de origen
├─ output/                # extracciones limpias
├─ registros/
│  ├─ state/              # estado por corrida
│  └─ logs/               # historial.jsonl
├─ bin/                   # binarios estáticos
└─ src/
   ├─ cli/                # argumentos, menús, banner, diagnóstico
   ├─ core/               # escaneo, planificación, portable
   ├─ engines/            # ZIP / RAR / externo
   └─ state/              # RunState / History
```

`.gitignore` excluye `input/`, `output/`, `registros/` y `target/` para mantener el repo limpio.

---

## 📊 Diagnóstico

```text
╭─ Diagnóstico ──────────────────╮
│ Archivos        3              │
│ Compatibles     3              │
│ Especiales      1              │
│ No compatibles  0              │
╰────────────────────────────────╯
```

---

## 🔁 Estado y reanudación

Cada corrida se identifica con un hash de entrada/salida y se guarda en `registros/state/<hash>.json`. Al repetir la misma corrida los archivos ya `Extracted` se omiten.

Historial: `registros/logs/historial.jsonl`

---

## 🧪 Formatos

| Formato | Detección | Extracción |
| --- | --- | --- |
| ZIP | ✅ | ✅ |
| RAR 1.5–4 / 5 | ✅ | ✅ |
| 7z | ✅ | ⏳ |
| TAR | ✅ | ⏳ |

---

## 🛠️ Requisitos

- Linux x86_64 / aarch64
- Rust solo para la primera compilación, o binario en `bin/`

---

**d-press** — extracción masiva, segura y reproducible.
