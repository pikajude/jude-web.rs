extern crate mustache;
extern crate lazy_static;

use mustache::Data;
use iron::modifiers;
use iron::headers::ContentType;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    pub static ref TEMPLATE_CACHE: RwLock<TemplateCache> = {
        let mut h = HashMap::new();
        RwLock::new(TemplateCache { cache: h })
    };
}

pub struct TemplateCache {
    cache: HashMap<String, mustache::Template>
}

impl TemplateCache {
    fn template_for(&mut self, file: String) -> &mut mustache::Template {
        self.cache.entry(file.clone()).or_insert_with(|| {
            let filepath = format!("templates/{}", file);
            info!(target: "mustache", "Loading template for {}", file);
            let t = mustache::compile_path(filepath.clone())
                .expect(&format!("Could not load {}", filepath));
            debug!(target: "mustache", "Loaded template for {}: {:?}", filepath, t);
            t
        })
    }
}

pub fn template(filename: String, context: Data) -> (String, modifiers::Header<ContentType>) {
    let mut h = TEMPLATE_CACHE.write().unwrap();
    let tmpl = h.template_for(filename);
    let mut v = Vec::new();
    tmpl.render_data(&mut v, &context);

    (String::from_utf8(v).unwrap(), modifiers::Header(ContentType::html()))
}
