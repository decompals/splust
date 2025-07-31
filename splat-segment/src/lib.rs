use splat_segment_api::Segment;
use splat_segment_api::bindings::export;

struct MySegment;

impl Segment for MySegment {
    fn testfunc(name: String) -> String {
        println!("STDIO WORKS!");
        format!("Greetings {name}! I'm a WASI plugin!")
    }
}
export!(MySegment with_types_in splat_segment_api::bindings);
