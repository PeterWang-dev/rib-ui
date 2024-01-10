use crate::common::user::covert_to_identifier;
use rib_backend::{
    error::{
        Error::{FirstRun, LoginError},
        LoginError as LoginErr,
    },
    init::{initialize as backend_initialize, setup as backend_setup},
    state::State,
    user::UserIdentifier,
};
use std::io;

pub async fn initialize() -> State {
    println!("正在初始化……");

    match backend_initialize().await {
        Ok(state) => state,
        Err(FirstRun) => setup().await,
        Err(err) => panic!("初始化失败：{:#?}", err),
    }
}

pub async fn setup() -> State {
    println!("检测到首次运行，正在进行首次运行设置……");

    let admin_identifier: UserIdentifier;
    let admin_password: String;

    loop {
        println!("请输入管理员手机号或邮箱：");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        admin_identifier = match covert_to_identifier(input.trim().to_string()) {
            Some(identifier) => identifier,
            None => {
                eprint!("输入的管理员手机号或邮箱格式不正确。");
                continue;
            }
        };

        println!("请输入管理员密码：");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        admin_password = input.trim().to_string();

        break;
    }

    match backend_setup(admin_identifier.clone(), admin_password.clone()).await {
        Ok(state) => {
            println!("首次运行设置成功！");
            state
        }
        Err(err) => panic!("首次运行设置失败：{:#?}", err),
    }
}

pub async fn login(state: &mut State) {
    loop {
        println!("Ribrarian 图书馆管理系统, 请登陆...");
        println!("请输入手机号或邮箱：");

        let mut user_identifier = String::new();
        io::stdin().read_line(&mut user_identifier).unwrap();

        let user_identifier = match covert_to_identifier(user_identifier.trim().to_string()) {
            Some(identifier) => identifier,
            None => {
                eprint!("输入的手机号或邮箱格式不正确。");
                continue;
            }
        };

        println!("请输入密码：");

        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();

        let password = password.trim();

        match state.set_login(&user_identifier, password).await {
            Ok(_) => {
                println!("登录成功！");
                break;
            }
            Err(LoginError(err)) => match err {
                LoginErr::InvalidUsername => {
                    eprint!("输入的手机号或邮箱不存在。");
                    continue;
                }
                LoginErr::InvalidPassword => {
                    eprint!("输入的密码不正确。");
                    continue;
                }
            },
            Err(err) => panic!("登录失败：{:#?}", err),
        }
    }
}
