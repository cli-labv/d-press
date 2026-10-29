use console::{Term, colors_enabled};

const DPRESS_BIG: &[&str] = &[
    "██████╗       ██████╗ ██████╗ ███████╗███████╗███████╗",
    "██╔══██╗      ██╔══██╗██╔══██╗██╔════╝██╔════╝██╔════╝",
    "██║  ██║      ██████╔╝██████╔╝█████╗  ███████╗███████╗",
    "██║  ██║      ██╔═══╝ ██╔══██╗██╔══╝  ╚════██║╚════██║",
    "██████╔╝      ██║     ██║  ██║███████╗███████║███████║",
    "╚═════╝       ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝",
];

pub fn print_big_banner() {
    let term = Term::stderr();
    let colors_on = colors_enabled() && std::env::var_os("NO_COLOR").is_none();
    if !colors_on {
        for line in DPRESS_BIG {
            let _ = term.write_line(line);
        }
        let _ = term.write_line("");
        return;
    }
    // Degradado: magenta -> cian -> azul
    let colors = [
        "\x1b[95m", "\x1b[35m", "\x1b[96m", "\x1b[36m", "\x1b[94m", "\x1b[34m",
    ];
    for (i, line) in DPRESS_BIG.iter().enumerate() {
        let c = colors[i % colors.len()];
        eprintln!("{}{}\x1b[0m", c, line);
    }
    let _ = term.write_line("");
}
