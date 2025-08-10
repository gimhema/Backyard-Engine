
use eframe::egui;
use egui::{Layout, ScrollArea, TextEdit, Align, Align2};

pub struct ServerMonitorApp {
    pub log_messages: Vec<String>,    // 서버 로그 메시지 저장
    pub connections: Vec<String>,     // 현재 연결된 클라이언트 정보
    pub message_input: String,        // 하단 메시지 입력창 텍스트
}

impl ServerMonitorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // ---- 폰트 설정 ----
        let mut fonts = egui::FontDefinitions::default();

        // TrueType 폰트 로드
        fonts.font_data.insert(
            "nanum".to_owned(),
            egui::FontData::from_owned(
                std::fs::read("fonts/korean_font.ttf").expect("폰트 파일을 찾을 수 없습니다"),
            ),
        );

        // Proportional (일반 UI 폰트) 맨 앞에 추가
        fonts.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "nanum".to_owned());

        // Monospace (코드/로그용 폰트)에도 추가
        fonts.families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "nanum".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        // ---- 폰트 설정 끝 ----

        Self::default()
    }
}

impl Default for ServerMonitorApp {
    fn default() -> Self {
        Self {
            log_messages: vec![
                // "[INFO] 서버 시작 대기중...".into(),
                // "[INFO] 연결됨: Client #1".into(),
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
                    if !self.message_input.is_empty() {
                        self.command_action(String::from(&self.message_input));
                    }
                }
            });
        });

        // egui::SidePanel::left("left_panel")
        //     .min_width(150.0)
        //     .show(ctx, |ui| {
        //         ui.heading("서버 제어");
        //         if ui.button("Server Connect").clicked() {
        //             // TODO: 서버 소켓 bind & listen 시작
        //             self.add_log_message("[INFO] 서버 연결을 요청합니다".into());
        //         }
        //     });

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



pub fn StartMainApp()  -> eframe::Result<()>  {
    let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0]),
    ..Default::default()
    };

    eframe::run_native(
        "Backyard Monitor",
        options,
        Box::new(|cc| Box::new(ServerMonitorApp::new(cc))),
    )
}

