use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType, FileChooserDialog, FileChooserAction, MessageDialog, ButtonsType};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Inicializa o GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Criação da janela principal
    let window = Window::new(WindowType::Toplevel);
    window.set_title("RUST");
    window.set_default_size(300, 100);

    // Criação de um rótulo (label) para exibir o número
    let label = Label::new(Some("0"));
    
    // Contador armazenado em uma variável compartilhada
    let counter = Rc::new(RefCell::new(0));

    // Criação do botão de incrementar
    let increment_button = Button::with_label("Incrementar");
    increment_button.connect_clicked({
        let label = label.clone(); // Clonando a referência do label
        let counter = Rc::clone(&counter); // Clonando a referência do counter
        move |_| {
            *counter.borrow_mut() += 1;
            label.set_text(&counter.borrow().to_string());
        }
    });

    // Criação do campo de busca de arquivos
    let file_button = Button::with_label("Buscar Arquivo");
    file_button.connect_clicked({
        let window = window.clone(); // Clonando a referência de window
        move |_| {
            // Criação de um dialog para o usuário escolher um arquivo
            let dialog = FileChooserDialog::new(
                Some("Escolher um Arquivo"),
                Some(&window),
                FileChooserAction::Open,
            );
            
            dialog.add_button("Cancelar", gtk::ResponseType::Cancel);
            dialog.add_button("Abrir", gtk::ResponseType::Accept);

            // Mostra o diálogo e captura a resposta
            if dialog.run() == gtk::ResponseType::Accept {
                // Se o usuário escolher um arquivo, exibe uma mensagem
                let file_name = dialog.get_filename().unwrap().to_str().unwrap().to_string();
                let message = format!("Arquivo '{}' recebido com sucesso!", file_name);
                
                // Exibe um pop-up de sucesso
                let msg_dialog = MessageDialog::new(
                    Some(&window),
                    gtk::DialogFlags::empty(),
                    gtk::MessageType::Info,
                    ButtonsType::Ok,
                    &message,
                );
                msg_dialog.run();
                msg_dialog.close();
            }

            dialog.close();
        }
    });

    // Layout da interface
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox.pack_start(&label, false, false, 0);
    vbox.pack_start(&increment_button, false, false, 0);
    vbox.pack_start(&file_button, false, false, 0);

    window.add(&vbox);

    // Mostrar tudo
    window.show_all();

    // Iniciar o loop GTK
    gtk::main();
}
