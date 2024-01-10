use crate::common::user::covert_to_identifier;
use rib_backend::{
    borrow::BorrowBuilder,
    state::{Role, State},
    user,
};
use uuid::Uuid;

pub async fn management_menu(state: &State) {
    match state.role().unwrap() {
        Role::Librarian | Role::Reader => (),
        _ => {
            eprintln!("权限不足");
            return;
        }
    }

    loop {
        println!("借阅管理：");
        if state.role().unwrap() == Role::Reader {
            println!("1. 查询个人借阅信息");
            println!("2. 续借图书");
            println!("0. 返回上一级");
            println!("请选择：");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "1" => query_self_borrow(state).await,
                "2" => renew_book(state).await,
                "0" => break,
                _ => {
                    eprintln!("无效的输入");
                    continue;
                }
            }
        } else {
            println!("1. 查询用户借阅信息");
            println!("2. 借阅图书");
            println!("3. 归还图书");
            println!("0. 返回上一级");
            println!("请选择：");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "1" => query_user_borrow(state).await,
                "2" => borrow_book(state).await,
                "3" => return_book(state).await,
                "0" => break,
                _ => {
                    eprintln!("无效的输入");
                    continue;
                }
            }
        }
    }
}

async fn query_user_borrow(state: &State) {
    println!("请输入用户手机号或者邮箱：");
    let mut identifier = String::new();
    std::io::stdin().read_line(&mut identifier).unwrap();
    let identifier = covert_to_identifier(identifier.trim().to_string()).unwrap();

    let user_id = match user::read_user_by_identifier(state.db_conn(), &identifier).await {
        Ok(user) => user.id,
        Err(e) => {
            eprintln!("查询失败：{:#?}", e);
            return;
        }
    };

    query_borrow(state, user_id).await;
}

async fn borrow_book(state: &State) {
    println!("请输入用户手机号或者邮箱：");
    let mut identifier = String::new();
    std::io::stdin().read_line(&mut identifier).unwrap();
    let identifier = match covert_to_identifier(identifier.trim().to_string()) {
        Some(identifier) => identifier,
        None => {
            eprintln!("无效的输入");
            return;
        }
    };

    let user_id = match user::read_user_by_identifier(state.db_conn(), &identifier).await {
        Ok(user) => user.id,
        Err(e) => {
            eprintln!("查询失败：{:#?}", e);
            return;
        }
    };

    println!("请输入图书编号：");
    let mut uuid = String::new();
    std::io::stdin().read_line(&mut uuid).unwrap();
    let uuid = Uuid::parse_str(uuid.trim()).unwrap();

    let borrow_builder = BorrowBuilder::new(user_id, uuid);

    match rib_backend::borrow::borrow_book(state.db_conn(), borrow_builder).await {
        Ok(_) => println!("借阅成功"),
        Err(e) => eprintln!("借阅失败：{:#?}", e),
    }
}

async fn return_book(state: &State) {
    println!("请输入图书编号：");
    let mut uuid = String::new();
    std::io::stdin().read_line(&mut uuid).unwrap();
    let uuid = Uuid::parse_str(uuid.trim()).unwrap();

    match rib_backend::borrow::return_book(state.db_conn(), &uuid).await {
        Ok(_) => println!("归还成功"),
        Err(e) => eprintln!("归还失败：{:#?}", e),
    }
}

async fn query_borrow(state: &State, user_id: i32) {
    let borrows = rib_backend::borrow::user_borrows(state.db_conn(), user_id)
        .await
        .unwrap();

    for borrow in borrows {
        println!("书籍编号：{}", borrow.book_uuid);
        println!("书籍标题：{}", borrow.book_title);
        println!("借阅时间：{}", borrow.borrow_date);
        println!("归还时间：{}", borrow.return_date);
        println!();
    }
}
async fn query_self_borrow(state: &State) {
    query_borrow(state, state.login_user().unwrap()).await;
}

async fn renew_book(state: &State) {
    println!("请输入图书编号：");
    let mut uuid = String::new();
    std::io::stdin().read_line(&mut uuid).unwrap();
    let uuid = Uuid::parse_str(uuid.trim()).unwrap();

    match rib_backend::borrow::renew_book(state.db_conn(), &uuid).await {
        Ok(_) => println!("续借成功"),
        Err(e) => eprintln!("续借失败：{:#?}", e),
    }
}
