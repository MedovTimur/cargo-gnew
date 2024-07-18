use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "gnew" {
        eprintln!("Usage: cargo gnew <project-name>");
        std::process::exit(1);
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
