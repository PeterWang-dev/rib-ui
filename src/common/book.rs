use chrono::NaiveDate;
use rib_backend::{
    book::{self, BookBuilder},
    state::{Role, State},
};
use uuid::Uuid;

pub async fn menu(state: &State) {
    match state.role().unwrap() {
        Role::Administrator | Role::Librarian => (),
        _ => {
            eprintln!("权限不足");
            return;
        }
    }

    loop {
        println!("图书管理：");
        println!("1. 添加图书");
        println!("2. 删除图书");
        println!("3. 修改图书信息");
        println!("4. 查询图书");
        println!("0. 返回上一级");

        println!("请选择：");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => add_book(state).await,
            "2" => delete_book(state).await,
            "3" => update_book(state).await,
            "4" => query_book(state).await,
            "0" => break,
            _ => {
                eprintln!("无效的输入");
                continue;
            }
        }
    }
}

async fn add_book(state: &State) {
    println!("添加图书：");

    println!("请输入图书标题：");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();
    let title = match title.trim() {
        "" => {
            eprintln!("图书标题不能为空");
            return;
        }
        title => title.to_string(),
    };

    println!("请输入图书作者：");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();
    let author = match author.trim() {
        "" => {
            eprintln!("图书作者不能为空");
            return;
        }
        author => author.to_string(),
    };

    println!("请输入图书出版社：");
    let mut publisher = String::new();
    std::io::stdin().read_line(&mut publisher).unwrap();
    let publisher = match publisher.trim() {
        "" => {
            eprintln!("图书出版社不能为空");
            return;
        }
        publisher => publisher.to_string(),
    };

    println!("请输入图书出版日期：(格式：YYYY-MM-DD)");
    let mut published_time = String::new();
    std::io::stdin().read_line(&mut published_time).unwrap();
    let published_time = match published_time.trim() {
        "" => {
            eprintln!("图书出版日期不能为空");
            return;
        }
        published_time => match published_time.parse::<NaiveDate>() {
            Ok(published_time) => published_time,
            Err(_) => {
                eprintln!("图书出版日期格式错误");
                return;
            }
        },
    };

    println!("请输入图书分类：");
    let mut category = String::new();
    std::io::stdin().read_line(&mut category).unwrap();
    let category = match category.trim() {
        "" => {
            eprintln!("图书分类不能为空");
            return;
        }
        category => category.to_string(),
    };

    println!("请输入图书 ISBN：");
    let mut isbn = String::new();
    std::io::stdin().read_line(&mut isbn).unwrap();
    let isbn = match isbn.trim() {
        "" => {
            eprintln!("图书 ISBN 不能为空");
            return;
        }
        isbn => isbn.to_string(),
    };

    let book_builder = BookBuilder::new(title, author, publisher, published_time, category, isbn);

    match book::create_book(state.db_conn(), book_builder).await {
        Ok(res) => println!("添加成功：{}", res),
        Err(e) => eprintln!("添加失败：{:#?}", e),
    }
}

async fn delete_book(state: &State) {
    println!("删除图书：");

    println!("请输入图书编号：");
    let mut uuid = String::new();
    std::io::stdin().read_line(&mut uuid).unwrap();
    let uuid = Uuid::parse_str(uuid.trim()).unwrap();

    match book::delete_book(state.db_conn(), &uuid).await {
        Ok(_) => println!("删除成功"),
        Err(e) => eprintln!("删除失败：{:#?}", e),
    }
}

async fn update_book(state: &State) {
    println!("请输入图书编号：");
    let mut uuid = String::new();
    std::io::stdin().read_line(&mut uuid).unwrap();
    let uuid = Uuid::parse_str(uuid.trim()).unwrap();

    let book = match book::read_book(state.db_conn(), &uuid).await {
        Ok(book) => book,
        Err(e) => {
            eprintln!("查询失败：{:#?}", e);
            return;
        }
    };

    println!("修改图书（非必填可以留空）：");

    let mut book_builder = BookBuilder::from_model(book);

    println!("请输入图书标题：");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();
    let title = title.trim().to_string();
    if !title.is_empty() {
        book_builder.set_title(title);
    }

    println!("请输入图书作者：");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();
    let author = author.trim().to_string();
    if !author.is_empty() {
        book_builder.set_author(author);
    }

    println!("请输入图书出版社：");
    let mut publisher = String::new();
    std::io::stdin().read_line(&mut publisher).unwrap();
    let publisher = publisher.trim().to_string();
    if !publisher.is_empty() {
        book_builder.set_publisher(publisher);
    }

    println!("请输入图书出版日期：(格式：YYYY-MM-DD)");
    let mut published_time = String::new();
    std::io::stdin().read_line(&mut published_time).unwrap();
    let published_time = published_time.trim().to_string();
    if !published_time.is_empty() {
        let published_time = published_time.parse::<NaiveDate>().unwrap();
        book_builder.set_published_time(published_time);
    }

    println!("请输入图书分类：");
    let mut category = String::new();
    std::io::stdin().read_line(&mut category).unwrap();
    let category = category.trim().to_string();
    if !category.is_empty() {
        book_builder.set_category(category);
    }

    println!("请输入图书 ISBN：");
    let mut isbn = String::new();
    std::io::stdin().read_line(&mut isbn).unwrap();
    let isbn = isbn.trim().to_string();
    if !isbn.is_empty() {
        book_builder.set_isbn(isbn);
    }

    match book::update_book(state.db_conn(), &uuid, book_builder).await {
        Ok(_) => println!("修改成功"),
        Err(e) => eprintln!("修改失败：{:#?}", e),
    }
}

async fn query_book(state: &State) {
    println!("查询图书：");

    println!("请输入图书编号：");
    let mut uuid = String::new();
    std::io::stdin().read_line(&mut uuid).unwrap();
    let uuid = Uuid::parse_str(uuid.trim()).unwrap();

    match book::read_book(state.db_conn(), &uuid).await {
        Ok(book) => {
            println!("图书信息：");
            print_book(&book);
        }
        Err(e) => eprintln!("查询失败：{:#?}", e),
    }
}

pub fn print_book(book: &book::Book) {
    println!("图书编号：{}", book.uuid);
    println!("图书标题：{}", book.title);
    println!("图书作者：{}", book.author);
    println!("图书出版社：{}", book.publisher);
    println!("图书出版日期：{}", book.published_time);
    println!("图书分类：{}", book.category);
    println!("图书 ISBN：{}", book.isbn);
    println!();
}
