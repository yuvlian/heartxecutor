#![windows_subsystem = "windows"] // to hide console

mod proto;

use eframe::egui;
use rfd::FileDialog;

use prost::Message;
use proto::{ClientUploadData, PlayerHeartBeatCsReq};
use amia_packet::net_packet::NetPacket;
use std::io::Write;
use std::net::TcpStream;

const MAGIC_TIME: u64 = 11112222;
const PLAYER_HEART_BEAT_CS_REQ_CMD_ID: u16 = 90;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "heartxecutor",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    file_content: String,
    addr: String,
    send_result: Option<String>,
    open_result: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_content: include_str!("./default.lua").to_string(),
            addr: String::from("127.0.0.1:23301"),
            send_result: None,
            open_result: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("made by yuvlian <3");
            ui.separator();
            if ui.button("Open from file").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => {
                            self.file_content = content;
                            self.open_result = Some(format!("Loaded {:?}", path));
                        }
                        Err(e) => {
                            self.open_result = Some(format!("Failed to load file: {}", e));
                        }
                    }
                }
            }
            ui.add_sized(
                [ui.available_width(), 200.0],
                egui::TextEdit::multiline(&mut self.file_content),
            );

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("SEND").clicked() {
                    let msg = PlayerHeartBeatCsReq {
                        client_time_ms: MAGIC_TIME,
                        unknown: 0,
                        lua_file: Some(ClientUploadData {
                            file_id: String::new(),
                            file_content: self.file_content.clone(),
                        }),
                    };

                    let packet = NetPacket {
                        cmd: PLAYER_HEART_BEAT_CS_REQ_CMD_ID,
                        head: Vec::new(),
                        body: msg.encode_to_vec(),
                    };

                    let buf = Box::<[u8]>::from(packet);

                    match TcpStream::connect(self.addr.clone()) {
                        Ok(mut stream) => {
                            if let Err(e) = stream.write_all(&buf) {
                                self.send_result = Some(format!("Write failed: {}", e));
                            } else {
                                self.send_result = Some("Sent successfully".into());
                            }
                        }
                        Err(e) => self.send_result = Some(format!("Connect failed: {}", e)),
                    }
                }

                ui.label("addr");
                ui.text_edit_singleline(&mut self.addr);
            });

            if let Some(result) = &self.open_result {
                ui.label(result);
            }

            if let Some(result) = &self.send_result {
                ui.label(result);
            }
        });
    }
}
