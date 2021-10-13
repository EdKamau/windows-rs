use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // let output = std::env::args()
    //     .nth(1)
    //     .expect("Expected one command line argument for output directory");
    let output = r#"c:\git\windows-apis-rs\crates"#;

    let output = std::path::Path::new(&output);
    let _ = std::fs::remove_dir_all(output);

    let mut path = std::path::PathBuf::from(output);
    std::fs::create_dir_all(&path)?;
    path.push("publish.bat");
    let mut file = std::fs::File::create(&path)?;

    let crates = namespaces();

    for namespace in &crates {
        let crate_name = namespace.replace('.', "-").to_lowercase();
        println!("{}", crate_name);

        gen_crate(
            &output,
            &crate_name,
            namespace,
            &crates,
            env!("CARGO_PKG_VERSION"),
        )?;

        file.write_all(
            format!(
                "cargo publish --no-verify --manifest-path {}\\Cargo.toml\n",
                crate_name
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}
 
fn namespaces() -> std::collections::BTreeSet<&'static str> {
    let mut set = std::collections::BTreeSet::new();

    for namespace in reader::TypeReader::get().namespaces() {
        if !namespace.starts_with("Windows.") {
            continue;
        }

        if let Some(first) = namespace.find('.') {
            if let Some(second) = namespace[first + 1..].find('.') {

                // Win32 APIs are further partitioned. 
                if namespace.starts_with("Windows.Win32.") {
                    if let Some(third) = namespace[first + 1 + second + 1..].find('.') {
                        set.insert(&namespace[..first + 1 + second + 1 + third]);    
                    } else {
                        set.insert(namespace);
                    }
                } else {
                    set.insert(&namespace[..first + 1 + second]);
                }
            } else {
                set.insert(namespace);
            }
        }
    }

    // Windows.UI.Xaml should be distinct from Windows.UI as it is so large.
    set.insert("Windows.UI.Xaml");
    set
}

fn dependencies(module: &str) -> Vec<&'static str> {
    let mut dependencies = Vec::new();

    fn walk(namespaces: & std::collections::BTreeMap<&'static str, reader::TypeTree>, module: &str, dependencies: &mut Vec<&'static str>) {
        for tree in namespaces.values() {
            if !tree.include {
                continue;
            }
            
            if !gen::contains_namespace(module, tree.namespace) {
                dependencies.push(tree.namespace);
            }

            walk(&tree.namespaces, module, dependencies);
        }
    }

    walk(&reader::TypeReader::get().types.namespaces, module, &mut dependencies);
    dependencies
}

fn gen_crate(
    output: &std::path::Path,
    crate_name: &str,
    module: &'static str,
    crates: &std::collections::BTreeSet<&'static str>,
    version: &str,
) -> std::io::Result<()> {
    let mut path = std::path::PathBuf::from(output);
    path.push(&crate_name);
    std::fs::create_dir_all(&path)?;
    path.push("Cargo.toml");
    let mut file = std::fs::File::create(&path)?;

    let reader = reader::TypeReader::get_mut();
    let mut namespaces = Vec::new();

    for namespace in reader.namespaces() {
        if gen::contains_namespace(module, namespace) {
            //println!("- {}", namespace);
            namespaces.push(namespace);
        }
    }

    reader.clear_imports();

    for namespace in &namespaces {
        // TODO: use import here so we can track dependencies
        reader.import_namespace(namespace);
    }

    let tree = gen::gen_crate_source_tree(module, &namespaces, crates);

    // TODO: pin the windows crate dependency to the same "version" so everything is versioned in lockstep.
    // Currently its "0.21" to ease development.
    file.write_all(
        format!(
            r#"
[package]
name = "{}"
version = "{}"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "{}"

[dependencies]
windows = {{ version = "0.21", default-features = false }}"#,
            crate_name, version, module
        )
        .as_bytes(),
    )?;

    for dependency in dependencies(module) {
        if crates.contains(dependency) {
            let crate_name = dependency.replace('.', "-").to_lowercase();

            file.write_all(
                format!(
                    r#"
{} = {{ path = "../{}", version = "{}" }}"#,
                    crate_name, crate_name, version
                ).as_bytes()
            )?;
        }
    }

    // TODO: write features and dependencies including feature dependencies

    path.pop();
    path.push("src");
    std::fs::create_dir_all(&path)?;
    path.push("lib.rs");
    let mut file = std::fs::File::create(&path)?;

    // TODO: add helper function to do this for all generated code
    file.write_all(
        r#"// This file was generated by the `windows` crate - do not edit by hand!
#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
"#.as_bytes(),
    )?;

    file.write_all(tree.into_string().as_bytes())?;
    drop(file);
    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;
    Ok(())
}