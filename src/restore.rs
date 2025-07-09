// Módulo para restauración
use std::process::Command;

pub fn run_restore() {
    println!("[Restauración] Revirtiendo cambios realizados por el debloater...");

    // Restaurar servicios de telemetría
    let services = [
        "DiagTrack",
        "dmwappushservice",
        "WMPNetworkSvc",
    ];
    for service in &services {
        let output = Command::new("powershell")
            .args(["-Command", &format!("Set-Service -Name '{}' -StartupType Automatic; Start-Service -Name '{}'", service, service)])
            .output();
        match output {
            Ok(o) if o.status.success() => println!("  Servicio '{}' restaurado.", service),
            Ok(o) => println!("  No se pudo restaurar '{}': {}", service, String::from_utf8_lossy(&o.stderr)),
            Err(e) => println!("  Error al intentar restaurar '{}': {}", service, e),
        }
    }

    // Restaurar tareas programadas de telemetría
    let tasks = [
        "\\Microsoft\\Windows\\Customer Experience Improvement Program\\Consolidator",
        "\\Microsoft\\Windows\\Customer Experience Improvement Program\\UsbCeip",
        "\\Microsoft\\Windows\\Application Experience\\ProgramDataUpdater",
    ];
    for task in &tasks {
        let output = Command::new("powershell")
            .args(["-Command", &format!("Enable-ScheduledTask -TaskName '{}' -ErrorAction SilentlyContinue", task)])
            .output();
        match output {
            Ok(o) if o.status.success() => println!("  Tarea '{}' restaurada.", task),
            Ok(o) => println!("  No se pudo restaurar tarea '{}': {}", task, String::from_utf8_lossy(&o.stderr)),
            Err(e) => println!("  Error al intentar restaurar tarea '{}': {}", task, e),
        }
    }

    // Restaurar telemetría en el registro
    let reg_cmd = "Set-ItemProperty -Path 'HKLM:SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection' -Name 'AllowTelemetry' -Value 1 -Force";
    let reg_output = Command::new("powershell")
        .args(["-Command", reg_cmd])
        .output();
    match reg_output {
        Ok(o) if o.status.success() => println!("  Telemetría restaurada en el registro."),
        Ok(o) => println!("  No se pudo restaurar el registro: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al restaurar el registro: {}", e),
    }

    // Restaurar animaciones visuales
    let anim_cmd = "Set-ItemProperty -Path 'HKCU:Control Panel\\Desktop' -Name 'UserPreferencesMask' -Value ([byte[]](0x9e,0x3e,0x07,0x80,0x12,0x00,0x00,0x00))";
    let anim_output = Command::new("powershell")
        .args(["-Command", anim_cmd])
        .output();
    match anim_output {
        Ok(o) if o.status.success() => println!("  Animaciones visuales restauradas."),
        Ok(o) => println!("  No se pudo restaurar animaciones: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al restaurar animaciones: {}", e),
    }

    // Restaurar inicio rápido
    let fastboot_cmd = "powercfg /hibernate on";
    let fastboot_output = Command::new("powershell")
        .args(["-Command", fastboot_cmd])
        .output();
    match fastboot_output {
        Ok(o) if o.status.success() => println!("  Inicio rápido restaurado."),
        Ok(o) => println!("  No se pudo restaurar inicio rápido: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al restaurar inicio rápido: {}", e),
    }

    println!("[Restauración] Proceso completado.");
}
