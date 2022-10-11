use std::{collections::HashMap, error, sync::Arc};

use anyhow::{anyhow, Result};
use bytes::Bytes;
use futures::StreamExt;
use log::{error, info};
use num_enum::TryFromPrimitive;
use quinn::{Connection, Datagrams};
use tokio::sync::{mpsc, Mutex};
use tuple_conv::RepeatedTuple;
use uuid::Uuid;

pub(super) struct Manager {
    conn: Connection,
    input: Datagrams,
    rx: mpsc::Receiver<Request>,
    mp: Arc<Mutex<HashMap<Uuid, mpsc::Sender<SessionPacket>>>>,
}

#[derive(Debug)]
pub struct SessionPacket {
    id: Uuid,
    payload: Bytes,
}

#[derive(Debug)]
pub struct RegistrationRequest {
    id: Uuid,
    tx: mpsc::Sender<SessionPacket>,
}

#[derive(Debug)]
pub struct UnregistrationRequest {
    id: Uuid,
}

#[derive(Debug)]
pub enum Request {
    Packet(SessionPacket),
    Registration(RegistrationRequest),
    Unregistration(UnregistrationRequest),
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum DatagramType {
    UDP = 0,
    IP = 1,
}

impl Manager {
    pub fn new(conn: Connection, input: Datagrams) -> (Self, mpsc::Sender<Request>) {
        let (tx, rx) = mpsc::channel(32);

        (
            Self {
                conn,
                input,
                rx,
                mp: Arc::new(Mutex::new(HashMap::new())),
            },
            tx,
        )
    }

    pub async fn serve(self) -> Result<()> {
        let Self {
            conn,
            input,
            mut rx,
            mp,
        } = self;

        let read_datagrams = {
            let mp = mp.clone();
            tokio::spawn(async move {
                let mut iter = input.enumerate();

                while let Some((size, datagram)) = iter.next().await {
                    info!("Got datagram: {:?} {:?}", size, datagram);
                    // we need to demux this packet.
                    let pkt = Self::demux(datagram?)?;
                    let mut mp = mp.lock().await;
                    if let Some(tx) = mp.get_mut(&pkt.id) {
                        tx.send(pkt).await?;
                    } else {
                        error!("no session for id: {}", pkt.id);
                    }
                }

                Ok::<(), anyhow::Error>(())
            })
        };

        let handle_requests = {
            let mp = mp.clone();
            tokio::spawn(async move {
                while let Some(reg) = rx.recv().await {
                    match reg {
                        Request::Packet(pkt) => {
                            let pkt = Self::mux(pkt)?;
                            conn.send_datagram(pkt)?;
                        }
                        Request::Registration(reg) => {
                            let mut mp = mp.lock().await;
                            info!("Registering session: {:?}", reg);
                            mp.insert(reg.id, reg.tx);
                        }
                        Request::Unregistration(reg) => {
                            let mut mp = mp.lock().await;
                            info!("Unregisering session: {:?}", reg);
                            mp.remove(&reg.id);
                        }
                    }
                }

                Ok::<(), anyhow::Error>(())
            })
        };

        let handles = tokio::try_join!(read_datagrams, handle_requests)?.to_vec();

        let errors = handles
            .iter()
            .filter(|h| h.is_err())
            .map(|h| h.as_ref().unwrap_err())
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            Err(anyhow!("Errors: {:?}", errors))
        } else {
            Ok(())
        }
    }

    fn mux(pkt: SessionPacket) -> Result<Bytes> {
        let mut buf = Vec::new();

        buf.extend(pkt.payload);
        buf.extend(pkt.id.as_bytes());
        buf.push(DatagramType::UDP as u8);

        Ok(Bytes::from(buf))
    }

    fn demux(data: Bytes) -> Result<SessionPacket> {
        let msg_type = data.split_last();
        if msg_type.is_none() {
            return Err(anyhow!("Invalid packet"));
        }

        let (msg_type, msg) = msg_type.unwrap();

        let msg_type: DatagramType = msg_type
            .to_owned()
            .try_into()
            .or_else(|_| Err(anyhow!("Invalid packet unknown datatype {}", msg_type)))?;

        match msg_type {
            DatagramType::UDP => {
                if let Some(id) = msg.get(msg.len() - 16..msg.len()) {
                    let id = Uuid::from_slice(id)?;
                    let payload = msg.get(..msg.len() - 16).unwrap_or(&[]);
                    Ok(SessionPacket {
                        id,
                        payload: Bytes::copy_from_slice(payload),
                    })
                } else {
                    Err(anyhow!("Invalid packet"))
                }
            }
            DatagramType::IP => {
                // Well, cloudflare seems to just cycle this packet into the demuxer.
                // Which is really weird. What is the point of that, its essentially just a for loop until it hits the UDP case.
                info!("Got IP packet, rerunning demux");
                Self::demux(Bytes::copy_from_slice(msg))
            }
        }
    }
}
