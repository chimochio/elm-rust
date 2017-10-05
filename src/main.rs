extern crate iron;
extern crate mount;
extern crate staticfile;

use std::path::Path;

use iron::prelude::*;
use mount::Mount;
use staticfile::Static;

fn main() {
    let mut mount = Mount::new();
    let static_html = Static::new(Path::new("build/index.html"));
    let static_js = Static::new(Path::new("build/js/"));
    mount
        .mount("/", static_html)
        .mount("/assets/", static_js);
        

    Iron::new(mount).http("localhost:4000").unwrap();
}
