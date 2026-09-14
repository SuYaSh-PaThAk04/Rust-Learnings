use ::tokio::net::TcpListener;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() {
    let store: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Redis port is running on port 6379");

    loop {
        let (socket, _) = listner.accept().await.unwrap();
        let store = Arc::clone(&store);

        tokio::spawn(async move {
            handle_client(socket, store).await;
        });
    }
}
async fn handle_client(socket: tokio::net::TcpStream, store: Arc<DashMap<String, String>>) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    loop {
        line.clear();

        if reader.read_line(&mut line).await.unwrap() == 0 {
            break;
        }
        if !line.starts_with('*') {
            continue;
        }
        let count: usize = line[1..].trim().parse().unwrap_or(0);
        let mut args: Vec<String> = Vec::new();

        for _ in 0..count {
            line.clear();
            reader.read_line(&mut line).await.unwrap();
            line.clear();
            reader.read_line(&mut line).await.unwrap();

            args.push(line.trim().to_string());
        }
        if args.is_empty() {
            continue;
        }
        let response = match args[0].to_uppercase().as_str() {
            "PING" => "+PONG\r\n".to_string(),
            "GET" => {
                if args.len() < 2 {
                    "-ERR wrong number of arguments \r\n".to_string()
                } else {
                    match store.get(&args[1]) {
                        Some(v) => format!("${}\r\n{}\r\n", v.len(), v.value()),
                        None => "$-1\r\n".to_string(),
                    }
                }
            }
            "SET" => {
                if args.len() < 3 {
                    "-ERR wrong number of arguments!!\r\n".to_string()
                } else {
                    store.insert(args[1].clone(), args[2].clone());
                    "+OK\r\n".to_string()
                }
            }
            _ => "-WRR unknown command \r\n".to_string(),
        };
        writer.write_all(response.as_bytes()).await.unwrap();
    }
}
