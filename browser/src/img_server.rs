// src/img_server.rs
use bevy::{
    prelude::*,
    tasks::IoTaskPool,
};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use crate::{UdpSender, UdpReceiver};

const UDP_PORT: &str = "0.0.0.0:12345"; // 例のポート

// UDP受信を開始するシステム
pub fn start_udp_receiver(
    udp_sender: Res<UdpSender>,
    runtime_handle: Res<TokioRuntimeHandle>, // main.rsからTokioRuntimeHandleをインポート
) {
    let sender_clone = udp_sender.sender.clone();
    let runtime_handle_clone = runtime_handle.0.clone();

    runtime_handle_clone.spawn(async move {
        let socket = UdpSocket::bind(UDP_PORT).await.expect("Failed to bind UDP socket");
        println!("UDP receiver started on {}", UDP_PORT);
        let mut buf = vec![0u8; 65536]; // Max UDP packet size

        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, _addr)) => {
                    if let Err(e) = sender_clone.lock().unwrap().send(buf[..len].to_vec()) {
                        eprintln!("Failed to send UDP data to Bevy channel: {}", e);
                        break; // Channel disconnected
                    }
                }
                Err(e) => {
                    eprintln!("UDP receive error: {}", e);
                }
            }
        }
    });
}

// UDPパケットをBevyのWorldで処理するシステム
pub fn process_udp_packets(
    udp_receiver: Res<UdpReceiver>,
    mut commands: Commands,
    // ここで受信した画像データを処理するロジック
    // 例: TextureHandle を更新したり、Image コンポーネントを更新したり
) {
    let receiver_mutex = udp_receiver.receiver.lock().unwrap();
    while let Ok(data) = receiver_mutex.try_recv() {
        // ここでUDPデータ（おそらく画像データ）を処理
        println!("Received UDP packet with {} bytes.", data.len());
        // TODO: 画像データをBevyのImageアセットに変換し、EguiContextsに登録する
        // 例: commands.spawn(MyImageData(data));
    }
}
