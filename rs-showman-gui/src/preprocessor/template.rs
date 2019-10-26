use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use regex::{Regex, Match};

use super::get_template;
use super::token::Token;
use super::settings::Settings;

pub struct TemplateVariant {
    tokens: Vec<Token>
}

pub struct Template {
    variants: HashMap<String, TemplateVariant>
}

impl TemplateVariant {
    pub fn from_file<P>(filename: P) -> Result<TemplateVariant, std::io::Error>
        where
            P: AsRef<Path>
    {
        let contents = std::fs::read_to_string(filename)?;
        Ok(TemplateVariant::from_string(contents))
    }

    pub fn from_string(contents: String) -> TemplateVariant {
        let regex = Regex::new(r#"<\?rs (\w+)::(.*?) \?>"#).unwrap();

        let mut tokens = Vec::new();
        let mut last_match: Option<Match> = None;

        for token in regex.captures_iter(&contents) {
            let full_match = token.get(0).unwrap();
            let type_match = token.get(1).unwrap();
            let value_match = token.get(2).unwrap();
            if let Some(m) = last_match {
                tokens.push(Token::text(&contents[m.end()..full_match.start()]));
            } else {
                tokens.push(Token::text(&contents[0..full_match.start()]));
            }

            match type_match.as_str() {
                "var" => tokens.push(Token::variable(value_match.as_str())),
                "env" => tokens.push(Token::Text(std::env::var(value_match.as_str()).unwrap_or(String::new()))),
                "template" => tokens.push(Token::template(value_match.as_str())),
                _ => {}
            }

            last_match = Some(token.get(0).unwrap());
        }

        if let Some(m) = last_match {
            tokens.push(Token::text(&contents[m.end()..]));
        } else {
            tokens.push(Token::text(&contents));
        }

        TemplateVariant { tokens }
    }

    pub fn get(&self, settings: &Settings) -> String {
        let mut result = String::new();

        for token in self.tokens.iter() {
            match token {
                Token::Text(text) => result += text,
                Token::Variable(var) => result += settings.var(var).unwrap_or(Cow::Owned(String::new())).as_str(),
                Token::Template(template) => result += &get_template(template, settings).unwrap_or(String::new())
            }
        }

        result
    }
}

impl Template {
    pub fn new() -> Template {
        Template {
            variants: HashMap::new()
        }
    }

    pub fn set<P>(&mut self, variant: &str, filename: P) -> Result<(), std::io::Error>
        where
            P: AsRef<Path>
    {
        self.variants.insert(variant.to_owned(), TemplateVariant::from_file(filename)?);
        Ok(())
    }

    pub fn get(&self, name: &str, settings: &Settings) -> String {
        let variant = settings.template(name)
            .and_then(|v| self.variants.get(v.as_ref()))
            .or_else(|| self.variants.get("default"));
        if let Some(variant) = variant {
            variant.get(settings)
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::TemplateVariant;
    use super::Settings;

    #[test]
    fn test_correctness() {
        let template = TemplateVariant::from_string(r#"<div>Hello, I am <?rs var::name ?> <?rs var::surname ?>.</div>"#.to_owned());
        let settings = Settings::builder()
            .var("name", "John")
            .var("surname", "Smith")
            .build();

        assert_eq!(template.get(&settings), r#"<div>Hello, I am John Smith.</div>"#);
    }
}