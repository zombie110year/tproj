#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// 项目配置文件名
pub const PROJ_CONF_FILENAME: &'static str = "tproj.yml";

/// TPROJ 使用的数据、配置目录
///
/// 按优先级排列：
///
/// 1. 环境变量 `TPROJ_HOME` 配置的路径
/// 2. `dirs::data_dir()/tproj` 路径：
///     - Linux: `$XDG_DATA_HOME/tproj`、`~/.local/share/tproj`
///     - Windows: `$APPDATA/tproj`
///     - MacOS: `$HOME/Library/Application/tproj`
pub fn get_tproj_home() -> std::path::PathBuf {
    let env_datadir: std::path::PathBuf = match std::env::var("TPROJ_HOME") {
        Ok(dir) => std::path::PathBuf::from(dir),
        Err(_) => dirs::data_dir().expect("无法获取到 DATA 目录路径"),
    };
    env_datadir.join("tproj")
}

/// 构造 TprojConfig 结构
///
/// 可用方法：
///
/// - nameit: 设置 name 字段
/// - auth: 设置 author 字段
/// - include: 添加一个 include 项目
/// - exclude: 添加一个 exclude 项目
/// - describe: 设置 description 字段
///
/// ```rust
/// let mut br = TprojConfigBuilder::new();
/// let br = br
///     .nameit("Example")
///     .auth("zombie110year")
///     .include("./main.rs")
///     .exclude("./.git/")
///     .describe("这是一个示例");
/// let obj = br.build();
/// let msg = serde_yaml::to_string(&obj).unwrap();
/// print!("{}", msg);
/// ```
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
    pub fn nameit<S: ToString>(&mut self, name: S) -> &mut Self {
        match &mut self.name {
            None => {
                self.name = Some(name.to_string());
            }
            Some(exname) => {
                *exname = name.to_string();
            }
        }
        return self;
    }
    pub fn auth<S: ToString>(&mut self, author: S) -> &mut Self {
        self.authors.push(author.to_string());
        return self;
    }
    pub fn include<S: ToString>(&mut self, pattern: S) -> &mut Self {
        self.includes.push(pattern.to_string());
        return self;
    }
    pub fn exclude<S: ToString>(&mut self, pattern: S) -> &mut Self {
        let pat = pattern.to_string();
        match &mut self.excludes {
            None => {
                self.excludes = Some(vec![pat]);
            }
            Some(exarr) => {
                exarr.push(pat);
            }
        };
        return self;
    }
    pub fn describe<S: ToString>(&mut self, description: S) -> &mut Self {
        self.description = Some(description.to_string());
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
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
