use std::collections::HashMap;
use std::borrow::Cow;

pub struct SettingsBuilder {
    variables: HashMap<String, String>,
    templates: HashMap<String, String>
}

impl SettingsBuilder {
    pub fn var(mut self, name: &str, value: &str) -> Self {
        self.variables.insert(name.to_owned(), value.to_owned());
        self
    }

    pub fn template(mut self, name: &str, value: &str) -> Self {
        self.templates.insert(name.to_owned(), value.to_owned());
        self
    }

    pub fn build(self) -> Settings {
        Settings {
            variables: self.variables,
            templates: self.templates
        }
    }
}

pub struct Settings {
    variables: HashMap<String, String>,
    templates: HashMap<String, String>
}

impl Settings {
    pub fn new() -> Settings {
        Settings::builder().build()
    }

    pub fn builder() -> SettingsBuilder {
        let mut templates = HashMap::new();

        for var in std::env::vars() {
            if var.0.to_lowercase().starts_with("showengine_default_template_") {
                let template_name = var.0.to_lowercase().replace("showengine_default_template_", "");
                let template_value = var.1.to_lowercase();
                templates.insert(template_name, template_value);
            }
        }

        SettingsBuilder {
            variables: HashMap::new(),
            templates
        }
    }

    pub fn set_var(&mut self, name: &str, value: &str) -> &Self {
        self.variables.insert(name.to_owned(), value.to_owned());
        self
    }

    pub fn var(&self, name: &str) -> Option<Cow<String>> {
        self.variables.get(name).map(|v| Cow::Borrowed(v))
    }

    pub fn set_template(&mut self, name: &str, value: &str) -> &Self {
        self.templates.insert(name.to_owned(), value.to_owned());
        self
    }

    pub fn template(&self, name: &str) -> Option<Cow<String>> {
        self.templates.get(name).map(|v| Cow::Borrowed(v))
    }
}