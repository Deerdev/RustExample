use bytes::Bytes;
use mini_redis::client;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

// client 并没有实现 Copy 特征，无法再多个任务中使用
// 改为消息传递，一个 C 初始化 connect，其他任务都和 C 通信；而且这种方式还有利于实现连接池，例如不止一个 P 和 C 时，多个 P 可以往消息通道中发送消息，同时多个 C，其中每个 C 都维护一条连接，并从消息通道获取消息。

// 一个任务可以通过此通道将命令发送给管理 redis 连接的任务，同时由于通道支持多个生产者，因此多个任务可以同时发送命令。创建该通道会返回一个发送和接收句柄，这两个句柄可以分别被使用，例如它们可以被移动到不同的任务中。
// 你可以使用 clone 方法克隆多个发送者，但是接收者无法被克隆，因为我们的通道是 mpsc 类型。
// 当所有的发送者都被 Drop 掉后(超出作用域或被 drop(...) 函数主动释放)，就不再会有任何消息发送给该通道，此时 recv 方法将返回 None，也意味着该通道已经被关闭。
// 在我们的例子中，接收者是在管理 redis 连接的任务中，当该任务发现所有发送者都关闭时，它知道它的使命可以完成了，因此它会关闭 redis 连接。

// mpsc: 发送消息
// onseshot: 返回结果
use tokio::sync::{mpsc, oneshot};
/// 管理任务可以使用该发送端将命令执行的结果传回给发出命令的任务
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    // 将消息通道接收者 rx 的所有权转移到管理任务中
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        // 建立到 redis 服务器的连接
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收消息
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // 忽略错误
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // 忽略错误
                    let _ = resp.send(res);
                }
            }
        }
    });

    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };

        // 发送 GET 请求
        tx.send(cmd).await.unwrap();

        // 等待回复
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // 发送 SET 请求
        tx2.send(cmd).await.unwrap();

        // 等待回复
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
