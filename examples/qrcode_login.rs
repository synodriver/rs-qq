use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

use rs_qq::client::handler::DefaultHandler;
use rs_qq::client::Client;
use rs_qq::device::Device;
use rs_qq::ext::common::after_login;
use rs_qq::version::{get_version, Protocol};
use rs_qq::{LoginResponse, QRCodeState};

#[tokio::main]
async fn main() -> Result<()> {
    let env = tracing_subscriber::EnvFilter::from("rs_qq=debug,sled=warn,info");
    tracing_subscriber::fmt()
        .with_env_filter(env)
        .without_time()
        .init();

    let device = match Path::new("device.json").exists() {
        true => serde_json::from_str(
            &tokio::fs::read_to_string("device.json")
                .await
                .expect("failed to read device.json"),
        )
        .expect("failed to parse device json"),
        false => Device::random(),
    };
    tokio::fs::write("device.json", serde_json::to_string(&device).unwrap())
        .await
        .expect("failed to write device.json"); //todo

    let client = Arc::new(Client::new(
        device,
        get_version(Protocol::IPad),
        DefaultHandler,
    ));
    let stream = TcpStream::connect(client.get_address())
        .await
        .expect("failed to connect");
    let c = client.clone();
    let handle = tokio::spawn(async move { c.start(stream).await });
    tokio::task::yield_now().await; // 等一下，确保连上了
    let mut resp = client.fetch_qrcode().await.expect("failed to fetch qrcode");

    // // vvv 如果不关心二维码状态，可以用这个替换下面的 vvv
    // use rs_qq::ext::login::auto_query_qrcode;
    // match resp {
    //     QRCodeState::QRCodeImageFetch {
    //         ref image_data,
    //         ref sig,
    //     } => {
    //         tokio::fs::write("qrcode.png", &image_data)
    //             .await
    //             .expect("failed to write file");
    //         if let Err(err) = auto_query_qrcode(&client, sig).await {
    //             panic!("登录失败 {}", err)
    //         };
    //     }
    //     _ => {
    //         panic!("resp error")
    //     }
    // }
    // // ^^^ 如果不关心二维码状态，可以用这个替换下面的 ^^^

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

    // vvv 如果关心二维码状态，可以用这个 vvv
    let mut image_sig = Bytes::new();
    loop {
        match resp {
            QRCodeState::QRCodeImageFetch {
                ref image_data,
                ref sig,
            } => {
                tokio::fs::write("qrcode.png", &image_data)
                    .await
                    .expect("failed to write file");
                image_sig = sig.clone();
                tracing::info!(target = "rs_qq", "二维码: qrcode.png");
            }
            QRCodeState::QRCodeWaitingForScan => {
                tracing::info!(target = "rs_qq", "二维码待扫描")
            }
            QRCodeState::QRCodeWaitingForConfirm => {
                tracing::info!(target = "rs_qq", "二维码待确认")
            }
            QRCodeState::QRCodeTimeout => {
                tracing::info!(target = "rs_qq", "二维码已超时，重新获取");
                if let QRCodeState::QRCodeImageFetch {
                    ref image_data,
                    ref sig,
                } = client.fetch_qrcode().await.expect("failed to fetch qrcode")
                {
                    tokio::fs::write("qrcode.png", &image_data)
                        .await
                        .expect("failed to write file");
                    image_sig = sig.clone();
                    tracing::info!(target = "rs_qq", "二维码: qrcode.png");
                }
            }
            QRCodeState::QRCodeConfirmed {
                ref tmp_pwd,
                ref tmp_no_pic_sig,
                ref tgt_qr,
                ..
            } => {
                tracing::info!(target = "rs_qq", "二维码已确认");
                let mut login_resp = client
                    .qrcode_login(tmp_pwd, tmp_no_pic_sig, tgt_qr)
                    .await
                    .expect("failed to qrcode login");
                if let LoginResponse::DeviceLockLogin { .. } = login_resp {
                    login_resp = client
                        .device_lock_login()
                        .await
                        .expect("failed to device lock login");
                }
                tracing::info!(target = "rs_qq", "{:?}", login_resp);
                break;
            }
            QRCodeState::QRCodeCanceled => {
                panic!("二维码已取消")
            }
        }
        sleep(Duration::from_secs(5)).await;
        resp = client
            .query_qrcode_result(&image_sig)
            .await
            .expect("failed to query qrcode result");
    }
    // ^^^ 如果不关心二维码状态，可以用这个 ^^^

    after_login(&client).await;
    {
        client
            .reload_friends()
            .await
            .expect("failed to reload friend list");
        tracing::info!(target = "rs_qq", "{:?}", client.friends.read().await);
        client
            .reload_groups()
            .await
            .expect("failed to reload group list");
        tracing::info!(target = "rs_qq", "{:?}", client.groups.read().await);
    }

    handle.await.unwrap();
    Ok(())
}
