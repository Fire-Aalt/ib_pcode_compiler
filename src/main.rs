#[cfg(not(target_arch = "wasm32"))]
const SOURCE: &str = "source";

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut contents = std::fs::read_to_string(SOURCE).expect("Should have been able to read the file");
        contents.push('\n');

        ib_pseudocompiler::compile_release_and_run(contents.as_str());
    }
    #[cfg(target_arch = "wasm32")]
    {
        ib_pseudocompiler::ensure_link();
    }
}
