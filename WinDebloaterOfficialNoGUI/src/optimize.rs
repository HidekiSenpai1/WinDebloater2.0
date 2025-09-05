// Módulo para optimización de Windows
use std::process::Command;

pub fn run_optimization() {
    println!("[Optimización de Windows] Aplicando mejoras de rendimiento...");

    // Desactivar animaciones visuales
    let anim_cmd = "Set-ItemProperty -Path 'HKCU:Control Panel\\Desktop' -Name 'UserPreferencesMask' -Value ([byte[]](0x90,0x12,0x03,0x80,0x10,0x00,0x00,0x00))";
    let anim_output = Command::new("powershell")
        .args(["-Command", anim_cmd])
        .output();
    match anim_output {
        Ok(o) if o.status.success() => println!("  Animaciones visuales desactivadas."),
        Ok(o) => println!("  No se pudo desactivar animaciones: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al modificar animaciones: {}", e),
    }

    // Desactivar inicio rápido
    let fastboot_cmd = "powercfg /hibernate off";
    let fastboot_output = Command::new("powershell")
        .args(["-Command", fastboot_cmd])
        .output();
    match fastboot_output {
        Ok(o) if o.status.success() => println!("  Inicio rápido desactivado."),
        Ok(o) => println!("  No se pudo desactivar inicio rápido: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al desactivar inicio rápido: {}", e),
    }

    // Limpiar archivos temporales
    let clean_cmd = "Remove-Item -Path $env:TEMP/* -Recurse -Force -ErrorAction SilentlyContinue";
    let clean_output = Command::new("powershell")
        .args(["-Command", clean_cmd])
        .output();
    match clean_output {
        Ok(o) if o.status.success() => println!("  Archivos temporales eliminados."),
        Ok(o) => println!("  No se pudieron eliminar archivos temporales: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al limpiar temporales: {}", e),
    }

    println!("[Optimización de Windows] Proceso completado.");
}
