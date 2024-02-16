use interface::{load_root_module_in_directory, ExampleSeg_Ref};

fn main() {
    // The type annotation is for the reader
    let segment: ExampleSeg_Ref =
        load_root_module_in_directory("target/debug".as_ref()).unwrap_or_else(|e| panic!("{}", e));

    {
        let response = segment;
    }
    println!("success");
}
