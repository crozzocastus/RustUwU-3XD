use std::env;
use std::process::{Command, exit};

pub fn check_and_install_dbus() {
    let os = env::consts::OS; // Detecta o sistema operacional

    if os == "linux" {
        let output = Command::new("which") // Verifica se o dbus está instalado
            .arg("dbus-launch")
            .output();

        match output {
            Ok(output) => {
                if output.stdout.is_empty() {
                    println!("O D-Bus não está instalado. Instalando dbus-x11..."); // Se dbus-launch não estiver instalado, tenta instalar
                    let install_output = Command::new("sudo")
                        .arg("apt-get")
                        .arg("install")
                        .arg("-y")
                        .arg("dbus-x11")
                        .output();

                    match install_output {
                        Ok(install_result) => {
                            if !install_result.stdout.is_empty() {
                                println!("D-Bus instalado com sucesso.");
                            } else {
                                println!("Falha ao instalar o D-Bus.");
                                exit(1);
                            }
                        }
                        Err(e) => {
                            println!("Erro ao tentar instalar o D-Bus: {}", e);
                            exit(1);
                        }
                    }
                } else {
                    println!("D-Bus já está instalado.");
                }
            }
            Err(e) => {
                println!("Erro ao verificar se o D-Bus está instalado: {}", e);
                exit(1);
            }
        }
    } else {
        println!("Este sistema não é Linux. O D-Bus não será verificado.");
    }
}
