use chrono::NaiveDate;
use rib_backend::{
    state::{Role, State},
    user::{self, User, UserBuilder, UserIdentifier},
};

pub async fn menu(state: &State) {
    match state.role().unwrap() {
        Role::Administrator => (),
        _ => {
            eprintln!("权限不足");
            return;
        }
    }

    loop {
        println!("用户管理：");
        println!("1. 添加用户");
        println!("2. 删除用户");
        println!("3. 修改用户");
        println!("4. 查询用户");
        println!("0. 返回上一级");
        println!("请选择：");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => add_user(state).await,
            "2" => delete_user(state).await,
            "3" => update_user(state).await,
            "4" => query_user(state).await,
            "0" => {
                break;
            }
            _ => {
                eprintln!("无效的输入");
                continue;
            }
        }
    }
}

async fn add_user(state: &State) {
    println!("添加用户（非必填可以留空）：");

    println!("请输入用户类型（1. 管理员 2. 图书管理员 3. 读者）[必填]：");
    let mut role_id = String::new();
    std::io::stdin().read_line(&mut role_id).unwrap();
    let role_id = match role_id.trim() {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        _ => {
            eprintln!("无效的输入");
            return;
        }
    };

    println!("请输入用户手机号或邮箱[必填]：");
    let mut user_identifier = String::new();
    std::io::stdin().read_line(&mut user_identifier).unwrap();
    let user_identifier = match covert_to_identifier(user_identifier.trim().to_string()) {
        Some(identifier) => identifier,
        None => {
            eprintln!("输入的手机号或邮箱格式不正确。");
            return;
        }
    };

    let is_phone_number = match user_identifier {
        UserIdentifier::PhoneNumber(_) => true,
        UserIdentifier::EmailAddress(_) => false,
    };

    println!("请输入用户密码[必填]：");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    let password = match password.trim() {
        "" => {
            eprintln!("密码不能为空");
            return;
        }
        password => password.to_string(),
    };

    let mut user_builder = UserBuilder::new(user_identifier, password, role_id);

    println!("请输入用户姓名：");
    println!("姓：");
    let mut first_name = String::new();
    std::io::stdin().read_line(&mut first_name).unwrap();
    let first_name = first_name.trim();
    println!("名：");
    let mut last_name = String::new();
    std::io::stdin().read_line(&mut last_name).unwrap();
    let last_name = last_name.trim();
    match (first_name, last_name) {
        ("", "") | ("", _) | (_, "") => (),
        (_, _) => user_builder.set_name(first_name.to_string(), last_name.to_string()),
    };

    println!("请输入生日（格式：YYYY-MM-DD）：");
    let mut birthday = String::new();
    std::io::stdin().read_line(&mut birthday).unwrap();
    match birthday.trim() {
        "" => (),
        birthday => {
            let birthday = match birthday.parse::<NaiveDate>() {
                Ok(birthday) => birthday,
                Err(e) => {
                    eprintln!("生日格式不正确：{:#?}", e);
                    return;
                }
            };

            user_builder.set_birthday(birthday);
        }
    };

    if is_phone_number {
        println!("请输入电子邮箱地址：");
        let mut email_address = String::new();
        std::io::stdin().read_line(&mut email_address).unwrap();
        match email_address.trim() {
            "" => (),
            email => user_builder.set_email_address(email.to_string()),
        };
    } else {
        println!("请输入手机号码：");
        let mut phone_number = String::new();
        std::io::stdin().read_line(&mut phone_number).unwrap();
        match phone_number.trim() {
            "" => (),
            phone => user_builder.set_phone_number(phone.to_string()),
        };
    }

    println!("请输入地址：");
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();
    match address.trim() {
        "" => (),
        addr => user_builder.set_address(addr.to_string()),
    };

    match user::create_user(state.db_conn(), user_builder).await {
        Ok(_) => println!("添加成功"),
        Err(e) => eprintln!("添加失败：{:#?}", e),
    };
}

async fn delete_user(state: &State) {
    println!("请输入要删除的用户手机号或邮箱：");
    let mut user_identifier = String::new();
    std::io::stdin().read_line(&mut user_identifier).unwrap();
    let user_identifier = match covert_to_identifier(user_identifier.trim().to_string()) {
        Some(identifier) => identifier,
        None => {
            eprintln!("输入的手机号或邮箱格式不正确。");
            return;
        }
    };

    let user_id = match user::read_user_by_identifier(state.db_conn(), &user_identifier).await {
        Ok(user) => user.id,
        Err(e) => {
            eprintln!("查找失败：{:#?}", e);
            return;
        }
    };

    match user::delete_user(state.db_conn(), user_id).await {
        Ok(_) => println!("删除成功"),
        Err(e) => eprintln!("删除失败：{:#?}", e),
    };
}

async fn update_user(state: &State) {
    println!("请输入要修改的用户手机号或邮箱：");
    let mut user_identifier = String::new();
    std::io::stdin().read_line(&mut user_identifier).unwrap();
    let user_identifier = match covert_to_identifier(user_identifier.trim().to_string()) {
        Some(identifier) => identifier,
        None => {
            eprintln!("输入的手机号或邮箱格式不正确。");
            return;
        }
    };

    let user = match user::read_user_by_identifier(state.db_conn(), &user_identifier).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("查找失败：{:#?}", e);
            return;
        }
    };

    update_basic_info(state, &user).await;
}

async fn query_user(state: &State) {
    println!("请输入要查询的用户手机号或邮箱：");
    let mut user_identifier = String::new();
    std::io::stdin().read_line(&mut user_identifier).unwrap();
    let user_identifier = match covert_to_identifier(user_identifier.trim().to_string()) {
        Some(identifier) => identifier,
        None => {
            eprintln!("输入的手机号或邮箱格式不正确。");
            return;
        }
    };

    match user::read_user_by_identifier(state.db_conn(), &user_identifier).await {
        Ok(user) => {
            println!("用户信息：");
            print_user(&user);
        }
        Err(e) => {
            eprintln!("查找失败：{:#?}", e);
        }
    }
}

pub async fn update_basic_info(state: &State, user: &User) {
    println!("修改用户（留空为保持原值）:");

    let mut user_builder = UserBuilder::from_model(user.clone());

    println!("请输入新的密码：");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    match password.trim() {
        "" => (),
        password => user_builder.set_password(password.to_string()),
    };

    println!("请输入新的姓名：");
    println!("姓：");
    let mut first_name = String::new();
    std::io::stdin().read_line(&mut first_name).unwrap();
    let first_name = first_name.trim();
    println!("名：");
    let mut last_name = String::new();
    std::io::stdin().read_line(&mut last_name).unwrap();
    let last_name = last_name.trim();
    match (first_name, last_name) {
        ("", "") | ("", _) | (_, "") => (),
        (_, _) => user_builder.set_name(first_name.to_string(), last_name.to_string()),
    };

    println!("请输入新的生日（格式：YYYY-MM-DD）：");
    let mut birthday = String::new();
    std::io::stdin().read_line(&mut birthday).unwrap();
    match birthday.trim() {
        "" => (),
        birthday => {
            let birthday = match birthday.parse::<NaiveDate>() {
                Ok(birthday) => birthday,
                Err(e) => {
                    eprintln!("生日格式不正确：{:#?}", e);
                    return;
                }
            };

            user_builder.set_birthday(birthday);
        }
    };

    println!("请输入新的手机号码：");
    let mut phone_number = String::new();
    std::io::stdin().read_line(&mut phone_number).unwrap();
    match phone_number.trim() {
        "" => (),
        phone => user_builder.set_phone_number(phone.to_string()),
    };

    println!("请输入新的电子邮箱地址：");
    let mut email_address = String::new();
    std::io::stdin().read_line(&mut email_address).unwrap();
    match email_address.trim() {
        "" => (),
        email => user_builder.set_email_address(email.to_string()),
    };

    println!("请输入新的地址：");
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();
    match address.trim() {
        "" => (),
        addr => user_builder.set_address(addr.to_string()),
    };

    match user::update_user(state.db_conn(), user.id, user_builder).await {
        Ok(_) => println!("修改成功"),
        Err(e) => eprintln!("修改失败：{:#?}", e),
    };
}

pub fn covert_to_identifier(user_identifier: String) -> Option<UserIdentifier> {
    let phone_number_regex =
        regex::Regex::new(r"^(13[0-9]|14[5|7]|15[0|1|2|3|5|6|7|8|9]|18[0|1|2|3|5|6|7|8|9])\d{8}$")
            .unwrap();

    let email_regex = regex::Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();

    if phone_number_regex.is_match(&user_identifier) {
        Some(UserIdentifier::PhoneNumber(user_identifier))
    } else if email_regex.is_match(&user_identifier) {
        Some(UserIdentifier::EmailAddress(user_identifier))
    } else {
        None
    }
}

pub fn print_user(user: &User) {
    println!("ID: {}", user.id);

    println!(
        "用户类型：{}",
        match user.role_id {
            1 => "管理员",
            2 => "图书管理员",
            3 => "读者",
            _ => unreachable!("无效的用户类型"),
        }
    );

    println!("当前密码：{}", user.password);

    println!(
        "姓名：{}",
        match (&user.first_name, &user.last_name) {
            (Some(first_name), Some(last_name)) => format!("{} {}", first_name, last_name),
            (Some(first_name), None) => first_name.to_string(),
            (None, Some(last_name)) => last_name.to_string(),
            (None, None) => "".to_string(),
        }
    );

    println!(
        "生日：{}",
        match &user.birthday {
            Some(birthday) => birthday,
            None => "",
        }
    );

    println!(
        "手机号码：{}",
        match &user.phone_number {
            Some(phone_number) => phone_number,
            None => "",
        }
    );

    println!(
        "电子邮箱地址：{}",
        match &user.email_address {
            Some(email_address) => email_address,
            None => "",
        }
    );

    println!(
        "地址：{}",
        match &user.address {
            Some(address) => address,
            None => "",
        }
    );

    println!("注册时间：{}", user.registration_time);
}
