#[macro_use]
extern crate clap;
use clap::App;
use tproj::subcommands::list;
use tproj::subcommands::create;
use tproj::subcommands::apply;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let cli = App::from_yaml(yaml);
    let args = cli.get_matches();
    match args.subcommand_name() {
        Some("apply") => {
            let subargs = args
                .subcommand_matches("apply")
                .expect("未获取到 apply 子命令");
            let aname = subargs.value_of("aname").expect("未获取到 aname 参数");
            let dest = subargs.value_of("dest").expect("未获取到目标文件夹");
            apply::cli_apply(aname, dest);
        }
        Some("create") => {
            let subargs = args
                .subcommand_matches("create")
                .expect("未获取到 create 子命令 ");
            let _config = subargs.value_of("config").expect("未获取到 config 参数");
            create::cli_create();
        }
        Some("list") => {
            let subargs = args
                .subcommand_matches("list")
                .expect("未获取到 list 子命令");
            let pattern = subargs.value_of("pattern").expect("未获取到 pattern 参数");
            let verbose = subargs.is_present("verbose");
            list::cli_list(pattern, verbose)
                .expect("list 指令执行失败");
        }
        Some(_) => panic!("预期外的子命令"),
        None => panic!("?"),
    };
    return ();
}
