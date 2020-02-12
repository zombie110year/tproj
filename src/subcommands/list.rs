#![allow(dead_code)]
use crate::config::get_tproj_home;
use crate::config::TprojConfig;
use regex::Regex;
use std::fmt::Display;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use zip::ZipArchive;

/// 列出数据目录下的归档文件。
/// 这个方法向 stdout 输出，为命令行工具接口
pub fn cli_list<P: AsRef<str> + Display>(pattern: P, verbose: bool) -> Result<(), std::io::Error> {
    let paths = get_path(pattern)?;
    for p in paths {
        let desc = get_description(&p).unwrap_or_default();
        let stem = p.file_stem().unwrap().to_str().unwrap();
        println!("{}: {}", stem, desc)
    }
    Ok(())
}

/// 获取满足 pattern 的归档文件路径
pub fn get_path<P: AsRef<str> + Display>(pattern: P) -> Result<Vec<PathBuf>, std::io::Error> {
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

/// 获取指定归档文件中的描述
pub fn get_description(p: &PathBuf) -> Option<String> {
    let p = std::path::Path::new(p);
    let file = std::fs::File::open(&p).expect("无法打开 zip 文件");
    let mut zfile = ZipArchive::new(file).expect("无法解析为可读的 zip 归档");
    let yml = zfile.by_name("tproj.yml").expect("无法打开 tproj.yml");
    let obj: TprojConfig = serde_yaml::from_reader(yml).expect("无法读取或解析 tproj.yml");
    return obj.description;
}
