#![allow(dead_code)]
use crate::config::get_tproj_home;
use regex::Regex;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

/// 列出数据目录下的归档文件。
/// 这个方法向 stdout 输出，为命令行工具接口
pub fn cli_list<P: AsRef<str> + Display>(pattern: P, verbose: bool) -> Result<(), std::io::Error> {
    dbg!(getpath(pattern)?);
    Ok(())
}

/// 获取满足 pattern 的归档文件路径
pub fn getpath<P: AsRef<str> + Display>(pattern: P) -> Result<Vec<PathBuf>, std::io::Error> {
    let pattern = format!("{}.zip", pattern);
    let pattern = Regex::from_str(pattern.as_str()).expect("构造正则表达式失败");
    let home = get_tproj_home();
    let adir = home.join("template");
    let entries: Vec<PathBuf> = fs::read_dir(adir)?
        .map(|res| res.map(|e: fs::DirEntry| e.path()))
        .filter(|p| pattern.is_match(&p.as_ref().unwrap().file_name().unwrap().to_str().unwrap()))
        .collect::<Result<Vec<PathBuf>, std::io::Error>>()?;
    return Ok(entries);
}
