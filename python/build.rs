use std::process::Command;

fn main() {
    if std::env::var_os("BOLT_QUADTREE_SKIP_PYTHON_LINK").is_some() {
        return;
    }

    let output = Command::new("python3-config")
        .arg("--embed")
        .arg("--ldflags")
        .output();
    let output = match output {
        Ok(output) if output.status.success() => output,
        Ok(output) => {
            eprintln!(
                "python3-config failed with status {}; stdout: {}; stderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        Err(err) => {
            eprintln!("failed to run python3-config: {err}");
            return;
        }
    };

    let flags = String::from_utf8_lossy(&output.stdout);
    for token in flags.split_whitespace() {
        if let Some(path) = token.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={path}");
        } else if let Some(lib) = token.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={lib}");
        }
    }
}
