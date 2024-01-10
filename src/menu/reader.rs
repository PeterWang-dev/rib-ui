use crate::common::{borrow, self_info};
use rib_backend::state::State;

pub async fn menu(state: &State) {
    loop {
        println!("读者菜单：");
        println!("1. 个人信息");
        println!("2. 借阅管理");
        println!("0. 退出");
        println!("请选择：");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => self_info::menu(state).await,
            "2" => borrow::management_menu(state).await,
            "0" => break,
            _ => {
                eprintln!("无效的输入");
                continue;
            }
        }
    }
}
