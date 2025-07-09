use eframe::egui;

pub struct WinDebloaterApp {
    selected: Vec<usize>,
    menu: MenuOption,
}

#[derive(PartialEq)]
enum MenuOption {
    Main,
    Bloatware,
    Privacy,
    Optimize,
    Win11,
    Customize,
    Restore,
}

impl Default for WinDebloaterApp {
    fn default() -> Self {
        Self {
            selected: Vec::new(),
            menu: MenuOption::Main,
        }
    }
}

impl eframe::App for WinDebloaterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));
            ui.visuals_mut().widgets.noninteractive.bg_fill = egui::Color32::from_rgb(80, 180, 255);
            match self.menu {
                MenuOption::Main => {
                    ui.heading("WinDebloater - Made by Hideki");
                    ui.separator();
                    if ui.button("1. Eliminación de Bloatware").clicked() {
                        self.menu = MenuOption::Bloatware;
                    }
                    if ui.button("2. Protección de Privacidad").clicked() {
                        self.menu = MenuOption::Privacy;
                    }
                    if ui.button("3. Optimización de Windows").clicked() {
                        self.menu = MenuOption::Optimize;
                    }
                    if ui.button("4. Ajustes para Windows 11").clicked() {
                        self.menu = MenuOption::Win11;
                    }
                    if ui.button("5. Personalización").clicked() {
                        self.menu = MenuOption::Customize;
                    }
                    if ui.button("6. Restauración").clicked() {
                        self.menu = MenuOption::Restore;
                    }
                    if ui.button("Salir").clicked() {
                        std::process::exit(0);
                    }
                }
                MenuOption::Bloatware => {
                    ui.heading("Eliminación de Bloatware");
                    ui.label("Esta función eliminará aplicaciones preinstaladas innecesarias.");
                    if ui.button("Volver al menú principal").clicked() {
                        self.menu = MenuOption::Main;
                    }
                }
                MenuOption::Privacy => {
                    ui.heading("Protección de Privacidad");
                    ui.label("Esta función desactivará la telemetría y servicios de recopilación de datos.");
                    if ui.button("Volver al menú principal").clicked() {
                        self.menu = MenuOption::Main;
                    }
                }
                MenuOption::Optimize => {
                    ui.heading("Optimización de Windows");
                    ui.label("Esta función mejorará el rendimiento del sistema.");
                    if ui.button("Volver al menú principal").clicked() {
                        self.menu = MenuOption::Main;
                    }
                }
                MenuOption::Win11 => {
                    ui.heading("Ajustes para Windows 11");
                    ui.label("Esta función aplicará ajustes específicos para Windows 11.");
                    if ui.button("Volver al menú principal").clicked() {
                        self.menu = MenuOption::Main;
                    }
                }
                MenuOption::Customize => {
                    ui.heading("Personalización");
                    ui.label("Selecciona las aplicaciones que deseas eliminar:");
                    let apps = [
                        ("Groove Música", "Microsoft.ZuneMusic"),
                        ("Películas y TV", "Microsoft.ZuneVideo"),
                        ("Xbox", "Microsoft.XboxApp"),
                        ("Xbox Game Overlay", "Microsoft.XboxGameOverlay"),
                        ("Xbox Gaming Overlay", "Microsoft.XboxGamingOverlay"),
                        ("Xbox Identity Provider", "Microsoft.XboxIdentityProvider"),
                        ("Xbox Speech to Text Overlay", "Microsoft.XboxSpeechToTextOverlay"),
                        ("Obtener ayuda", "Microsoft.GetHelp"),
                        ("Primeros pasos", "Microsoft.Getstarted"),
                        ("Microsoft 3D Viewer", "Microsoft.Microsoft3DViewer"),
                        ("Microsoft Office Hub", "Microsoft.MicrosoftOfficeHub"),
                        ("Juego Solitario", "Microsoft.MicrosoftSolitaireCollection"),
                        ("Notas rapidas", "Microsoft.MicrosoftStickyNotes"),
                        ("One Connect", "Microsoft.OneConnect"),
                        ("Gente", "Microsoft.People"),
                        ("Skype", "Microsoft.SkypeApp"),
                        ("Wallet", "Microsoft.Wallet"),
                        ("Alarmas y reloj", "Microsoft.WindowsAlarms"),
                        ("Centro de comentarios", "Microsoft.WindowsFeedbackHub"),
                        ("Mapas", "Microsoft.WindowsMaps"),
                        ("Grabadora de sonido", "Microsoft.WindowsSoundRecorder"),
                        ("Tu Teléfono", "Microsoft.YourPhone"),
                        ("Paint 3D", "Microsoft.MSPaint"),
                        ("Bing Weather", "Microsoft.BingWeather"),
                        ("Microsoft Edge (estable)", "Microsoft.MicrosoftEdge.Stable"),
                    ];
                    for (i, (nombre, _)) in apps.iter().enumerate() {
                        let mut checked = self.selected.contains(&i);
                        if ui.checkbox(&mut checked, *nombre).clicked() {
                            if checked {
                                self.selected.push(i);
                            } else {
                                self.selected.retain(|&x| x != i);
                            }
                        }
                    }
                    if ui.button("Eliminar seleccionadas").clicked() {
                        for &i in &self.selected {
                            let (_nombre, paquete) = apps[i];
                            // Aquí puedes llamar a la lógica de eliminación real
                            println!("Eliminar: {}", paquete);
                        }
                        self.selected.clear();
                    }
                    if ui.button("Volver al menú principal").clicked() {
                        self.menu = MenuOption::Main;
                    }
                }
                MenuOption::Restore => {
                    ui.heading("Restauración");
                    ui.label("Esta función revertirá los cambios realizados.");
                    if ui.button("Volver al menú principal").clicked() {
                        self.menu = MenuOption::Main;
                    }
                }
            }
        });
    }
}
