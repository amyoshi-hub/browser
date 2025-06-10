use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::sync::Arc;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy_tokio_tasks::TokioTasksRuntime; // Explicitly import TokioTasksRuntime
use tracing::{info, error};

// Bevyリソースとして受信チャネルを保持する構造体
#[derive(Resource)] // Resource traitを導出
pub struct P2pUdpReceiver(pub mpsc::Receiver<(Vec<u8>, SocketAddr)>);

// P2P UDPパケット受信イベント
// イベントはBevyシステム間でデータを受け渡すためのものです。
#[derive(Event)] // Make sure bevy::prelude::Event is imported
pub struct P2pUdpPacketReceived {
    pub data: Vec<u8>,
    pub sender: SocketAddr,
}


pub fn setup_p2p_udp_listener(
    mut commands: Commands,
    runtime: NonSend<TokioTasksRuntime>, // Correct: Use NonSend<...>
) {
    // IPv6リスナーの開始メッセージのみ表示
    println!("P2P UDPリスナーを IPv6 ([::]:8080) で開始します。");

    let (tx, rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(100); // メッセージ送信用チャネル
    // P2pUdpReceiverリソースはここでコマンドによって挿入されます。
    // main.rsではこのリソースをAppにinit_resourceしたりinsert_resourceしたりする必要はありません。
    commands.insert_resource(P2pUdpReceiver(rx));

    let task_pool = AsyncComputeTaskPool::get();
    let tx_clone_ipv6 = tx.clone(); // 受信タスク用の送信クローン

    // IPv6ソケットのバインドとリスニングタスクの起動
    let ipv6_bind_address: SocketAddr = "[::]:8080".parse().unwrap();
    // FIX: Get the Tokio runtime handle and clone it to move into the async block
    let runtime_handle_clone = runtime.runtime().handle().clone();

    task_pool.spawn(async move {
        let socket: Arc<UdpSocket> = match runtime_handle_clone.spawn(UdpSocket::bind(ipv6_bind_address)).await {
            Ok(Ok(s)) => { // 二重のResultを処理
                info!("IPv6 UDPソケットがバインドされました: {}", ipv6_bind_address);
                Arc::new(s)
            },
            Ok(Err(e)) => {
                error!("IPv6 UDPソケットのバインドに失敗しました ({}): {}", ipv6_bind_address, e);
                return;
            },
            Err(e) => { // spawnタスク自体のエラー
                error!("IPv6 UDPソケットのバインドタスク起動に失敗しました: {}", e);
                return;
            }
        };

        let mut buf = vec![0u8; 65507]; // UDPパケットの最大サイズ
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    let received_data = buf[..len].to_vec();
                    info!("IPv6 UDPパケットを受信: {} バイト from {}", len, addr);
                    if tx_clone_ipv6.send((received_data, addr)).await.is_err() {
                        error!("Bevyシステムへのデータ送信に失敗しました (IPv6)。");
                        break;
                    }
                },
                Err(e) => {
                    error!("IPv6 UDP受信エラー: {}", e);
                    break;
                }
            }
        }
    });
}

// UDPチャネルからパケットをポーリングし、Bevyイベントとして発行するシステム
pub fn poll_p2p_udp_packets(
    mut receiver: NonSendMut<P2pUdpReceiver>, // NonSendMutでチャネルの受信側を取得
    mut packet_events: EventWriter<P2pUdpPacketReceived>, // イベントライター
) {
    // チャネルから利用可能なすべてのパケットを受信する
    while let Ok((data, sender)) = receiver.0.try_recv() {
        info!("BevyシステムでUDPパケットを受信しました (長さ: {}, 送信元: {})", data.len(), sender);
        // 受信したデータをイベントとして発行
        packet_events.write(P2pUdpPacketReceived { data, sender }); // FIX: Changed .send() to .write()
    }
}

