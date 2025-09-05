use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;

use crate::{bloatware, customize, optimize, privacy, restore, win11};

#[derive(Debug, Clone)]
pub enum Message {
    BloatwareComplete(String),
    PrivacyComplete(String),
    OptimizeComplete(String),
    Win11Complete(String),
    CustomizeComplete(String),
    RestoreComplete(String),
}

#[derive(PartialEq)]
enum ActiveTab {
    Bloatware,
    Privacy,
    Optimize,
    Win11,
    Customize,
    Restore,
}

pub struct WinDebloaterApp {
    active_tab: ActiveTab,
    is_running: Arc<Mutex<bool>>,
    output_log: Arc<Mutex<Vec<String>>>,
    tx: Option<mpsc::UnboundedSender<Message>>,
    rx: Option<mpsc::UnboundedReceiver<Message>>,
    selected_apps: Vec<bool>,
}

impl Default for WinDebloaterApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            active_tab: ActiveTab::Bloatware,
            is_running: Arc::new(Mutex::new(false)),
            output_log: Arc::new(Mutex::new(Vec::new())),
            tx: Some(tx),
            rx: Some(rx),
            selected_apps: vec![false; 25], // Para las 25 apps en customize
        }
    }
}

impl eframe::App for WinDebloaterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Procesar mensajes del canal
        if let Some(rx) = &mut self.rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    Message::BloatwareComplete(output)
                    | Message::PrivacyComplete(output)
                    | Message::OptimizeComplete(output)
                    | Message::Win11Complete(output)
                    | Message::CustomizeComplete(output)
                    | Message::RestoreComplete(output) => {
                        self.output_log.lock().unwrap().push(output);
                        *self.is_running.lock().unwrap() = false;
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Título principal
            ui.heading("🛡️ WinDebloater - Optimizador de Windows");
            ui.separator();

            // Pestañas principales
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, ActiveTab::Bloatware, "🗑️ Bloatware");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Privacy, "🔒 Privacidad");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Optimize, "⚡ Optimización");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Win11, "🪟 Windows 11");
                ui.selectable_value(
                    &mut self.active_tab,
                    ActiveTab::Customize,
                    "⚙️ Personalizar",
                );
                ui.selectable_value(&mut self.active_tab, ActiveTab::Restore, "🔄 Restaurar");
            });

            ui.separator();

            // Contenido de las pestañas
            match self.active_tab {
                ActiveTab::Bloatware => self.show_bloatware_tab(ui),
                ActiveTab::Privacy => self.show_privacy_tab(ui),
                ActiveTab::Optimize => self.show_optimize_tab(ui),
                ActiveTab::Win11 => self.show_win11_tab(ui),
                ActiveTab::Customize => self.show_customize_tab(ui),
                ActiveTab::Restore => self.show_restore_tab(ui),
            }

            ui.separator();

            // Log de salida
            ui.heading("📋 Registro de Actividad");
            egui::ScrollArea::vertical()
                .id_source("activity_log_scroll")
                .max_height(200.0)
                .show(ui, |ui| {
                    let log = self.output_log.lock().unwrap();
                    for entry in log.iter() {
                        ui.label(entry);
                    }
                });
        });

        // Actualizar la UI continuamente
        ctx.request_repaint();
    }
}

impl WinDebloaterApp {
    fn show_bloatware_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Eliminación de Bloatware");
        ui.label("Elimina aplicaciones preinstaladas innecesarias de Windows.");

        ui.add_space(10.0);

        let is_running = *self.is_running.lock().unwrap();

        if ui
            .add_enabled(!is_running, egui::Button::new("🗑️ Eliminar Bloatware"))
            .clicked()
        {
            self.run_bloatware_removal();
        }

        if is_running {
            ui.label("⏳ Eliminando bloatware...");
            ui.spinner();
        }
    }

    fn show_privacy_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Protección de Privacidad");
        ui.label("Desactiva servicios de telemetría y recopilación de datos.");

        ui.add_space(10.0);

        let is_running = *self.is_running.lock().unwrap();

        if ui
            .add_enabled(!is_running, egui::Button::new("🔒 Proteger Privacidad"))
            .clicked()
        {
            self.run_privacy_protection();
        }

        if is_running {
            ui.label("⏳ Aplicando protecciones de privacidad...");
            ui.spinner();
        }
    }

    fn show_optimize_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Optimización de Windows");
        ui.label("Aplica mejoras de rendimiento al sistema.");

        ui.add_space(10.0);

        let is_running = *self.is_running.lock().unwrap();

        if ui
            .add_enabled(!is_running, egui::Button::new("⚡ Optimizar Sistema"))
            .clicked()
        {
            self.run_optimization();
        }

        if is_running {
            ui.label("⏳ Optimizando sistema...");
            ui.spinner();
        }
    }

    fn show_win11_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Ajustes para Windows 11");
        ui.label("Aplica ajustes específicos para Windows 11.");

        ui.add_space(10.0);

        let is_running = *self.is_running.lock().unwrap();

        if ui
            .add_enabled(!is_running, egui::Button::new("🪟 Aplicar Ajustes Win11"))
            .clicked()
        {
            self.run_win11_tweaks();
        }

        if is_running {
            ui.label("⏳ Aplicando ajustes de Windows 11...");
            ui.spinner();
        }
    }

    fn show_customize_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Personalización");
        ui.label("Selecciona aplicaciones específicas para eliminar.");

        ui.add_space(10.0);

        let apps = [
            "Groove Música",               // 1
            "Películas y TV",              // 2
            "Xbox",                        // 3
            "Xbox Game Overlay",           // 4
            "Xbox Gaming Overlay",         // 5
            "Xbox Identity Provider",      // 6
            "Xbox Speech to Text Overlay", // 7
            "Obtener ayuda",               // 8
            "Primeros pasos",              // 9
            "Microsoft 3D Viewer",         // 10
            "Microsoft Office Hub",        // 11
            "Juego Solitario",             // 12
            "Notas rápidas",               // 13
            "One Connect",                 // 14
            "Gente",                       // 15
            "Skype",                       // 16
            "Wallet",                      // 17
            "Alarmas y reloj",             // 18
            "Centro de comentarios",       // 19
            "Mapas",                       // 20
            "Grabadora de sonido",         // 21
            "Tu Teléfono",                 // 22
            "Paint 3D",                    // 23
            "Bing Weather",                // 24
            "Microsoft Edge (estable)",    // 25
        ];

        // Asegurar que selected_apps tenga el tamaño correcto
        if self.selected_apps.len() != apps.len() {
            self.selected_apps.resize(apps.len(), false);
        }

        egui::ScrollArea::vertical()
            .id_source("customize_apps_scroll")
            .max_height(300.0)
            .show(ui, |ui| {
                for (i, app_name) in apps.iter().enumerate() {
                    ui.checkbox(&mut self.selected_apps[i], *app_name);
                }
            });

        ui.add_space(10.0);

        let is_running = *self.is_running.lock().unwrap();
        let has_selection = self.selected_apps.iter().any(|&selected| selected);

        if ui
            .add_enabled(
                !is_running && has_selection,
                egui::Button::new("⚙️ Eliminar Seleccionadas"),
            )
            .clicked()
        {
            self.run_custom_removal();
        }

        if is_running {
            ui.label("⏳ Eliminando aplicaciones seleccionadas...");
            ui.spinner();
        }
    }

    fn show_restore_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Restauración");
        ui.label("Revierte los cambios realizados por el debloater.");

        ui.add_space(10.0);

        let is_running = *self.is_running.lock().unwrap();

        if ui
            .add_enabled(!is_running, egui::Button::new("🔄 Restaurar Sistema"))
            .clicked()
        {
            self.run_restore();
        }

        if is_running {
            ui.label("⏳ Restaurando sistema...");
            ui.spinner();
        }
    }

    // Métodos para ejecutar las funciones en hilos separados
    fn run_bloatware_removal(&mut self) {
        *self.is_running.lock().unwrap() = true;
        let tx = self.tx.as_ref().unwrap().clone();

        thread::spawn(move || {
            bloatware::run_bloatware_removal();
            let _ = tx.send(Message::BloatwareComplete(
                "Eliminación de bloatware completada.".to_string(),
            ));
        });
    }

    fn run_privacy_protection(&mut self) {
        *self.is_running.lock().unwrap() = true;
        let tx = self.tx.as_ref().unwrap().clone();

        thread::spawn(move || {
            privacy::run_privacy_protection();
            let _ = tx.send(Message::PrivacyComplete(
                "Protección de privacidad aplicada.".to_string(),
            ));
        });
    }

    fn run_optimization(&mut self) {
        *self.is_running.lock().unwrap() = true;
        let tx = self.tx.as_ref().unwrap().clone();

        thread::spawn(move || {
            optimize::run_optimization();
            let _ = tx.send(Message::OptimizeComplete(
                "Optimización del sistema completada.".to_string(),
            ));
        });
    }

    fn run_win11_tweaks(&mut self) {
        *self.is_running.lock().unwrap() = true;
        let tx = self.tx.as_ref().unwrap().clone();

        thread::spawn(move || {
            win11::run_win11_tweaks();
            let _ = tx.send(Message::Win11Complete(
                "Ajustes de Windows 11 aplicados.".to_string(),
            ));
        });
    }

    fn run_custom_removal(&mut self) {
        *self.is_running.lock().unwrap() = true;
        let tx = self.tx.as_ref().unwrap().clone();
        let _selected = self.selected_apps.clone();

        thread::spawn(move || {
            // Aquí implementarías la lógica personalizada basada en _selected
            customize::run_customization();
            let _ = tx.send(Message::CustomizeComplete(
                "Eliminación personalizada completada.".to_string(),
            ));
        });
    }

    fn run_restore(&mut self) {
        *self.is_running.lock().unwrap() = true;
        let tx = self.tx.as_ref().unwrap().clone();

        thread::spawn(move || {
            restore::run_restore();
            let _ = tx.send(Message::RestoreComplete(
                "Restauración del sistema completada.".to_string(),
            ));
        });
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/logo.png")[..])
                    .unwrap_or_default(),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "WinDebloater - Optimizador de Windows",
        options,
        Box::new(|_cc| Box::new(WinDebloaterApp::default())),
    )
}
