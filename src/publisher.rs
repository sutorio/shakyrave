use std::sync::OnceLock;
use tera::Tera;

use crate::config::Config;


pub fn get_templates() -> &'static Tera {
    static TEMPLATES: OnceLock<Tera> = OnceLock::new();

    TEMPLATES.get_or_init(|| {
        let conf = Config::get_static();

        let mut tera = match Tera::new(&conf.build.templates_folder) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    })
}

