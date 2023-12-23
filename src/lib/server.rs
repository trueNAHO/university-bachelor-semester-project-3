use anyhow::{Error, Result};
use tap::{Conv, Pipe, Tap};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    select, spawn,
    sync::broadcast,
};
use tracing::info;

use std::ops::Add;

use crate::{action::Action, client::Client, lobby::Lobby};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = self
            .addr
            .pipe_ref(TcpListener::bind)
            .await?
            .tap(|listener| {
                info!(
                    "Server is running at: {}",
                    listener
                        .local_addr()
                        .expect("Socket address should be available")
                );
            });

        let (broadcast_sender, _) = u16::MAX.conv::<usize>().pipe(broadcast::channel);
        let lobby = Lobby::new();

        loop {
            let (mut stream, socket_addr) = listener.accept().await?;

            let (broadcast_sender, mut broadcast_receiver) =
                broadcast_sender.pipe_ref(|sender| (sender.clone(), sender.subscribe()));

            format!("New client connected at: {}", socket_addr).pipe(|msg| {
                info!("{}", msg);

                broadcast_sender
                    .send((socket_addr.to_owned(), msg.add("\n")))
                    .ok();
            });

            lobby
                .insert(socket_addr.to_string(), Client::new())
                .is_some()
                .then(|| panic!("Socket addresses should not register multiple times"));

            let lobby = lobby.clone();

            spawn(async move {
                let (mut buf_reader, mut socket_writer) = stream
                    .split()
                    .pipe(|(read_half, write_half)| (read_half.pipe(BufReader::new), write_half));

                {
                    let greeting = format!("Lobby consists of the following clients: \n{}", lobby);
                    socket_writer.write_all(greeting.as_bytes()).await?;
                }

                let mut line = String::new();

                loop {
                    select! {
                        read_line = buf_reader.read_line(&mut line) => match read_line {
                            Err(err) => {
                                err
                                    .to_string()
                                    .tap(|err| info!("{}: {}", err, socket_addr));

                                line.clear();
                            }

                            Ok(0) => {
                                format!("Client disconnected: {}", socket_addr).pipe(|msg| {
                                    info!("{}", msg);

                                    broadcast_sender
                                        .send((socket_addr.to_owned(), msg.add("\n")))
                                        .ok();
                                });

                                lobby.remove(socket_addr.to_string().as_str());

                                return Ok(())
                            }

                            _ => {
                                broadcast_sender.send((
                                    socket_addr,
                                    match line.parse::<Action>() {
                                        Ok(action) => {
                                            lobby.update(socket_addr.to_string().as_str(), action);

                                            let msg =
                                                format!("Lobby has been updated: \n{}", lobby);

                                            socket_writer.write_all(msg.as_bytes()).await?;

                                            msg
                                        }

                                        Err(_) => line.to_owned(),
                                    },
                                ))?;

                                line.clear();
                            }
                        },

                        receive = broadcast_receiver.recv() => {
                            let (transmitter_addr, msg) = receive?;

                            if transmitter_addr != socket_addr {
                                socket_writer.write_all(msg.as_bytes()).await?;
                            }
                        }

                        else => {
                            return Ok::<(), Error>(())
                        }
                    }
                }
            });
        }
    }
}
