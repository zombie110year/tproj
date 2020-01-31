#![cfg(test)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tproj::TprojConfigBuilder;

#[test]
fn test_read_full() {
    let mut br = TprojConfigBuilder::new();
    br.nameit("HelloWorld")
        .auth("zombie110year")
        .include("README.md")
        .include("FILE0.txt")
        .exclude(".git/**")
        .exclude(".ignored")
        .describe("描述此项目\n");
    let obj_exp = br.build();
    let src = File::open(Path::new("yaml-tests/full.yml")).unwrap();
    let reader = BufReader::new(src);
    let obj_got = serde_yaml::from_reader(reader).unwrap();
    assert_eq!(obj_exp, obj_got);
}

#[test]
fn test_read_no_description() {
    let mut br = TprojConfigBuilder::new();
    br.nameit("HelloWorld")
        .auth("zombie110year")
        .include("README.md")
        .include("FILE0.txt")
        .exclude(".git/**")
        .exclude(".ignored");
    let obj_exp = br.build();
    let src = File::open(Path::new("yaml-tests/no-description.yml")).unwrap();
    let reader = BufReader::new(src);
    let obj_got = serde_yaml::from_reader(reader).unwrap();
    assert_eq!(obj_exp, obj_got);
}

#[test]
fn test_read_no_excludes() {
    let mut br = TprojConfigBuilder::new();
    br.nameit("HelloWorld")
        .auth("zombie110year")
        .include("README.md")
        .include("FILE0.txt")
        .describe("描述此项目\n");
    let obj_exp = br.build();
    let src = File::open(Path::new("yaml-tests/no-excludes.yml")).unwrap();
    let reader = BufReader::new(src);
    let obj_got = serde_yaml::from_reader(reader).unwrap();
    assert_eq!(obj_exp, obj_got);
}
