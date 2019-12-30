use hyper::{
    service::{make_service_fn, service_fn},
    Body, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use structopt::StructOpt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "snake_case")]
enum Cmd {
    Read { addr: SocketAddr },
    Write { addr: SocketAddr },
    Http { addr: SocketAddr },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cmd::from_args();

    match args {
        Cmd::Read { addr } => read(addr).await?,
        Cmd::Write { addr } => write(addr).await?,
        Cmd::Http { addr } => http(addr).await?,
    };

    Ok(())
}

async fn read(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind(addr).await?;

    loop {
        let (mut stream, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            loop {
                let len = stream.read(&mut buf[..]).await.unwrap();

                if len == 0 {
                    break;
                }
            }
        });
    }
}

async fn write(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = TcpStream::connect(addr).await?;
    let mut interval = tokio::time::interval(Duration::from_millis(100));

    loop {
        interval.tick().await;

        let s = random_string(100);

        conn.write_all(s.as_bytes()).await.unwrap();
    }
}

async fn http(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    Server::bind(&addr)
        .serve(make_service_fn(|_conn| {
            async {
                Ok::<_, Infallible>(service_fn(|_| {
                    async { Ok::<_, Infallible>(Response::new(Body::empty())) }
                }))
            }
        }))
        .await?;

    Ok(())
}

fn random_string(len: usize) -> String {
    use rand::Rng;

    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .collect::<String>()
}
