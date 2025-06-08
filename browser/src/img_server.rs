use bevy::prelude::*;
use pnet::transport::{transport_channel, TransportChannelType::Layer4, TransportProtocol};
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tokio::runtime::Handle;

const END_SIG: u32 = 0xFFFFFFFF;

#[derive(Resource, Clone)]
pub struct TokioRuntimeHandle(pub Handle);

#[derive(Event)]
pub struct ImageChunkReceived {
    pub chunk_num: u32,
    pub data: Vec<u8>,
}

#[derive(Event)]
pub struct ImageReceptionComplete;

#[derive(Event)]
pub struct ImageReceptionError(pub String);

#[derive(Resource, Default)]
pub struct ReceivedImageData(pub Arc<Mutex<Vec<u8>>>);
#[derive(Resource)]
pub struct UdpListenPort(pub u16);



/// UDP受信チャネルを設定し、Bevyリソースとして `TransportReceiver` を登録します。
/// ここでポートを引数で受け取るように変更しました。
pub fn setup_udp_receiver(mut commands: Commands) {
    let port = 12345;
    let protocol = TransportProtocol::Ipv4(pnet::packet::ip::IpNextHeaderProtocols::Udp);
    let (mut _tx, rx) = transport_channel(4096, Layer4(protocol))
        .expect("Failed to create transport channel");

    commands.insert_resource(UdpReceiverResource(Arc::new(Mutex::new(rx))));
    commands.insert_resource(ReceivedImageData::default());
    println!("UDP receiver setup on port {}", port);

}

#[derive(Resource)]
pub struct UdpReceiverResource(pub Arc<Mutex<pnet::transport::TransportReceiver>>);

pub fn poll_udp_packets(
    udp_receiver_res: Option<Res<UdpReceiverResource>>,
    udp_listen_port_option: Option<Res<UdpListenPort>>, 
    mut image_chunk_events: EventWriter<ImageChunkReceived>,
    mut image_reception_complete_events: EventWriter<ImageReceptionComplete>,
    mut image_reception_error_events: EventWriter<ImageReceptionError>,
) {
    let Some(udp_receiver_res) = udp_receiver_res else { return; };
    let Some(udp_listen_port) = udp_listen_port_option else { return; };
    let mut rx = udp_receiver_res.0.lock().unwrap();

    let mut packet_iter = pnet::transport::udp_packet_iter(&mut *rx);

    // BevyのUpdateサイクル中に、利用可能なパケットをすべて処理
    // ノンブロッキングで、かつ目的のポートへのパケットのみを処理
    while let Ok(Some((packet, addr))) = packet_iter.next_with_timeout(std::time::Duration::from_millis(0)) {
        // パケットがUDPであることを確認し、宛先ポートをチェック
        if let Some(udp_packet) = UdpPacket::new(packet.packet()) { // packet() を呼び出してIPパケットを取得
            if udp_packet.get_destination() == udp_listen_port.0 {
                let payload = udp_packet.payload(); // UDPパケットのペイロードを取得

                if payload.len() < 4 {
                    eprintln!("Payload too short: {:?}", payload);
                    continue;
                }
                let chunk_num = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);

                if chunk_num == END_SIG {
                    println!("End of transmission from {:?}", addr); // 送信元アドレスも表示
                    image_reception_complete_events.write(ImageReceptionComplete);
                    return;
                }

                let data = payload[4..].to_vec();
                image_chunk_events.write(ImageChunkReceived { chunk_num, data });
            } else {
                // info!("Ignoring UDP packet on unexpected port: {}", udp_packet.get_destination());
            }
        } else {
            // debug!("Received non-UDP packet or malformed packet");
        }
    }
}

// 受信したチャンクイベントを処理するシステム
pub fn handle_image_chunks(
    mut events: EventReader<ImageChunkReceived>,
    mut received_image_data: ResMut<ReceivedImageData>,
) {
    let mut image_data = received_image_data.0.lock().unwrap();
    for event in events.read() {
        // TODO: 欠損・順不同に対応するには、Vec<Option<Vec<u8>>>のような構造で管理し、
        // 全てのチャンクが揃った時点でファイルに書き出すなどのロジックが必要です。
        println!("Processing chunk {}", event.chunk_num);
        image_data.extend_from_slice(&event.data);
    }
}

// 受信完了イベントを処理するシステム
pub fn on_image_reception_complete(
    mut events: EventReader<ImageReceptionComplete>,
    received_image_data: Res<ReceivedImageData>,
) {
    for _ in events.read() {
        println!("Finalizing image reception.");
        let image_data = received_image_data.0.lock().unwrap();
        if let Ok(mut file) = File::create("received_image.png") {
            if let Err(e) = file.write_all(&image_data) {
                eprintln!("Failed to write final image to file: {}", e);
            } else {
                println!("Image saved as received_image.png");
            }
        } else {
            eprintln!("Failed to create received_image.png file.");
        }
       //  *image_data = Vec::new();
    }
}

// 受信エラーイベントを処理するシステム
pub fn on_image_reception_error(
    mut events: EventReader<ImageReceptionError>,
) {
    for event in events.read() {
        eprintln!("Image reception error: {}", event.0);
    }
}
