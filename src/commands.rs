use std::process::Command;

pub fn actualizar_sistema() {
    let comando = "sudo pacman -Syu; echo; read -p 'Presione Enter para cerrar...'";

    Command::new("konsole")
        .args(["-e", "bash", "-c", comando])
        .spawn()
        .expect("Error abriendo Konsole");
}

pub fn limpiar_cache() {
    let comando = r#"
clear
echo "=== Limpieza de cache pacman ==="
echo
sudo pacman -Sc
echo
echo "Proceso terminado"
read -p "Presione Enter para cerrar..."
"#;

    Command::new("konsole")
        .args(["-e", "bash", "-c", comando])
        .spawn()
        .expect("Error abriendo Konsole");
}

pub fn eliminar_huerfanos() {
    let comando = r#"
clear
echo "=== Eliminar paquetes huérfanos ==="
echo
sudo pacman -Rns $(pacman -Qtdq)
echo
echo "Proceso terminado"
read -p "Presione Enter para cerrar..."
"#;

    Command::new("konsole")
        .args(["-e", "bash", "-c", comando])
        .spawn()
        .expect("Error abriendo Konsole");
}

pub fn evaluar_mirrors() {
    let comando = r#"
clear
echo "=== Evaluar mirrors con rate-mirrors ==="
echo
rate-mirrors --max-mirrors-to-output=10 --protocol=https arch | sudo tee /etc/pacman.d/mirrorlist
echo
echo "Proceso terminado"
read -p "Presione Enter para cerrar..."
"#;

    Command::new("konsole")
        .args(["-e", "bash", "-c", comando])
        .spawn()
        .expect("Error abriendo Konsole");
}
