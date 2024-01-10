mod menu;
mod init;

use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    println!("欢迎使用 Ribrarian 图书馆管理系统！");
    println!("程序正在启动……");

    let mut state = init::initialize().await;
    println!("程序启动成功！");

    init::login(&mut state).await;

    menu::main_menu(&state).await;

    Ok(())
}

