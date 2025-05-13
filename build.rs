fn main() {
    #[cfg(feature = "esp")]
    {
        embuild::espidf::sysenv::output();
    }

    let mut gen = micropb_gen::Generator::new();

    gen.use_container_std();

    let proto_files = find_proto_files();

    // Compile all found proto files
    if !proto_files.is_empty() {
        gen.add_protoc_arg("--proto_path=./src");

        // Keep the proto path as is, but modify the file paths
        gen.compile_protos(&proto_files, "src/protos.rs").unwrap();
    }
}

fn find_proto_files() -> Vec<String> {
    let mut proto_files = Vec::new();

    // print current directory
    println!(
        "Current directory: {}",
        std::env::current_dir().unwrap().display()
    );

    if let Ok(entries) = std::fs::read_dir("./src") {
        for entry in entries.flatten() {
            search_proto_dirs(&entry.path(), &mut proto_files);
        }
    }

    proto_files
}

fn search_proto_dirs(path: &std::path::Path, proto_files: &mut Vec<String>) {
    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() && entry_path.extension().is_some_and(|ext| ext == "proto")
                {
                    if let Some(path_str) = entry_path.to_str() {
                        proto_files.push(path_str.to_string());
                    }
                }
            }
        }

        // Continue searching in subdirectories
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    search_proto_dirs(&entry.path(), proto_files);
                }
            }
        }
    }
}
