/// Crates usadas no projeto
use dotenv::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
/// Struct final, trás cada um dos artistas que são semelhantes com a nossa pesquisa
/// Representa um artista que a Last.FM retornou sobre a pesquisa
struct RetornoArtista {
    /// Serde acha o valor "name" e retorna como nome_artista
    #[serde(rename = "name")]
    nome_artista: String,
    /// Mesma coisa que o comentario a cima porém de match -> match_artista
    #[serde(rename = "match")]
    match_artista: String,
    /// Mesma coisa que o comentario a cima porém de url -> url_artista
    #[serde(rename = "url")]
    url_artista: String,
}

#[derive(Deserialize)]
/// Trás o retorno de todos os artista que vieram dentro da lista do JSON response da API do Last.FM
struct RetornoTodosArtistas {
    artist: Vec<RetornoArtista>,
}

#[derive(Deserialize)]
/// Primeira tag que vem no JSON de retorno da Last.FM
struct RetornoArtistaSimilares {
    similarartists: RetornoTodosArtistas,
}

/// Chamada da crate Tokio para tornar o programa assincrono
#[tokio::main]

async fn main() {
    // Carregando o dotenv no inicio do códidgo
    dotenv().expect("Arquivo .env não encontrado!");

    // Montagem da URL de pesquisa
    let api_key = env::var("LASTFM_API_KEY").expect("Erro ao buscar a chave de API");
    let url_montada = format!(
        "http://ws.audioscrobbler.com/2.0/?method=artist.getSimilar&artist=Arctic+Monkes&api_key={}&format=json&limit=10&autocorret=1",
        api_key
    );

    // Criando o cliente HTTP com reqwest
    reqwest::Client::new();

    // Criando a variavel que chama a API
    let response = reqwest::get(url_montada) // Chamando a partir da crate reqwest a função get
        .await // Aguardando o retorno do metodo GET
        .expect("Problemas com a URL") // Trativa de erros de forma simples TODO Trocar na hora de subir versão release
        .text()
        .await
        .expect("Problemas com o JSON de retorno");

    println!("{}", response)
}
