use rib_backend::state::State;
use crate::common::{book, user, self_info};

pub async fn menu(state: &State) {
    loop {
        println!("超级管理员菜单：");
        println!("1. 用户管理");
        println!("2. 图书管理");
        println!("3. 个人信息");
        println!("0. 退出");
        println!("请选择：");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => user::menu(state).await,
            "2" => book::menu(state).await,
            "3" => self_info::menu(state).await,
            "0" => break,
            _ => {
                eprintln!("无效的输入");
                continue;
            }
        }
    }
}
