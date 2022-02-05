use oas3;
use oas3::OpenApiV3Spec;
use serde_json;
use std::collections::BTreeMap;
use std::env;

fn main() {
    println!("Running program {}", env::args().nth(0).unwrap());
    let path = env::args().nth(1).expect("No input openapi file specified");
    let spec = match oas3::from_path(path) {
        Ok(sp) => sp,
        Err(err) =>  {
            println!("error: {}", err);
            panic!();
        }
    };

    println!("openapi Version: {}", spec.openapi);
    let paths = spec.paths;

    let num_paths = paths.len();
    println!("Number of paths = {}", num_paths);

    let mut path_index = 0;
    //  Iterate through the paths and go through the responses.
    for (name, path_obj) in paths.iter() {
        println!("{}", name);
        let get = match &path_obj.get {
            Some(get_obj) => get_obj,
            None => continue
        };
        let responses = &get.responses;
        for (code, resp_obj) in responses.iter() {
            println!("\tResponse code {}", code);
        }       
//        if 0 == path_index {
//            println!("{}", serde_json::to_string_pretty(&path_obj).unwrap());
//        }
        path_index = path_index + 1;
    }
}
