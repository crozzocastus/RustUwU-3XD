use mysql::*;
use mysql::prelude::*;
use std::result::Result;

pub fn connect_to_db() -> Result<PooledConn, mysql::Error> {
    let url = "mysql://root:05062002@localhost:3306/rust_app"; 
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}

pub fn fetch_data() -> Result<Vec<(i32, String)>, mysql::Error> {
    let mut conn = connect_to_db()?;

    // Exemplo de consulta SQL na tabela "files"
    let result: Vec<(i32, String)> = conn.query("SELECT id, name FROM files")?;

    if result.is_empty() {
        eprintln!("Nenhum dado encontrado na tabela 'files'.");
    } else {
        for row in &result {
            println!("ID: {}, Filename: {}", row.0, row.1);
        }
    }

    Ok(result)
}

pub fn handle_error() {
    if let Err(e) = fetch_data() {
        eprintln!("Erro ao buscar dados: {}", e);
    }
}

pub fn handle_connection() {
    match connect_to_db() {
        Ok(_conn) => println!("Conexão com o banco de dados estabelecida com sucesso!"),
        Err(e) => eprintln!("Erro ao conectar ao banco de dados: {}", e),
    }
}