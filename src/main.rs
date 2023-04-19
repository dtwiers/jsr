use ansi_term::Color::{Black, Blue, Cyan, Green, Purple, Red, Yellow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

fn try_dir(path: &Path) -> Option<PathBuf> {
    let current_file_path = path.join(Path::new("package.json"));
    if current_file_path.exists() {
        return Some(current_file_path);
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PackageManagerType {
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManagerType {
    fn detect_from_path(path: &Path) -> Option<PackageManagerType> {
        if path.with_file_name("package-lock.json").exists() {
            return Some(PackageManagerType::Npm);
        } else if path.with_file_name("yarn.lock").exists() {
            return Some(PackageManagerType::Yarn);
        } else if path.with_file_name("pnpm-lock.yaml").exists() {
            return Some(PackageManagerType::Pnpm);
        }
        None
    }
}
impl Display for PackageManagerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Npm => write!(f, "{}", "NPM"),
            Self::Yarn => write!(f, "{}", "Yarn"),
            Self::Pnpm => write!(f, "{}", "PNPM"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
struct PackageJson {
    name: String,
    scripts: Option<HashMap<String, String>>,
}

fn print_package_manager(path: &Path) {
    if let Some(package_manager) = PackageManagerType::detect_from_path(path) {
        println!(
            "{}",
            Blue.bold()
                .paint(format!("{} lockfile found", package_manager))
        );
    } else {
        println!("{}", Blue.bold().paint("No package manager found"));
    }
}

fn print_commands(path: &Path) {
    if let Ok(package_json_string) = read_to_string(path) {
        match serde_json::from_str::<PackageJson>(&package_json_string) {
            Ok(package_json_struct) => {
                if let Some(scripts) = package_json_struct.scripts {
                    println!("{}", Green.bold().paint("Available Commands"));
                    for (key, value) in scripts {
                        println!("{} {}", Cyan.bold().paint(key), Purple.bold().paint(value));
                    }
                } else {
                    println!("{}", Red.bold().paint("No commands found"))
                }
            }
            Err(err) => {
                println!("{}", Red.bold().paint("package.json is not valid JSON"));
                println!("{}", Red.paint(err.to_string()));
            }
        }
    } else {
        println!("{}", Red.bold().paint("package.json could not be read"))
    }
}

fn inspect_dir(path: &Path) {
    print_package_manager(path);
    print_commands(path);
}

fn main() {
    let current_dir = env::current_dir().expect("could not get current directory");
    if let Some(package_path) = current_dir.ancestors().find_map(try_dir) {
        println!(
            "{}",
            Black.bold().paint(format!(
                "{}",
                package_path
                    .parent()
                    .expect("foobar")
                    .to_str()
                    .expect("msg")
            ))
        );
        inspect_dir(&package_path);
    } else {
        println!("{}", Yellow.paint("No package.json found"));
    }
}
