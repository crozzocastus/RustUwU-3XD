mod check; // Módulo responsável por checagem do sistema operacional
mod database; // Módulo para manipulação de banco de dados
mod validate; // Módulo para validação de arquivos

use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType, FileChooserDialog, FileChooserAction, MessageDialog, ButtonsType};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    check::check_and_install_dbus(); // Verifica o sistema operacional e instala o dbus se necessário
    database::handle_connection();  // Trata a conexão com o banco de dados
    database::handle_error(); // Verifica se há erros ao buscar dados
    gtk::init().expect("Failed to initialize GTK."); // Inicializa o GTK
    
    let window = create_main_window(); // Cria a janela principal e configura seus componentes
    let vbox = create_layout(&window); // Layout da interface

    window.add(&vbox);
    window.show_all(); // Mostrar tudo

    gtk::main(); // Iniciar o loop GTK
}

// Função que cria a janela principal
fn create_main_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("RUST");
    window.set_default_size(500, 500);
    window
}

// Função que cria e organiza o layout da interface
fn create_layout(window: &Window) -> gtk::Box {
    let label = create_label();
    let counter = Rc::new(RefCell::new(0));

    let increment_button = create_increment_button(&label, &counter);
    let file_button = create_file_button(window.clone()); // Clone the window for use inside closure

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox.pack_start(&label, false, false, 0);
    vbox.pack_start(&increment_button, false, false, 0);
    vbox.pack_start(&file_button, false, false, 0);

    vbox
}

// Função que cria o label para exibição do número
fn create_label() -> Label {
    Label::new(Some("0"))
}

// Função que cria o botão de incremento
fn create_increment_button(label: &Label, counter: &Rc<RefCell<i32>>) -> Button {
    let increment_button = Button::with_label("Incrementar");
    increment_button.connect_clicked({
        let label = label.clone(); // Clonando a referência do label
        let counter = Rc::clone(&counter); // Clonando a referência do counter
        move |_| {
            *counter.borrow_mut() += 1;
            label.set_text(&counter.borrow().to_string());
        }
    });
    increment_button
}

// Função que cria o botão de busca de arquivos
fn create_file_button(window: Window) -> Button {
    let file_button = Button::with_label("Buscar Arquivo");
    file_button.connect_clicked(move |_| {
        open_file_dialog(&window); // Passa a referência da janela
    });
    file_button
}

// Função que abre o diálogo de busca de arquivos
fn open_file_dialog(window: &Window) {
    // Criação de um dialog para o usuário escolher um arquivo
    let dialog = FileChooserDialog::new(
        Some("Escolher um Arquivo"),
        Some(window), // Passa o objeto `window` diretamente aqui
        FileChooserAction::Open,
    );
    
    dialog.add_button("Cancelar", gtk::ResponseType::Cancel);
    dialog.add_button("Abrir", gtk::ResponseType::Accept);

    // Mostra o diálogo e captura a resposta
    if dialog.run() == gtk::ResponseType::Accept {
        // Se o usuário escolher um arquivo, obtém o nome do arquivo selecionado
        let file_name = dialog.get_filename().unwrap().to_str().unwrap().to_string();
        
        // Verifica se o arquivo tem a extensão .json.gz antes de prosseguir
        if validate::is_valid_file(&file_name) {
            show_success_message(window, &file_name);
        } else {
            show_error_message(window);
        }
    }

    dialog.close();
}

// Função que exibe uma mensagem de sucesso
fn show_success_message(window: &Window, file_name: &str) {
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

// Função que exibe uma mensagem de erro
fn show_error_message(window: &Window) {
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
