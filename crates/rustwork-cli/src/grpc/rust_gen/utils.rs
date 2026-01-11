use std::fmt::Write;

/// Convertit un nom PascalCase en snake_case
pub(crate) fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && prev_is_lower {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_is_lower = false;
        } else {
            result.push(c);
            prev_is_lower = c.is_lowercase();
        }
    }

    result
}

/// Génère le fichier mod.rs pour le module grpc
pub fn generate_grpc_mod(service_names: &[String]) -> Result<String, std::fmt::Error> {
    let mut output = String::new();

    writeln!(output, "// Généré automatiquement par Rustwork")?;
    writeln!(output, "#![allow(unused_imports)]")?;
    writeln!(output)?;

    // Importer chaque service
    for service_name in service_names {
        let module_name = to_snake_case(service_name);
        writeln!(output, "pub mod {};", module_name)?;
    }

    writeln!(output)?;
    writeln!(output, "// Ré-exports pour faciliter l'utilisation")?;
    for service_name in service_names {
        let module_name = to_snake_case(service_name);
        writeln!(output, "pub use {}::*;", module_name)?;
    }

    Ok(output)
}
