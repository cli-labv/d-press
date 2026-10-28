# d-press — Prompt de desarrollo

Actúa como ingeniero Rust senior especializado en CLIs multiplataforma de procesamiento masivo de archivos.

Construí **d-press**: una CLI en Rust para descomprimir masivamente archivos comprimidos desde una carpeta de entrada hacia una de salida, con menú interactivo por flechas, rápida, portable y en un solo binario.

---

## 1. Objetivo

Detectar y descomprimir en lote (cientos o miles de archivos, en paralelo, sin bloquear la interfaz):

**ZIP · RAR (RAR4 y RAR5) · 7Z · TAR · TAR.GZ/TGZ · TAR.BZ2/TBZ2 · TAR.XZ · GZ · BZ2 · XZ**

Detección por extensión, verificada por firma binaria (magic bytes) para no confiar solo en el nombre del archivo.

---

## 2. Motores de extracción (Rust puro, un solo binario)

| Formato | Crate |
|---|---|
| ZIP | `zip` |
| TAR y contenedores | `tar` + `flate2` (gzip) + `bzip2` + `xz2` |
| 7Z | `sevenz-rust` |
| RAR | `rar-rs` / `librar` (pure Rust, RAR4+RAR5) |
| Detección por firma binaria | `infer` |

**Paso obligatorio antes de construir el resto de la app:** probar `rar-rs` y `sevenz-rust` contra archivos reales de cada tipo, incluyendo RAR5 generado con una versión reciente de WinRAR. Si `rar-rs` falla en algún caso real, el respaldo es shellear al binario `unrar`/`7z` del sistema si está disponible, avisando explícitamente en el diagnóstico previo cuándo se usó ese camino en vez del nativo. Nunca fallar en silencio ni declarar un formato soportado si no se puede extraer de verdad.

---

## 3. Modos de organización de salida

**Regla fija en ambos modos, sin excepción:** cada archivo comprimido se extrae siempre dentro de su propia carpeta, nombrada igual que el archivo (sin extensión) — el contenido de dos archivos distintos nunca se mezcla.

**Modo A — Una sola carpeta de salida (por defecto):**
```
input/
├── fotos.rar
├── documentos.zip
└── respaldos/cliente.7z

output/
├── fotos/
├── documentos/
└── cliente/
```
Si dos archivos de subcarpetas distintas tienen el mismo nombre, se evita la colisión con un sufijo corto derivado de su ruta original — nunca se sobrescribe en silencio.

**Modo B — Preservar estructura original:**
```
input/
├── cliente-a/datos.zip
└── cliente-b/imagenes.rar

output/
├── cliente-a/datos/
└── cliente-b/imagenes/
```

El usuario elige el modo desde el menú antes de empezar.

---

## 4. Flujo de interfaz interactiva

```
Banner (figlet-rs, con modo simple automático si la terminal no soporta animación)
   │
   ▼ Menú principal (dialoguer, navegación por flechas, nunca escribir un número)
     ❯ Procesar carpeta ./input
       Seleccionar carpeta externa
       Ver historial
       Salir
   │
   ▼ Preguntas de configuración: modo de salida (A/B), secuencial o paralelo,
     cantidad de workers, política de colisiones
   │
   ▼ Diagnóstico previo: archivos encontrados, compatibles, no compatibles,
     tamaño total, cuántos piden contraseña, advertencias — antes de tocar disco
   │
   ▼ Confirmación explícita (obligatoria, sin excepción, antes de cualquier
     acción irreversible)
   │
   ▼ Ejecución con barra de progreso (indicatif): archivo actual, % global,
     velocidad, aciertos/errores/omitidos
   │
   ▼ Resumen final en un cuadro visual. La app NO se cierra sola: espera la
     siguiente orden (volver al menú o salir), con el historial ya guardado
```

La app también debe funcionar en modo no interactivo por flags (`clap`), para scripting y para cuando no hay una terminal real detrás (se detecta automáticamente y cae a ese modo).

---

## 5. Persistencia (JSON simple, sin base de datos)

- `state/<id-de-corrida>.json`: estado de cada archivo de la corrida (`pendiente`/`extraído`/`error`/`omitido`), actualizado después de cada archivo procesado, para poder **reanudar** una operación interrumpida. Al arrancar, si hay una corrida incompleta, se ofrece reanudarla como primera opción del menú.
- `logs/historial.jsonl`: una línea por corrida completa (fecha, carpeta de entrada, modo usado, cantidad de archivos, resultado), consultable desde "Ver historial".

---

## 6. Seguridad de extracción

- Protección contra **Zip Slip**: validar que la ruta final de cada archivo extraído siga dentro de la carpeta de destino antes de escribirlo; rechazar rutas `../` o absolutas dentro del comprimido.
- Límite de tamaño expandido configurable, para frenar una bomba de descompresión.
- Enlaces simbólicos dentro del archivo: validar antes de crearlos, nunca seguir ciegamente.
- Si un archivo puntual resulta peligroso, se detiene solo esa tarea, se registra el motivo, y se continúa con el resto del lote.

---

## 7. Procesamiento paralelo

- `rayon` para procesar varios archivos a la vez; `jwalk` para recorrer la carpeta de entrada.
- Número de workers: automático según núcleos disponibles, configurable por menú o `--workers`.
- Nunca dos tareas escribiendo al mismo destino a la vez.
- Modo secuencial disponible para ver el detalle de cada archivo uno por uno.

---

## 8. Contraseñas

- Detectar archivos protegidos antes de intentar extraerlos.
- Pedir la contraseña sin mostrarla en pantalla; opción de reutilizarla para el resto de la sesión si varios archivos la comparten.
- Nunca guardar la contraseña en logs ni en el historial — solo registrar que ese archivo la requería.
- Opción de omitir el archivo protegido en vez de pedir contraseña.

---

## 9. Archivos multivolumen

Detectar y agrupar como una sola tarea lógica los volúmenes partidos (`.part1.rar`, `.r00`, `.7z.001`, etc.), verificando que estén todos presentes. Si falta una parte, avisar cuál falta y permitir continuar sin esa tarea o cancelarla, sin bloquear el resto del lote.

---

## 10. Librerías

| Necesidad | Crate |
|---|---|
| Banner | `figlet-rs` |
| Menú interactivo | `dialoguer` |
| Progreso | `indicatif` |
| Colores | `console` |
| Argumentos / modo flags | `clap` (derive) |
| Recorrido de carpetas | `jwalk` |
| Paralelismo | `rayon` |
| Formatos | ver §2 |
| Estado/historial | `serde` + `serde_json` |
| Pruebas de CLI de punta a punta | `assert_cmd` + `predicates` |

---

## 11. Estructura del proyecto

```
d-press/
├─ Cargo.toml · Cargo.lock
├─ README.md · docs/FORMATOS.md · docs/SEGURIDAD.md
├─ start.sh · start.ps1
├─ input/ · output/ · temp/ · logs/ · state/
└─ src/
   ├─ main.rs
   ├─ cli/
   │  ├─ banner.rs · menus.rs · diagnostics.rs · output.rs
   ├─ core/
   │  ├─ scanner.rs · extractor.rs · planner.rs · resume.rs · collision.rs
   ├─ engines/
   │  ├─ traits.rs · zip_engine.rs · rar_engine.rs · seven_zip_engine.rs
   │  ├─ tar_engine.rs · gzip_engine.rs · bzip2_engine.rs · xz_engine.rs · detector.rs
   ├─ security/
   │  ├─ path_safety.rs · zip_slip.rs
   ├─ state/
   │  ├─ run_state.rs · history.rs
   └─ utils/
      ├─ files.rs · formatting.rs · logging.rs
└─ tests/
```

---

## 12. Pruebas obligatorias

- Cada motor de formato, contra un archivo real de ese tipo (incluir RAR4 y RAR5 por separado).
- Zip Slip: archivo de prueba con rutas `../` — debe rechazarse.
- Modo A vs. Modo B: misma entrada, confirmando la estructura de salida esperada en cada uno.
- Colisión de nombres en Modo A.
- Multivolumen con una parte faltante: debe detectarse y avisar, sin procesar a medias.
- Reanudación: interrumpir a mitad de un lote grande y confirmar que retoma desde `state/*.json`.
- Contraseñas: confirmar que nunca aparecen en logs ni en el historial.
- Rendimiento con un lote de cientos de archivos variados.

---

## 13. Fases de desarrollo

1. **Verificación de motores**: probar `rar-rs` y `sevenz-rust` contra archivos reales antes de seguir.
2. **Núcleo de extracción**: scanner + engines + detector, los dos modos de salida funcionando por línea de comandos.
3. **Seguridad y estado**: Zip Slip, límites, `state/run_state.rs` + `history.rs`, reanudación.
4. **Interfaz interactiva completa**: banner, menús, diagnóstico previo, confirmación, progreso, resumen.
5. **Paralelismo y multivolumen**.
6. **Pulido**: contraseñas, modo flags, documentación, empaquetado para Linux/macOS/Windows.

---

## 14. Reglas no negociables

- Menú siempre navegable por flechas — nunca pedir al usuario que escriba un número.
- La app no se cierra sola al terminar un proceso: muestra resumen y espera la siguiente orden.
- Confirmación explícita antes de cualquier extracción real.
- Nada se da por terminado sin al menos una prueba automatizada que lo cubra.
- Sin base de datos: todo el estado y el historial son archivos JSON/JSONL.
- No declarar un formato como soportado si el motor no puede extraerlo de verdad.

---

## Tarea inicial

a) Fase 1 (verificación de motores): probar `rar-rs`/`librar` y `sevenz-rust` contra archivos reales de cada formato, incluido RAR5. Reportar resultados antes de continuar.
b) Fase 2: `scanner.rs` + `engines/*` + `detector.rs`, con los dos modos de salida funcionando por línea de comandos, sin interfaz todavía.
c) Reportar estructura creada y siguiente paso.