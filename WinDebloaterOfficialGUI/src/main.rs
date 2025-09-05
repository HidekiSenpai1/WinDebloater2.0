mod bloatware;
mod privacy;
mod optimize;
mod win11;
mod customize;
mod restore;
mod gui;

use std::io::{self, Write};

fn main() {
    // Lanzar la GUI directamente
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Some([480.0, 600.0].into());
    eframe::run_native(
        "WinDebloater - Made by Hideki",
        native_options,
        Box::new(|_cc| Box::new(gui::WinDebloaterApp::default())),
    ).unwrap();

    loop {
        println!("\nWinDebloater - Selecciona una opción:");
        println!("1. Eliminación de Bloatware");
        println!("2. Protección de Privacidad");
        println!("3. Optimización de Windows");
        println!("4. Ajustes para Windows 11");
        println!("5. Personalización");
        println!("6. Restauración");
        println!("0. Salir");
        print!("Opción: ");
        io::stdout().flush().unwrap();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();
        match opcion.trim() {
            "1" => bloatware::run_bloatware_removal(),
            "2" => privacy::run_privacy_protection(),
            "3" => optimize::run_optimization(),
            "4" => win11::run_win11_tweaks(),
            "5" => customize::run_customization(),
            "6" => restore::run_restore(),
            "0" => { println!("Saliendo..."); break; },
            _ => println!("Opción no válida. Intenta de nuevo."),
        }
    }
}
