use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "gnew" {
        eprintln!("Usage: cargo gnew <project-name>");
        std::process::exit(1);
    }

    let generate_installed = Command::new("cargo")
        .arg("generate")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !generate_installed {
        println!("cargo-generate not found, installing...");
        let status = Command::new("cargo")
            .arg("install")
            .arg("cargo-generate")
            .status()
            .expect("failed to install cargo-generate");

        if !status.success() {
            eprintln!("failed to install cargo-generate");
            std::process::exit(1);
        }
    }

    let project_name = &args[2];
    let template_repo = "https://github.com/MedovTimur/template";

    let status = Command::new("cargo")
        .arg("generate")
        .arg("--git")
        .arg(template_repo)
        .arg("--name")
        .arg(project_name)
        .status()
        .expect("failed to execute cargo generate");

    if !status.success() {
        std::process::exit(1);
    }
}

