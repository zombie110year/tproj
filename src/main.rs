use serde_yaml;
use tproj::config::TprojConfigBuilder;

fn main() {
    let mut br = TprojConfigBuilder::new();
    let br = br
        .nameit("Example")
        .auth("zombie110year")
        .include("./main.rs")
        .exclude("./.git/")
        .describe("这是一个示例");
    let obj = br.build();
    let msg = serde_yaml::to_string(&obj).unwrap();
    print!("{}", msg);
}
