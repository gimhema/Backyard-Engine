use eframe::egui;
use egui::{Layout, ScrollArea, TextEdit, Align, Align2};

struct ServerMonitorApp {
    log_messages: Vec<String>,    // 서버 로그 메시지 저장
    connections: Vec<String>,     // 현재 연결된 클라이언트 정보
    message_input: String,        // 하단 메시지 입력창 텍스트
}

impl Default for ServerMonitorApp {
    fn default() -> Self {
        Self {
            log_messages: vec![
                "[INFO] 서버 시작 대기중...".into(),
                "[INFO] 연결됨: Client #1".into(),
            ],
            connections: vec!["Client #1".into(), "Client #2".into()],
            message_input: String::new(),
        }
    }
}

impl eframe::App for ServerMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("메시지 입력:");
                ui.text_edit_singleline(&mut self.message_input);
                if ui.button("보내기").clicked() {
                    // TODO: 서버로 메시지를 전송하는 로직
                    // 1. self.message_input의 값을 서버 소켓에 write
                    // 2. 전송 후 self.message_input.clear()
                }
            });
        });

        egui::SidePanel::left("left_panel")
            .min_width(150.0)
            .show(ctx, |ui| {
                ui.heading("서버 제어");
                if ui.button("서버 ON").clicked() {
                    // TODO: 서버 소켓 bind & listen 시작
                }
                if ui.button("서버 OFF").clicked() {
                    // TODO: 서버 소켓 close 및 모든 connection 종료
                }
            });

        egui::SidePanel::right("right_panel")
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.heading("연결된 클라이언트");
                ScrollArea::vertical().show(ui, |ui| {
                    for conn in &self.connections {
                        ui.label(conn);
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("서버 로그");
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                for log in &self.log_messages {
                    ui.label(log);
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]), // 창 초기 크기 지정
        ..Default::default()
    };
    eframe::run_native(
        "서버 모니터링 툴",
        options,
        Box::new(|_cc| Box::new(ServerMonitorApp::default())),
    )
}
