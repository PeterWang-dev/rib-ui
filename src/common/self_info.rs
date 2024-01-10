use crate::common::user::{print_user, update_basic_info};
use rib_backend::{
    state::State,
    user,
};

pub async fn menu(state: &State) {
    loop {
        let self_info = user::read_user(state.db_conn(), state.login_user().unwrap())
        .await
        .unwrap();

        println!("个人信息：");
        print_user(&self_info);

        println!("个人信息管理：");
        println!("1. 修改信息");
        println!("0. 返回上一级");
        println!("请选择：");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => update_basic_info(state, &self_info).await,
            "0" => break,
            _ => {
                eprintln!("无效的输入");
                continue;
            }
        }
    }
}
