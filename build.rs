use std::{
    env,
    fs,
    path::PathBuf,
    process::Command,
};
use std::path::Path;
use glib_build_tools;

fn compile_gsettings_schema() {
    #[cfg(not(target_os = "windows"))]
    let schema_dir = PathBuf::from(env!("HOME")).join(".local/share/glib-2.0/schemas");

    #[cfg(target_os = "windows")]
    let schema_dir = Path::new("C:/ProgramData/glib-2.0/schemas");

    // Create the schema directory if it doesn't exist
    fs::create_dir_all(&schema_dir).expect("Failed to create schema directory");

    // Copy the gschema.xml file to the schema directory
    let gschema_source = Path::new("data").join("com.lakoliu.Furtherance.gschema.xml");
    let gschema_dest = schema_dir.join(gschema_source.file_name().unwrap());
    fs::copy(gschema_source, &gschema_dest).expect("Failed to copy gschema.xml");

    // Compile the schemas
    let compile_schemas_command = Command::new("glib-compile-schemas")
        .arg(&schema_dir)
        .output()
        .expect("Failed to execute glib-compile-schemas");

    if !compile_schemas_command.status.success() {
        eprintln!(
            "Error compiling schemas: {}",
            String::from_utf8_lossy(&compile_schemas_command.stderr)
        );
        std::process::exit(1);
    }
}


fn main() {
    // Compile resources
    glib_build_tools::compile_resources(
        &["src"],
        Path::new("src").join("furtherance.gresource.xml").to_str().unwrap(),
        "furtherance.gresource",
    );

    compile_gsettings_schema()
}
