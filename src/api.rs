use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use bytes::Bytes;
use futures::{StreamExt, TryStreamExt};
use reqwest::Client;
use sanitize_filename::sanitize;
use sqlx::mysql::MySqlPool;
use tokio::fs;
use tokio_util::io::ReaderStream;
use tokio::io::AsyncWriteExt; // Necessário para escrita assíncrona

pub async fn upload_file(mut payload: Multipart, pool: web::Data<MySqlPool>) -> impl Responder {
    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            let content_disposition = field.content_disposition().clone();
            let filename = content_disposition
                .get_filename()
                .map(|name| name.to_string())
                .unwrap_or_else(|| "arquivo_desconhecido".to_string());
            let filepath = format!("./uploads/{}", sanitize(&filename));
            let mut f = match fs::File::create(filepath.clone()).await { // Correção: Usando tokio::fs::File::create() para evitar bloqueio
                Ok(file) => file,
                Err(_) => return HttpResponse::InternalServerError().body("Erro ao criar o arquivo."),
            };

            while let Some(chunk) = field.next().await {
                if let Ok(data) = chunk {
                    if f.write_all(&data).await.is_err() { // Correção: Usando write_all().await para evitar bloqueio
                        return HttpResponse::InternalServerError().body("Erro ao escrever no arquivo.");
                    }
                }
            }

            if sqlx::query("INSERT INTO files (name) VALUES (?)") // Correção: Usando bind() corretamente para evitar erro de sintaxe SQL
                .bind(filename) // Usar bind para passar o valor corretamente
                .execute(pool.get_ref()) 
                .await
                .is_err()
            {
                return HttpResponse::InternalServerError().body("Erro ao salvar no banco de dados.");
            }
        }
    }

    HttpResponse::Ok().body("Arquivo recebido com sucesso!")
}

pub async fn send_file_to_server(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let file = fs::File::open(file_name).await?; // Correção: fs::File::open já retorna um arquivo assíncrono adequado
    let stream = ReaderStream::new(file);
    let byte_vec = stream
        .map_ok(|chunk: Bytes| chunk.to_vec()) // Converte cada `Bytes` para `Vec<u8>`
        .try_concat() // Junta todos os `Vec<u8>` em um único vetor
        .await?;
    let body = reqwest::Body::from(byte_vec);
    let part = reqwest::multipart::Part::stream(body);
    let form = reqwest::multipart::Form::new().part("file", part);
    let res = client.post("http://127.0.0.1:8080/upload")
        .multipart(form)
        .send()
        .await?;
    let status = res.status(); // Correção: Melhor depuração, mostrando o status e corpo da resposta
    let response_body = res.text().await?;

    if status.is_success() {
        println!("Arquivo enviado com sucesso!");
    } else {
        println!("Falha ao enviar o arquivo: {} - {}", status, response_body);
    }

    Ok(())
}
