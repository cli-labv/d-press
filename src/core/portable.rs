use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NameIssue {
    Empty,
    TrailingDot,
    TrailingSpace,
    InvalidChar,
    ReservedName,
}

impl NameIssue {
    pub fn label(self) -> &'static str {
        match self {
            NameIssue::Empty => "nombre vacío",
            NameIssue::TrailingDot => "punto final",
            NameIssue::TrailingSpace => "espacio final",
            NameIssue::InvalidChar => "carácter no válido",
            NameIssue::ReservedName => "nombre reservado de Windows",
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameFix {
    pub original: String,
    pub value: String,
    pub issues: Vec<NameIssue>,
}

impl NameFix {
    pub fn reason(&self) -> String {
        self.issues
            .iter()
            .map(|issue| issue.label())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[derive(Debug, Clone)]
pub struct PathFix {
    pub original: String,
    pub value: PathBuf,
    pub fixes: Vec<NameFix>,
}

impl PathFix {
    pub fn changed(&self) -> bool {
        !self.fixes.is_empty()
    }

    pub fn reason(&self) -> String {
        let mut issues = BTreeSet::new();
        for fix in &self.fixes {
            issues.extend(fix.issues.iter().copied());
        }
        issues
            .iter()
            .map(|issue| issue.label())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub fn fix_component(component: &str) -> Option<NameFix> {
    let mut issues = BTreeSet::new();
    let mut value = String::with_capacity(component.len());

    for ch in component.chars() {
        let invalid = matches!(ch, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*')
            || (ch as u32) < 0x20;
        if invalid {
            issues.insert(NameIssue::InvalidChar);
            value.push('_');
        } else {
            value.push(ch);
        }
    }

    loop {
        if value.ends_with('.') {
            issues.insert(NameIssue::TrailingDot);
            value.pop();
        } else if value.ends_with(' ') {
            issues.insert(NameIssue::TrailingSpace);
            value.pop();
        } else {
            break;
        }
    }

    if value.is_empty() || value == "." || value == ".." {
        issues.insert(NameIssue::Empty);
        value.clear();
        value.push('_');
    }

    if is_reserved(&value) {
        issues.insert(NameIssue::ReservedName);
        value.insert(0, '_');
    }

    if issues.is_empty() {
        None
    } else {
        Some(NameFix {
            original: component.to_string(),
            value,
            issues: issues.into_iter().collect(),
        })
    }
}

pub fn fix_relative_path(raw: &str) -> Result<PathFix, String> {
    let normalized = raw.replace('\\', "/");
    if normalized.starts_with('/')
        || normalized.starts_with("//")
        || (normalized.len() >= 2 && normalized.as_bytes()[1] == b':')
    {
        return Err(format!("ruta insegura (absoluta): {raw}"));
    }

    let mut value = PathBuf::new();
    let mut fixes = Vec::new();

    for part in normalized.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            return Err(format!("ruta insegura ('..'): {raw}"));
        }
        match fix_component(part) {
            Some(fix) => {
                value.push(&fix.value);
                fixes.push(fix);
            }
            None => value.push(part),
        }
    }

    Ok(PathFix {
        original: raw.to_string(),
        value,
        fixes,
    })
}

fn is_reserved(name: &str) -> bool {
    let base = name
        .split('.')
        .next()
        .unwrap_or(name)
        .trim_end_matches([' ', '.']);
    let upper = base.to_ascii_uppercase();
    if matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL") {
        return true;
    }
    if upper.len() == 4 && (upper.starts_with("COM") || upper.starts_with("LPT")) {
        let digit = upper.as_bytes()[3];
        return digit.is_ascii_digit() && digit != b'0';
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_trailing_dots() {
        let fix = fix_component("Boku no Kaa-chan｜Mi Mamá es...").unwrap();
        assert_eq!(fix.value, "Boku no Kaa-chan｜Mi Mamá es");
        assert!(fix.issues.contains(&NameIssue::TrailingDot));
    }

    #[test]
    fn keeps_normal_names() {
        assert!(fix_component("tomo 1").is_none());
        assert!(fix_component("Boku no Kaa-chan 2｜Mi Mamá es... 2").is_none());
    }

    #[test]
    fn trims_trailing_spaces_and_dots_together() {
        let fix = fix_component("carpeta. . ").unwrap();
        assert_eq!(fix.value, "carpeta");
    }

    #[test]
    fn replaces_invalid_characters() {
        let fix = fix_component("a<b>c:d\"e|f?g*h").unwrap();
        assert_eq!(fix.value, "a_b_c_d_e_f_g_h");
    }

    #[test]
    fn protects_reserved_names() {
        assert_eq!(fix_component("CON").unwrap().value, "_CON");
        assert_eq!(fix_component("com1.txt").unwrap().value, "_com1.txt");
        assert!(fix_component("COM0").is_none());
    }

    #[test]
    fn rejects_unsafe_paths() {
        assert!(fix_relative_path("../outside.txt").is_err());
        assert!(fix_relative_path("/etc/passwd").is_err());
        assert!(fix_relative_path("C:/Windows/file.txt").is_err());
    }

    #[test]
    fn fixes_directory_components() {
        let fix = fix_relative_path("sub.../01.webp").unwrap();
        assert_eq!(fix.value, PathBuf::from("sub/01.webp"));
        assert!(fix.changed());
    }
}
