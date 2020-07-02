use crate::stdweb::js_export;
use crate::stdweb::js;

#[js_export]
pub fn main() {
    // std::compile_error!(concat!("Target = ", include_str!(concat!(env!("OUT_DIR"), "/../output"))));
    let message = "Hello, 世界!";
    let result = js! {
        alert( @{message} );
        return 2 + 2 * 2;
    };

    println!( "2 + 2 * 2 = {:?}", result );
}
