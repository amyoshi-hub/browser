use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::net::SocketAddr;
use tracing::{info, error};
use crate::AsyncComputeTaskPool;

#[derive(Event)]
pub struct P2pUdpPacketReceived {
    pub data: Vec<u8>,
    pub peer_addr: SocketAddr,
}

pub fn setup_p2p_udp_listener(
    mut commands: Commands,
    runtime: Res<TokioTasksRuntime>,
) {
    let bind_address = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    println!("P2P UDPリスナーを {} で開始します。", bind_address);

    let (tx, rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(100);

    let task_pool = AsyncComputeTaskPool::get();
    task_pool.spawn(async move {
        let socket = match UdpSocket::bind(bind_address).await {
            Ok(s) => {
                info!("UDPソケットがバインドされました: {}", bind_address);
                s
            },
            Err(e) => {
                error!("UDPソケットのバインドに失敗しました: {}", e);
                return;
            }
        };

        let socket_arc = Arc::new(socket);
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let mut buf = vec![0u8; 65507];
            loop {
                match socket_arc.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        let received_data = buf[..len].to_vec();
                        info!("UDPパケットを受信: {} バイト from {}", len, addr);
                        if tx_clone.send((received_data, addr)).await.is_err() {
                            error!("Bevyシステムへのデータ送信に失敗しました。");
                            break;
                        }
                    },
                    Err(e) => {
                        error!("P2P UDP受信エラー: {}", e);
                        break;
                    }
                }
            }
        });
    });

    commands.insert_resource(P2pUdpReceiver(rx));
}

#[derive(Resource)]
pub struct P2pUdpReceiver(mpsc::Receiver<(Vec<u8>, SocketAddr)>);

pub fn poll_p2p_udp_packets(
    mut receiver: ResMut<P2pUdpReceiver>,
    mut event_writer: EventWriter<P2pUdpPacketReceived>,
) {
    while let Ok((data, addr)) = receiver.0.try_recv() {
        info!("BevyシステムでUDPパケットを処理: {} バイト from {}", data.len(), addr);
        // `EventWriter::send`は非推奨のため、`write`を使用します。
        event_writer.write(P2pUdpPacketReceived { data, peer_addr: addr });
    }
}
