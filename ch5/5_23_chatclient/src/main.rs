use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = String::new();

    let stream = TcpStream::connect("localhost:1234").await?;
    let (reader, mut writer) = tokio::io::split(stream);
    let mut reader = BufReader::new(reader);

    print!("대화명을 입력하세요: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut username)?;
    writer.write_all(username.as_bytes()).await?;

    tokio::spawn(async move {
        loop {
            let mut message = String::new();

            match reader.read_line(&mut message).await {
                Ok(_) => {
                    print!("{}", message);
                },
                Err(_) => {
                    break;
                }
            };
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        writer.write_all(input.as_bytes()).await?;

        if input.trim() == "/exit" {
            break;
        }
    }

    Ok(())
}