// Parse config files

use super::error;
use std::{collections::HashMap, fs, path::Path};


// Remove one pair of quotes from a string, if present
fn remove_quotes<'a>(data: &'a str) -> &'a str {
    match data.chars().next() {
        Some(c) if "'\"".contains(c) => {
            if data.ends_with(c) {
                return &data[1..data.len() - 1];
            }
            else {
                return data;
            }
        },
        _ => data
    }
}


#[derive(Debug)]
pub struct ConfigSection {
    pub name: String,
    pub keys: HashMap<String, String>
}


struct SectionBuilder(ConfigSection);

impl SectionBuilder {
    pub fn new<S: Into<String>>(name: S) -> Self {
        SectionBuilder(ConfigSection { name: name.into(), keys: HashMap::new() })
    }

    pub fn add_keypair<S: Into<String>>(&mut self, key: S, value: S) {
        self.0.keys.insert(key.into(), value.into());
    }

    pub fn build(self) -> ConfigSection {
        self.0
    }
}


#[derive(Debug)]
pub struct ConfigFile(Vec<ConfigSection>);

impl ConfigFile {
    pub fn read<P: AsRef<Path>>(path: P) -> error::Result<Self> {
        let source = fs::read_to_string(path)?;
        Self::parse_source(&source)
    }

    pub fn sections(&self) -> &[ConfigSection] {
        &self.0
    }

    fn parse_source(source: &str) -> error::Result<Self> {
        let mut cfg = ConfigFile(vec![]);
        let mut section: Option<SectionBuilder> = None;

        for line in source.lines() {
            let line = line.split_once('#').unwrap_or((line, "")).0; // Strip comments

            if line.starts_with('[') && line.ends_with(']') {
                if let Some(s) = section.take() {
                    cfg.0.push(s.build());
                }

                section = Some(SectionBuilder::new(&line[1..line.len() - 1]));
            }
            else if !line.is_empty() {
                match section {
                    Some(ref mut s) => {
                        let (key, val) = line
                            .split_once(':')
                            .unwrap_or(line.split_once("->").unwrap_or((line, "")));

                        let key = remove_quotes(key.trim());
                        let val = remove_quotes(val.trim());

                        s.add_keypair(key, val);
                    },
                    None => return Err(error::Error::new(error::ErrorKind::InvalidKey, "Expected section, found key instead"))
                }
            }
        }

        Ok(cfg)
    }
}
