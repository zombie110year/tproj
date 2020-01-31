use serde_yaml;
use tproj::TprojConfigBuilder;

fn main() {
    let mut br = TprojConfigBuilder::new();
    let br = br
        .nameit("Example".to_string())
        .auth("zombie110year".to_string())
        .include("./main.rs".to_string())
        .exclude("./.git/".to_string())
        .describe("这是一个示例".to_string());
    let obj = br.build();
    let msg = serde_yaml::to_string(&obj).unwrap();
    print!("{}", msg);
}
