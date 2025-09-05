// Módulo para eliminación de bloatware
use std::process::Command;

pub fn run_bloatware_removal() {
    println!("[Eliminación de Bloatware] Eliminando aplicaciones preinstaladas innecesarias...");

    // Lista de apps comunes a eliminar (puedes personalizar esta lista)
    let apps = [
        "Microsoft.ZuneMusic",      // Groove Música
        "Microsoft.ZuneVideo",      // Películas y TV
        "Microsoft.XboxApp",        // Xbox
        "Microsoft.XboxGameOverlay",
        "Microsoft.XboxGamingOverlay",
        "Microsoft.XboxIdentityProvider",
        "Microsoft.XboxSpeechToTextOverlay",
        "Microsoft.GetHelp",        // Obtener ayuda
        "Microsoft.Getstarted",     // Primeros pasos
        "Microsoft.Microsoft3DViewer",
        "Microsoft.MicrosoftOfficeHub",
        "Microsoft.MicrosoftSolitaireCollection",
        "Microsoft.MicrosoftStickyNotes",
        "Microsoft.OneConnect",
        "Microsoft.People",
        "Microsoft.SkypeApp",
        "Microsoft.Wallet",
        "Microsoft.WindowsAlarms",
        "Microsoft.WindowsFeedbackHub",
        "Microsoft.WindowsMaps",
        "Microsoft.WindowsSoundRecorder",
        "Microsoft.YourPhone",
        "Microsoft.MSPaint",
        "Microsoft.BingWeather",
        "Microsoft.MicrosoftEdge.Stable",
    ];
    for app in &apps {
        let output = Command::new("powershell")
            .args(["-Command", &format!("Get-AppxPackage -Name {} | Remove-AppxPackage -ErrorAction SilentlyContinue", app)])
            .output();
        match output {
            Ok(o) if o.status.success() => println!("  App '{}' eliminada (o no estaba instalada).", app),
            Ok(o) => println!("  No se pudo eliminar '{}': {}", app, String::from_utf8_lossy(&o.stderr)),
            Err(e) => println!("  Error al intentar eliminar '{}': {}", app, e),
        }
    }

    println!("[Eliminación de Bloatware] Proceso completado.");
}
