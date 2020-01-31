#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// 项目配置文件名
pub const PROJ_CONF_FILENAME: &'static str = "tproj.yml";

pub struct TprojConfigBuilder {
    name: Option<String>,
    authors: Vec<String>,
    includes: Vec<String>,
    excludes: Option<Vec<String>>,
    description: Option<String>,
}

impl TprojConfigBuilder {
    pub fn new() -> Self {
        let authors: Vec<String> = Vec::new();
        let includes: Vec<String> = Vec::new();
        return TprojConfigBuilder {
            name: None,
            authors,
            includes,
            excludes: None,
            description: None,
        };
    }
    pub fn nameit(&mut self, name: String) -> &mut Self {
        match &mut self.name {
            None => {
                self.name = Some(name);
            }
            Some(exname) => {
                *exname = name;
            }
        }
        return self;
    }
    pub fn auth(&mut self, author: String) -> &mut Self {
        self.authors.push(author);
        return self;
    }
    pub fn include(&mut self, pattern: String) -> &mut Self {
        self.includes.push(pattern);
        return self;
    }
    pub fn exclude(&mut self, pattern: String) -> &mut Self {
        match &mut self.excludes {
            None => {
                self.excludes = Some(vec![pattern]);
            }
            Some(exarr) => {
                exarr.push(pattern);
            }
        };
        return self;
    }
    pub fn describe(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        return self;
    }
    pub fn build(&self) -> TprojConfig {
        let name: String = match &self.name {
            None => "default".to_string(),
            Some(exname) => exname.clone(),
        };
        TprojConfig {
            name,
            authors: self.authors.clone(),
            includes: self.includes.clone(),
            excludes: self.excludes.clone(),
            description: self.description.clone(),
        }
    }
}

///
/// TprojConfig 是与 tproj.yml 相对应的数据结构
///
/// 由以下键值组成：
///
/// name
/// :   模板的名字。如果不存在的
#[derive(Serialize, Deserialize)]
pub struct TprojConfig {
    name: String,
    authors: Vec<String>,
    includes: Vec<String>,
    excludes: Option<Vec<String>>,
    description: Option<String>,
}

impl TprojConfig {
    pub fn new(
        name: String,
        authors: Vec<String>,
        includes: Vec<String>,
        excludes: Option<Vec<String>>,
        description: Option<String>,
    ) -> Self {
        TprojConfig {
            name,
            authors,
            includes,
            excludes,
            description,
        }
    }
}
