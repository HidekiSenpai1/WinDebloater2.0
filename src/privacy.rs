// Módulo para protección de privacidad
use std::process::Command;

pub fn run_privacy_protection() {
    println!("[Protección de Privacidad] Desactivando telemetría y servicios de recopilación de datos...");

    // Desactivar servicios de telemetría comunes en Windows 10/11
    let services = [
        "DiagTrack", // Servicio de seguimiento de diagnóstico
        "dmwappushservice", // Servicio de envío de datos
        "WMPNetworkSvc", // Servicio de uso compartido de Windows Media Player
    ];
    for service in &services {
        let output = Command::new("powershell")
            .args(["-Command", &format!("Stop-Service -Name '{}' -Force; Set-Service -Name '{}' -StartupType Disabled", service, service)])
            .output();
        match output {
            Ok(o) if o.status.success() => println!("  Servicio '{}' desactivado.", service),
            Ok(o) => println!("  No se pudo desactivar '{}': {}", service, String::from_utf8_lossy(&o.stderr)),
            Err(e) => println!("  Error al intentar desactivar '{}': {}", service, e),
        }
    }

    // Desactivar tareas programadas de telemetría
    let tasks = [
        "\\Microsoft\\Windows\\Customer Experience Improvement Program\\Consolidator",
        "\\Microsoft\\Windows\\Customer Experience Improvement Program\\UsbCeip",
        "\\Microsoft\\Windows\\Application Experience\\ProgramDataUpdater",
    ];
    for task in &tasks {
        let output = Command::new("powershell")
            .args(["-Command", &format!("Disable-ScheduledTask -TaskName '{}' -ErrorAction SilentlyContinue", task)])
            .output();
        match output {
            Ok(o) if o.status.success() => println!("  Tarea '{}' desactivada.", task),
            Ok(o) => println!("  No se pudo desactivar tarea '{}': {}", task, String::from_utf8_lossy(&o.stderr)),
            Err(e) => println!("  Error al intentar desactivar tarea '{}': {}", task, e),
        }
    }

    // Opcional: Desactivar telemetría vía registro (requiere privilegios de administrador)
    let reg_cmd = "Set-ItemProperty -Path 'HKLM:SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection' -Name 'AllowTelemetry' -Value 0 -Force";
    let reg_output = Command::new("powershell")
        .args(["-Command", reg_cmd])
        .output();
    match reg_output {
        Ok(o) if o.status.success() => println!("  Telemetría desactivada en el registro."),
        Ok(o) => println!("  No se pudo modificar el registro: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al modificar el registro: {}", e),
    }

    println!("[Protección de Privacidad] Proceso completado.");
}
