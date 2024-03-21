use std::fs;

fn token() -> Result<String, std::io::Error> {
    let file = fs::read_to_string("src/athy.config")?;
    Ok(file.trim().to_string())
}

fn main() {
    let token:String = token().expect("Erro no arquivo de configuração");

}
