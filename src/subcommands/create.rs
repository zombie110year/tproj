#![allow(dead_code)]
use crate::config::{get_tproj_home, TprojConfig};
use crate::{TEMPLATE_DIR, YML_NAME};
use std::fs;
use std::io;
use std::io::Write;
use std::path;
use zip;

/// 根据已有的 tproj.yml 配置创建一个模板归档
pub fn cli_create() {
    let tpc = get_yml_conf();
    let name = &tpc.name;
    let files = &tpc.includes;
    let target_path = get_tproj_home()
        .join(TEMPLATE_DIR)
        .join(format!("{}.zip", name));
    let target_path = path::Path::new(&target_path);
    let mut zipfile = fs::File::create(target_path).expect("无法创建归档文件");
    let writer = io::BufWriter::new(&mut zipfile);
    let option =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut zwriter = zip::ZipWriter::new(writer);
    for filename in files {
        zwriter.start_file(filename, option).expect("创建文件失败");
        let fs_file = fs::read(filename).expect(format!("无法读取源文件 {}", filename).as_str());
        zwriter.write(&fs_file).expect("写入文件失败");
    }
}

/// 读取当前目录的 tproj.yml 配置
fn get_yml_conf() -> TprojConfig {
    let fp = path::Path::new(YML_NAME);
    let fd = fs::File::open(fp).expect("无法打开 tproj.yml 文件");
    let fr = io::BufReader::new(fd);
    let yml: TprojConfig = serde_yaml::from_reader(fr).expect("tproj.yml 读取错误");
    return yml;
}
