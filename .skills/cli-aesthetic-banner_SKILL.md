---
name: cli-aesthetic-banner
description: Diseña banners CLI animados, estéticos, responsivos y completamente configurables usando arte ASCII, colores, efectos de escritura, cajas centradas y modos animado/simple.
version: 1.0.0
---

# Skill: CLI Aesthetic Banner

## Objetivo

Crear banners visuales profesionales para aplicaciones CLI.

El banner debe poder:

- Mostrar arte ASCII.
- Mostrar emojis o símbolos.
- Aplicar colores individualmente por línea.
- Animar la aparición del contenido.
- Simular escritura progresiva.
- Mostrar subtítulos dentro de una caja.
- Adaptarse al ancho de la terminal.
- Funcionar en modo animado y modo simple.
- Desactivarse automáticamente en terminales no interactivas.
- Ser reutilizable para cualquier aplicación CLI.
- Permitir generar infinitos diseños mediante configuración declarativa.

La skill debe separar completamente:

1. El contenido visual.
2. La configuración de animación.
3. La lógica de renderizado.
4. El sistema de colores.
5. La detección de terminal.
6. Las variantes del banner.

---

# 1. Principio arquitectónico

Nunca escribir el banner completo dentro de una única función con muchos `println!`.

Usar una estructura basada en datos.

Cada elemento visual debe representarse como una línea configurable:

```rust
pub struct BannerLine {
    pub content: String,
    pub color: BannerColor,
    pub delay_ms: u64,
    pub typing: bool,
}
```

El banner completo debe representarse como una colección ordenada:

```rust
pub struct BannerDefinition {
    pub name: &'static str,
    pub logo: Vec<BannerLine>,
    pub tagline: Vec<BannerLine>,
    pub footer: Vec<BannerLine>,
    pub separator: Option<BannerLine>,
}
```

Esto permite crear nuevos banners sin modificar el motor principal.

---

# 2. Componentes del banner

Todo banner debe poder componerse con estas secciones:

## 2.1. Encabezado visual

Puede contener:

- Iconos.
- Llamas.
- Flechas.
- Cajas.
- Símbolos relacionados con la aplicación.
- Decoraciones simétricas.

Ejemplo conceptual:

```text
                 ◈
               ◈ ◈ ◈
             ◈ ◈ ◈ ◈ ◈
```

## 2.2. Logotipo ASCII

El nombre de la aplicación debe estar formado por varias líneas de arte ASCII.

Cada línea puede tener un color distinto para crear un efecto de degradado visual.

```rust
const LOGO_LINES: &[(&str, BannerColor)] = &[
    ("    ██████╗       ██████╗", BannerColor::CyanBright),
    ("    ██╔══██╗     ██╔════╝", BannerColor::Cyan),
    ("    ██████╔╝     ██║     ", BannerColor::Blue),
    ("    ██╔═══╝      ██║     ", BannerColor::BlueBright),
    ("    ██║          ╚██████╗", BannerColor::White),
];
```

El logotipo debe ser original. No copiar diseños protegidos de terceros.

## 2.3. Tagline

Debe explicar brevemente la función de la aplicación.

Ejemplo:

```text
╭──────────────────────────────────────────────╮
│       Descompresión masiva y segura          │
│        ⚡ Rápido · 📦 Portable · 🛡 Seguro   │
╰──────────────────────────────────────────────╯
```

La caja debe calcular automáticamente su ancho.

## 2.4. Información secundaria

Puede incluir:

- Versión.
- Sitio del proyecto.
- Modo de ejecución.
- Plataforma.
- Estado del runtime.
- Número de versión.

Ejemplo:

```text
                         v1.0.0 │ d-press
```

## 2.5. Separador

Usar un separador visual después del banner:

```text
    ────────────────────────────────────────────────────────────────
```

El separador debe adaptarse al ancho disponible de la terminal.

---

# 3. Sistema de colores

Crear un enum controlado para evitar colores arbitrarios:

```rust
#[derive(Debug, Clone, Copy)]
pub enum BannerColor {
    Default,
    Black,
    Red,
    RedBright,
    Yellow,
    YellowBright,
    Green,
    GreenBright,
    Cyan,
    CyanBright,
    Blue,
    BlueBright,
    Magenta,
    MagentaBright,
    White,
    WhiteBright,
    Dim,
}
```

Crear una función centralizada:

```rust
fn colorize(text: &str, color: BannerColor) -> String
```

La función debe:

- Aplicar el color solicitado.
- Respetar la variable `NO_COLOR`.
- Evitar códigos ANSI cuando la salida no sea una terminal.
- Devolver texto plano cuando los colores estén desactivados.

Nunca aplicar colores directamente por todo el proyecto. Toda la salida visual debe pasar por el sistema de estilos.

---

# 4. Animación de escritura

Implementar una función de escritura progresiva:

```rust
fn animate_typing(
    text: &str,
    color: BannerColor,
    char_delay_ms: u64,
)
```

Comportamiento:

1. Aplicar el color al texto.
2. Recorrer el texto carácter por carácter.
3. Imprimir cada carácter inmediatamente.
4. Ejecutar `flush()` después de cada carácter.
5. Esperar únicamente cuando el carácter no sea un espacio.
6. Finalizar con salto de línea.

Pseudocódigo:

```rust
for character in colored_text.chars() {
    print!("{character}");
    stdout().flush()?;

    if !character.is_whitespace() {
        sleep(Duration::from_millis(char_delay_ms));
    }
}

println!();
```

La velocidad debe ser configurable:

```rust
pub struct TypingConfig {
    pub enabled: bool,
    pub char_delay_ms: u64,
    pub skip_whitespace_delay: bool,
}
```

Valores recomendados:

- Rápido: 1–3 ms.
- Normal: 5–8 ms.
- Cinemático: 10–20 ms.
- Accesible: animación desactivada.

---

# 5. Animación por línea

Implementar una función independiente:

```rust
fn animate_line(
    text: &str,
    color: BannerColor,
    delay_ms: u64,
)
```

Esta función debe:

1. Imprimir la línea completa.
2. Aplicar el color.
3. Esperar el tiempo indicado.
4. No introducir pausas cuando las animaciones estén desactivadas.

Esto permite combinar:

- Líneas estáticas.
- Líneas con efecto typing.
- Líneas con aparición progresiva.
- Pausas dramáticas.
- Diferentes velocidades por sección.

---

# 6. Configuración global de animación

Crear una configuración centralizada:

```rust
pub struct BannerAnimationConfig {
    pub enabled: bool,
    pub typing_enabled: bool,
    pub char_delay_ms: u64,
    pub line_delay_ms: u64,
    pub dramatic_pause_ms: u64,
    pub tagline_delay_ms: u64,
    pub footer_delay_ms: u64,
}
```

La configuración debe poder modificarse mediante:

- Variables de entorno.
- Archivo `.env`.
- Argumentos CLI.
- Perfil predefinido.

Variables sugeridas:

```env
BANNER_ENABLED=true
BANNER_TYPING=true
BANNER_CHAR_DELAY_MS=8
BANNER_LINE_DELAY_MS=20
BANNER_TAGLINE_DELAY_MS=40
BANNER_DRAMATIC_PAUSE_MS=100
```

Argumentos sugeridos:

```bash
d-press --no-banner
d-press --simple-banner
d-press --fast-banner
d-press --cinematic-banner
d-press --banner-theme cyan
```

---

# 7. Centrado y cálculo de ancho

Nunca calcular el ancho usando únicamente `text.len()` porque:

- Los emojis ocupan más de un byte.
- Algunos caracteres Unicode tienen ancho visual diferente.
- Los códigos ANSI no deben contar como ancho visible.

Usar una función de ancho visual:

```rust
fn visible_width(text: &str) -> usize
```

Preferentemente utilizar:

```rust
console::measure_text_width
```

o una función equivalente que ignore correctamente los códigos ANSI.

Implementar:

```rust
fn pad_center_to_width(text: &str, width: usize) -> String
```

Comportamiento:

- Si el texto ya ocupa el ancho, devolverlo intacto.
- Calcular el espacio restante.
- Repartirlo entre izquierda y derecha.
- Si el espacio es impar, asignar el carácter sobrante al lado derecho.

También implementar:

```rust
fn pad_to_width(text: &str, width: usize) -> String
```

Esta función debe completar espacios a la derecha para que las cajas mantengan sus bordes alineados.

---

# 8. Cajas adaptables

Crear una función genérica para construir cajas:

```rust
fn build_box(
    lines: &[BannerLine],
    indent: usize,
    horizontal: char,
    vertical: char,
) -> Vec<BannerLine>
```

La función debe:

1. Calcular el ancho máximo visible.
2. Agregar margen interno.
3. Crear línea superior.
4. Centrar cada contenido.
5. Crear línea inferior.
6. Mantener el mismo ancho visual en todas las líneas.

Ejemplo:

```text
╭────────────────────────────────────────────╮
│        Descompresión masiva y segura       │
│       ⚡ Rápido · Portable · Seguro         │
╰────────────────────────────────────────────╯
```

Permitir diferentes estilos:

```rust
pub enum BoxStyle {
    Rounded,
    Double,
    Square,
    Minimal,
    Unicode,
    Ascii,
}
```

Ejemplos:

```text
Rounded:
╭──────────────╮
│ d-press      │
╰──────────────╯

Double:
╔══════════════╗
║ d-press      ║
╚══════════════╝

Ascii:
+--------------+
| d-press      |
+--------------+
```

---

# 9. Temas visuales ilimitados

El motor debe permitir crear temas mediante configuración:

```rust
pub struct BannerTheme {
    pub name: &'static str,
    pub logo_colors: Vec<BannerColor>,
    pub tagline_color: BannerColor,
    pub border_color: BannerColor,
    pub footer_color: BannerColor,
    pub separator_color: BannerColor,
    pub box_style: BoxStyle,
}
```

Incluir temas iniciales:

```rust
pub fn available_themes() -> Vec<BannerTheme> {
    vec![
        neon_theme(),
        ocean_theme(),
        terminal_theme(),
        fire_theme(),
        monochrome_theme(),
        extraction_theme(),
    ]
}
```

Cada tema debe modificar únicamente la presentación, no la lógica del banner.

Ejemplos de temas:

- `neon`
- `ocean`
- `terminal`
- `fire`
- `monochrome`
- `extraction`
- `minimal`
- `high-contrast`

Agregar selección:

```bash
d-press --banner-theme neon
d-press --banner-theme ocean
d-press --banner-theme monochrome
```

---

# 10. Variantes de banner

El sistema debe soportar varios banners para la misma aplicación:

```rust
pub enum BannerVariant {
    Full,
    Compact,
    Minimal,
    Success,
    Error,
    Goodbye,
    Processing,
    Completed,
}
```

Funciones requeridas:

```rust
pub fn show_animated_banner(config: &BannerConfig)
pub fn show_simple_banner(config: &BannerConfig)
pub fn show_compact_banner(config: &BannerConfig)
pub fn show_processing_banner(config: &BannerConfig)
pub fn show_completed_banner(config: &BannerConfig)
pub fn show_goodbye_banner(config: &BannerConfig)
```

Ejemplos de uso:

```rust
show_animated_banner(&config);
show_processing_banner(&config);
show_completed_banner(&config);
show_goodbye_banner(&config);
```

---

# 11. Banner de inicio

El flujo recomendado para un banner de inicio es:

1. Mostrar decoración superior.
2. Mostrar icono o símbolo principal.
3. Mostrar logo ASCII.
4. Hacer una pausa corta.
5. Mostrar tagline dentro de una caja.
6. Mostrar versión.
7. Mostrar URL o nombre del proyecto.
8. Mostrar separador.
9. Entregar el control al menú principal.

Pseudocódigo:

```rust
pub fn show_animated_banner(config: &BannerConfig) {
    if !config.should_render {
        return;
    }

    render_flame_or_symbol(config);
    render_logo(config);
    sleep(config.animation.dramatic_pause);
    render_tagline_box(config);
    render_version(config);
    render_separator(config);
}
```

---

# 12. Banner de despedida

Crear un banner de despedida con la misma identidad:

```text
        ◇ Gracias por usar d-press ◇
          Archivos liberados correctamente
```

Debe ser corto y no utilizar animaciones largas.

---

# 13. Compatibilidad de terminal

Antes de renderizar, comprobar:

- Si stdin es una terminal.
- Si stdout es una terminal.
- Si existe soporte de color.
- Si se definió `NO_COLOR`.
- Si la terminal tiene ancho suficiente.
- Si el entorno es CI.
- Si se solicitó `--no-banner`.
- Si se está ejecutando mediante redirección.

Reglas:

```rust
fn should_render_animated_banner() -> bool
```

Debe devolver `false` cuando:

- `stdout` no sea TTY.
- Se haya configurado `NO_COLOR`.
- Se use `--no-banner`.
- Se ejecute en modo JSON.
- Se ejecute en CI.
- La animación esté deshabilitada.

En terminales pequeñas:

- Usar banner compacto.
- Reducir el ancho del logo.
- Ocultar decoraciones secundarias.
- Evitar que las cajas se desborden.

---

# 14. Modo simple

El modo simple debe mostrar el mismo contenido sin animación:

```rust
pub fn show_simple_banner(config: &BannerConfig) {
    render_static_logo(config);
    render_static_tagline(config);
    render_static_footer(config);
}
```

Debe conservar:

- Colores.
- Alineación.
- Cajas.
- Separadores.
- Versión.
- Identidad visual.

Solo se eliminan:

- Delays.
- Typing.
- Pausas.
- Efectos animados.

---

# 15. Accesibilidad

El banner no debe depender únicamente de emojis o colores.

Debe incluir texto legible:

```text
d-press - Descompresión masiva y segura
```

Cuando se desactive el color:

- El contenido debe seguir siendo comprensible.
- Los estados no deben depender solo del color.
- Los símbolos deben acompañar al texto.
- El modo ASCII debe funcionar en terminales limitadas.

Agregar:

```bash
d-press --ascii-only
d-press --no-color
d-press --simple-banner
```

---

# 16. Seguridad y control

La skill debe impedir:

- Animaciones infinitas.
- Bucles sin límite.
- Delays excesivamente largos.
- Banner que bloquee el procesamiento.
- Salida que dañe terminales.
- Secuencias ANSI no controladas.
- Contenido de usuario ejecutado como código terminal.
- Ancho negativo o desbordado.
- Errores por emojis mal alineados.

Establecer límites:

```rust
const MAX_CHAR_DELAY_MS: u64 = 100;
const MAX_LINE_DELAY_MS: u64 = 2_000;
const MAX_BANNER_WIDTH: usize = 160;
const MIN_TERMINAL_WIDTH: usize = 40;
```

Todos los valores configurados por el usuario deben validarse.

---

# 17. Generación de nuevos banners

Para crear un nuevo banner, únicamente debe ser necesario definir:

```rust
pub fn dpress_banner() -> BannerDefinition {
    BannerDefinition {
        name: "d-press",
        logo: vec![
            BannerLine::typing("...", BannerColor::CyanBright),
            BannerLine::typing("...", BannerColor::Cyan),
            BannerLine::static_line("...", BannerColor::White),
        ],
        tagline: vec![
            BannerLine::static_line(
                "Descompresión masiva, rápida y segura",
                BannerColor::CyanBright,
            ),
            BannerLine::static_line(
                "📦 Portable · ⚡ Paralelo · 🛡 Seguro",
                BannerColor::White,
            ),
        ],
        footer: vec![
            BannerLine::static_line("v1.0.0", BannerColor::Dim),
        ],
        separator: Some(BannerLine::static_line(
            "────────────────────────────────────",
            BannerColor::Dim,
        )),
    }
}
```

No modificar el motor de renderizado para añadir un nuevo diseño.

---

# 18. Requisitos de salida de la skill

Cuando se solicite crear un banner, entregar:

1. Concepto visual.
2. Paleta de colores.
3. Arte ASCII.
4. Tagline.
5. Configuración de animación.
6. Variante animada.
7. Variante simple.
8. Variante compacta.
9. Funciones de renderizado.
10. Validaciones de terminal.
11. Pruebas de alineación.
12. Ejemplo de integración.
13. Variables de configuración.
14. Opciones CLI.
15. Límites de seguridad.

---

# 19. Checklist de calidad

Antes de considerar terminado un banner, comprobar:

- [ ] Tiene identidad visual propia.
- [ ] El logo está separado por líneas.
- [ ] Cada línea puede tener color propio.
- [ ] El color puede desactivarse.
- [ ] El typing puede desactivarse.
- [ ] El banner tiene versión simple.
- [ ] El banner tiene versión compacta.
- [ ] El texto está correctamente centrado.
- [ ] Los emojis no rompen las cajas.
- [ ] Funciona en terminales pequeñas.
- [ ] Funciona sin TTY.
- [ ] No genera secuencias ANSI peligrosas.
- [ ] No bloquea la aplicación.
- [ ] Tiene límites de animación.
- [ ] Puede reutilizarse para otra aplicación.
- [ ] Es posible crear nuevos temas sin modificar el motor.
- [ ] Es posible crear nuevos banners sin duplicar lógica.