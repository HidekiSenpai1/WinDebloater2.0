// Módulo para ajustes de Windows 11
use std::process::Command;

pub fn run_win11_tweaks() {
    println!("[Ajustes para Windows 11] Aplicando ajustes específicos...");

    // Restaurar menú contextual clásico
    let reg_cmd = "reg add \"HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32\" /f /ve";
    let reg_output = Command::new("powershell")
        .args(["-Command", reg_cmd])
        .output();
    match reg_output {
        Ok(o) if o.status.success() => println!("  Menú contextual clásico restaurado."),
        Ok(o) => println!("  No se pudo restaurar el menú contextual: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al modificar el registro: {}", e),
    }

    // Desactivar widgets
    let widgets_cmd = "reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" /v TaskbarDa /t REG_DWORD /d 0 /f";
    let widgets_output = Command::new("powershell")
        .args(["-Command", widgets_cmd])
        .output();
    match widgets_output {
        Ok(o) if o.status.success() => println!("  Widgets desactivados."),
        Ok(o) => println!("  No se pudo desactivar widgets: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al desactivar widgets: {}", e),
    }

    // Desactivar icono de chat
    let chat_cmd = "reg add \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\" /v TaskbarMn /t REG_DWORD /d 0 /f";
    let chat_output = Command::new("powershell")
        .args(["-Command", chat_cmd])
        .output();
    match chat_output {
        Ok(o) if o.status.success() => println!("  Icono de chat desactivado."),
        Ok(o) => println!("  No se pudo desactivar el icono de chat: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => println!("  Error al desactivar el icono de chat: {}", e),
    }

    println!("[Ajustes para Windows 11] Proceso completado.");
}
