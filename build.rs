use glib_build_tools;

fn main() {
    glib_build_tools::compile_resources(
        &["src"],
        "src/furtherance.gresource.xml",
        "furtherance.gresource",
    );
}