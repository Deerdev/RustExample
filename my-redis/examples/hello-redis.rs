use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 建立与mini-redis服务器的连接
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设置 key: "hello" 和 值: "world"
    for i in 1..100 {
        println!("{}", i);
        let r = client.set("hello", format!("word {}", i).into()).await;
        match r {
            Ok(_) => println!("ok"),
            Err(_) => println!("err"),
        };
    }

    // 获取"key=hello"的值
    let result = client.get("hello").await?;
    println!("get `hello` result: {:?}", result);
    Ok(())
}

// cargo install mini-redis
// 启动 server: mini-redis-server
