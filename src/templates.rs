extern crate mustache;
extern crate lazy_static;

use mustache::Data;
use iron::modifiers;
use iron::headers::ContentType;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    pub static ref TEMPLATE_CACHE: RwLock<TemplateCache> = RwLock::new(TemplateCache { cache: HashMap::new() });
}

pub struct TemplateCache {
    cache: HashMap<String, mustache::Template>
}

impl TemplateCache {
    fn template_for(&self, file: &String) -> Option<&mustache::Template> {
        self.cache.get(file)
    }

    fn load_template(&mut self, file: &String) {
        let filepath = format!("templates/{}", file);
        info!(target: "mustache", "Loading template for {}", file);
        let t = mustache::compile_path(filepath.to_owned())
            .expect(&format!("Could not load {}", filepath));
        debug!(target: "mustache", "Loaded template for {}: {:?}", filepath, t);
        self.cache.insert(file.to_owned(), t);
    }
}

pub fn template(filename: String, context: Data) -> (String, modifiers::Header<ContentType>) {
    let h = TEMPLATE_CACHE.read().unwrap();
    let tmpl = h.template_for(&filename);
    let t = match tmpl {
        None => {
            let mut w = TEMPLATE_CACHE.write().unwrap();
            w.load_template(&filename);
            h.template_for(&filename).expect("Template insertion failed!")
        }
        Some(x) => x
    };
    let mut v = Vec::new();
    t.render_data(&mut v, &context);

    (String::from_utf8(v).unwrap(), modifiers::Header(ContentType::html()))
}
