use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:1234").await?;
    let (tx, _) = broadcast::channel(10);

    let shared_tx = Arc::new(tx);

    loop {
        let (stream, _) = listener.accept().await?;
        let shared_tx = shared_tx.clone();
        let mut rx = shared_tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = tokio::io::split(stream);

            tokio::spawn(async move {
                loop {
                    let data: String = match rx.recv().await {
                        Ok(data) => { data },
                        Err(_) => {
                            return; // 수신 실패시 종료
                        }
                    };
    
                    if data == "/exit" {
                        break;
                    }
    
                    print!("{}", data);
                    match writer.write_all(data.as_bytes()).await {
                        Ok(_) => {},
                        Err(err) => {
                            println!("네트웍으로 데이터 전송중 오류: {:?}", err);
                            return;
                        }
                    };
                }
            });

            let mut buf_reader = BufReader::new(reader);
            let mut username = String::new();

            buf_reader.read_line(&mut username).await;
            let username = username.trim();

            match shared_tx.send(format!("{} 님이 입장하셨습니다.\n", username)) {
                Ok(_) => {},
                Err(_) => {
                    return;  // broadcast 실패시 종료
                }
            }

            loop {
                let mut message = String::new();
                buf_reader.read_line(&mut message).await;

                let mut message = String::from(message.trim());
                if message != "/exit" {
                    message = format!("{}: {}\n", username, message);
                }

                match shared_tx.send(message) {
                    Ok(_) => {},
                    Err(_) => {
                        break; // broadcast 실패시 종료
                    }
                };
            }

            match shared_tx.send(format!("{} 님이 채팅방을 나갔습니다.\n", username)) {
                Ok(_) => {},
                Err(_) => {
                    return; // broadcast 실패시 종료
                }
            }
        });
    }
}