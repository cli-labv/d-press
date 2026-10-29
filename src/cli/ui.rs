use console::{Color, Term, measure_text_width, style, truncate_str};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Normal,
    Good,
    Warn,
    Bad,
}

pub fn print_rows(title: &str, rows: &[(&str, String, Tone)]) {
    let label_width = rows
        .iter()
        .map(|(label, _, _)| measure_text_width(label))
        .max()
        .unwrap_or(0);
    let lines: Vec<(String, Tone)> = rows
        .iter()
        .map(|(label, value, tone)| {
            let pad = label_width.saturating_sub(measure_text_width(label));
            (format!("{}{}  {}", label, " ".repeat(pad), value), *tone)
        })
        .collect();
    print_box(title, &lines);
}

pub fn print_notice(title: &str, lines: &[String]) {
    let entries: Vec<(String, Tone)> = lines
        .iter()
        .cloned()
        .map(|line| (line, Tone::Warn))
        .collect();
    print_box(title, &entries);
}

fn print_box(title: &str, lines: &[(String, Tone)]) {
    let terminal_width = Term::stderr().size().1 as usize;
    let max_outer = terminal_width.saturating_sub(4).max(24);
    let max_inner = max_outer.saturating_sub(4).min(72);

    let title = truncate_str(title, max_inner.saturating_sub(2), "…").to_string();
    let title_width = measure_text_width(&title);
    let content_width = lines
        .iter()
        .map(|(line, _)| measure_text_width(line))
        .max()
        .unwrap_or(0);
    let inner = content_width
        .max(title_width + 1)
        .max(30)
        .min(max_inner)
        .max(1);
    let dashes = inner.saturating_sub(title_width + 1);

    let top = format!("╭─ {} {}╮", title, "─".repeat(dashes));
    let bottom = format!("╰{}╯", "─".repeat(inner + 2));
    eprintln!("{}", accent(&top, Color::Cyan, true));

    for (line, tone) in lines {
        for part in wrap_line(line, inner) {
            let pad = inner.saturating_sub(measure_text_width(&part));
            let body = format!(" {}{} ", part, " ".repeat(pad));
            eprintln!(
                "{}{}{}",
                accent("│", Color::Cyan, false),
                paint(&body, *tone),
                accent("│", Color::Cyan, false)
            );
        }
    }

    eprintln!("{}", accent(&bottom, Color::Cyan, false));
    eprintln!();
}

fn wrap_line(text: &str, width: usize) -> Vec<String> {
    if width == 0 || measure_text_width(text) <= width {
        return vec![text.to_string()];
    }

    let indent_len = text.len() - text.trim_start_matches(' ').len();
    let indent = &text[..indent_len];
    let body = &text[indent_len..];
    let available = width.saturating_sub(indent_len).max(1);

    let mut lines = Vec::new();
    let mut current = String::new();
    for word in body.split_whitespace() {
        let candidate_width = if current.is_empty() {
            measure_text_width(word)
        } else {
            measure_text_width(&current) + 1 + measure_text_width(word)
        };
        if candidate_width <= available {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
            continue;
        }
        if !current.is_empty() {
            lines.push(format!("{indent}{current}"));
            current.clear();
        }
        if measure_text_width(word) <= available {
            current.push_str(word);
            continue;
        }
        let mut rest = word.to_string();
        while !rest.is_empty() {
            let (head, tail) = split_at_width(&rest, available);
            if tail.is_empty() {
                current = head;
                break;
            }
            lines.push(format!("{indent}{head}"));
            rest = tail;
        }
    }
    if !current.is_empty() || lines.is_empty() {
        lines.push(format!("{indent}{current}"));
    }
    lines
}

fn split_at_width(text: &str, width: usize) -> (String, String) {
    let mut used = 0;
    for (index, character) in text.char_indices() {
        let character_width = measure_text_width(&character.to_string());
        if used + character_width > width {
            return (text[..index].to_string(), text[index..].to_string());
        }
        used += character_width;
    }
    (text.to_string(), String::new())
}

fn use_color() -> bool {
    console::colors_enabled_stderr() && std::env::var_os("NO_COLOR").is_none()
}

fn accent(text: &str, color: Color, bold: bool) -> String {
    if !use_color() {
        return text.to_string();
    }
    let styled = style(text.to_string()).for_stderr().fg(color);
    if bold {
        styled.bold().to_string()
    } else {
        styled.to_string()
    }
}

fn paint(text: &str, tone: Tone) -> String {
    if !use_color() {
        return text.to_string();
    }
    let styled = style(text.to_string()).for_stderr();
    match tone {
        Tone::Normal => styled.to_string(),
        Tone::Good => styled.fg(Color::Green).to_string(),
        Tone::Warn => styled.fg(Color::Yellow).to_string(),
        Tone::Bad => styled.fg(Color::Red).to_string(),
    }
}
