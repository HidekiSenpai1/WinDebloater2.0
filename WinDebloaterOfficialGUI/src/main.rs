mod bloatware;
mod customize;
mod optimize;
mod privacy;
mod restore;
mod win11;
mod gui;

use std::env;

fn main() -> Result<(), eframe::Error> {
    // Verificar si se ejecuta con argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--cli" {
        // Modo consola (para compatibilidad)
        run_cli_mode();
        Ok(())
    } else {
        // Modo GUI (por defecto)
        gui::run_gui()
    }
}

fn run_cli_mode() {
    use std::io::{self, Write};
    
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
            "0" => {
                println!("Saliendo...");
                break;
            }
            _ => println!("Opción no válida. Intenta de nuevo."),
        }
    }
}
