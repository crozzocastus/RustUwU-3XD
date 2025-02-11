mod check; // Módulo responsável por checagem do sistema operacional
mod database; // Módulo para manipulação de banco de dados
mod validate; // Módulo para validação de arquivos
mod api; // Módulo para requisições

use tokio::runtime::Runtime;
use gtk::prelude::*;
use gtk::{
    Button, Window, WindowType, FileChooserDialog, FileChooserAction, MessageDialog, ButtonsType,
};
use sqlx::mysql::MySqlPool;
use crate::database::init_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); // Carrega variáveis do arquivo .env
    check::check_and_install_dbus(); // Verifica o sistema operacional e instala o dbus se necessário

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.expect("Failed to create pool.");
    let _ = database::handle_connection().await;  // Trata a conexão com o banco de dados
    database::handle_error(&pool).await; // Verifica se há erros ao buscar dados
    init_server().await?; // Inicializa o servidor Actix

    gtk::init().expect("Failed to initialize GTK."); // Inicializa o GTK
    
    let window = create_main_window(); // Cria a janela principal e configura seus componentes
    let vbox = create_layout(&window); // Layout da interface

    window.add(&vbox);
    window.show_all(); // Mostrar tudo

    gtk::main(); // Iniciar o loop GTK

    Ok(())
}

fn create_main_window() -> Window { // Cria a janela principal
    let window = Window::new(WindowType::Toplevel);
    window.set_title("RUST");
    window.set_default_size(500, 500);
    window
}

fn create_layout(window: &Window) -> gtk::Box { // Cria e organiza o layout da interface
    let file_button = create_file_button(window.clone()); // Clone the window for use inside closure
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    vbox.pack_start(&file_button, false, false, 0);
    vbox
}

fn create_file_button(window: Window) -> Button { // Cria o botão de busca de arquivos
    let file_button = Button::with_label("Buscar Arquivo");
    file_button.connect_clicked(move |_| {
        open_file_dialog(&window); // Passa a referência da janela
    });
    file_button
}

fn open_file_dialog(window: &Window) { // Abre o diálogo de busca de arquivos
    let dialog = FileChooserDialog::new( // Criação de um dialogo para o usuário escolher um arquivo
        Some("Escolher um Arquivo"),
        Some(window), // Passa o objeto `window` diretamente aqui
        FileChooserAction::Open,
    );
    dialog.add_button("Cancelar", gtk::ResponseType::Cancel);
    dialog.add_button("Abrir", gtk::ResponseType::Accept);
    
    if dialog.run() == gtk::ResponseType::Accept { // Mostra o diálogo e captura a resposta
        let file_name = dialog.get_filename().unwrap().to_str().unwrap().to_string(); // Obtém o nome do arquivo selecionado
        
        if validate::is_valid_file(&file_name) { // Verificação de extensão .json.gz
            show_success_message(window, &file_name);
            // Envia o arquivo para o servidor
            let file_name_clone = file_name.clone();
            glib::idle_add_local(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(api::send_file_to_server(&file_name_clone)).unwrap();
                Continue(false)
            });
        } else {
            show_error_message(window);
        }
    }
    dialog.close();
}

fn show_success_message(window: &Window, file_name: &str) { // Exibe uma mensagem de sucesso
    let message = format!("Arquivo '{}' recebido com sucesso!", file_name);
    let msg_dialog = MessageDialog::new(
        Some(window), // Passa o objeto `window` diretamente
        gtk::DialogFlags::empty(),
        gtk::MessageType::Info,
        ButtonsType::Ok,
        &message,
    );
    msg_dialog.run();
    msg_dialog.close();
}

fn show_error_message(window: &Window) { // Exibe uma mensagem de erro
    let msg_dialog = MessageDialog::new(
        Some(window), // Passa o objeto `window` diretamente
        gtk::DialogFlags::empty(),
        gtk::MessageType::Error,
        ButtonsType::Ok,
        "Arquivo inválido! Apenas arquivos .json.gz são permitidos.",
    );
    msg_dialog.run();
    msg_dialog.close();
}