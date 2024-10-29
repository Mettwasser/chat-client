use client::Result;
use tokio::{
    io::{
        copy,
        stdin,
        stdout,
        Stdin,
        Stdout,
    },
    net::{
        tcp::{
            OwnedReadHalf,
            OwnedWriteHalf,
        },
        TcpStream,
    },
};

async fn handle_stdin(mut w: OwnedWriteHalf, mut stdin: Stdin) -> Result<()> {
    copy(&mut stdin, &mut w).await?;

    Ok(())
}

async fn handle_stdout(mut r: OwnedReadHalf, mut stdout: Stdout) -> Result<()> {
    copy(&mut r, &mut stdout).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6000").await?;
    let (r, w) = stream.into_split();

    println!("Connected to the server.");
    println!("Type your message, then press enter.");

    let stdin = stdin();
    let stdout = stdout();

    tokio::select! {
        res = handle_stdin(w, stdin) => res,
        res = handle_stdout(r, stdout) => res,
    }?;

    Ok(())
}
