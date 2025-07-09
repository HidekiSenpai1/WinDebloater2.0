use std::process::Command;
use std::io::{self, Write};

// Módulo para personalización
pub fn run_customization() {
    println!("[Personalización] Selecciona las aplicaciones que deseas eliminar:");

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
        println!("{}. {}", i + 1, nombre);
    }
    println!("0. Cancelar");
    print!("Ingresa los números de las apps a eliminar separados por coma (ej: 1,3,5): ");
    io::stdout().flush().unwrap();

    let mut seleccion = String::new();
    io::stdin().read_line(&mut seleccion).unwrap();
    let indices: Vec<usize> = seleccion
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .filter(|&i| i > 0 && i <= apps.len())
        .collect();

    for i in indices {
        let (nombre, paquete) = apps[i - 1];
        let output = Command::new("powershell")
            .args(["-Command", &format!("Get-AppxPackage -Name {} | Remove-AppxPackage -ErrorAction SilentlyContinue", paquete)])
            .output();
        match output {
            Ok(o) if o.status.success() => println!("  '{}' eliminada.", nombre),
            Ok(o) => println!("  No se pudo eliminar '{}': {}", nombre, String::from_utf8_lossy(&o.stderr)),
            Err(e) => println!("  Error al intentar eliminar '{}': {}", nombre, e),
        }
    }

    println!("[Personalización] Proceso completado.");
}
