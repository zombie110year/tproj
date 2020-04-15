use crate::config::get_tproj_home;
use crate::TEMPLATE_DIR;
use std::fs;
use std::io;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::path::PathBuf;

/// 根据输入的归档名，将归档在指定目录展开
pub fn cli_apply<'a, S: Into<&'a str>>(name: S, dest: S) {
    let name: &'a str = name.into();
    let dest: &'a str = dest.into();
    let home = get_tproj_home();
    let template = if name.ends_with(".zip") {
        home.join(TEMPLATE_DIR).join(name)
    } else {
        home.join(TEMPLATE_DIR)
            .join(format!("{}.zip", name).as_str())
    };
    let zipfile = fs::File::open(&template).expect("没有指定的归档");
    let mut zipreader = zip::ZipArchive::new(zipfile).expect("无法读取归档");
    for i in 0..zipreader.len() {
        let mut entry = zipreader.by_index(i).expect("无法获取项目");
        if entry.is_file() {
            let path: PathBuf = entry.sanitized_name();
            {
                let path = PathBuf::from(dest).join(&path);
                let parent = path.parent().unwrap();
                if !parent.exists() {
                    fs::create_dir_all(parent).expect("无法创建目录");
                }
                let mut outfile = fs::File::create(path).expect("无法创建文件");
                io::copy(&mut entry, &mut outfile).expect("无法写入文件");
            }
            println!("file: {}", path.to_str().unwrap());
        } else if entry.is_dir() {
            let path: PathBuf = entry.sanitized_name();
            {
                let path = PathBuf::from(dest).join(&path);
                if !path.exists() {
                    fs::create_dir_all(path).expect("无法创建目录");
                }
            }
            println!("file: {}", path.to_str().unwrap());
        }
    }
}
