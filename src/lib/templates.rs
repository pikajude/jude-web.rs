extern crate mustache;
extern crate lazy_static;

use mustache::Data;
use iron::modifiers;
use iron::headers::ContentType;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref TEMPLATE_CACHE: RwLock<TemplateCache> = RwLock::new(TemplateCache { cache: HashMap::new() });
}

pub struct TemplateCache {
    cache: HashMap<String, mustache::Template>
}

impl TemplateCache {
    fn has_template(&self, file: &String) -> bool {
        self.cache.contains_key(file)
    }

    fn template_for(&self, file: &String) -> &mustache::Template {
        self.cache.get(file).expect(&format!("template_for({}) missing", file))
    }

    fn load_template(&mut self, file: &String) {
        let filepath = format!("templates/{}.mst", file);
        info!(target: "mustache", "Loading template for {}", file);
        let t = mustache::compile_path(filepath.to_owned())
            .expect(&format!("Could not load {}", filepath));
        debug!(target: "mustache", "Loaded template for {}: {:?}", filepath, t);
        self.cache.insert(file.to_owned(), t);
    }
}

pub fn template(filename: String, context: Data) -> (String, modifiers::Header<ContentType>) {
    let render = |tmpl: &mustache::Template| {
        let mut v = Vec::new();
        tmpl.render_data(&mut v, &context);

        (String::from_utf8(v).unwrap(), modifiers::Header(ContentType::html()))
    };

    {
        let h = TEMPLATE_CACHE.read().unwrap();
        if h.has_template(&filename) {
            return render(h.template_for(&filename))
        }
    }
    {
        let mut w = TEMPLATE_CACHE.write().unwrap();
        w.load_template(&filename);
        let t = w.template_for(&filename);
        render(t)
    }
}
