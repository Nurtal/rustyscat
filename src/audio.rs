use std::{fs, io::Cursor, path::{Path, PathBuf}};
use url::Url;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

pub mod audio {
    pub use super::ensure_data_dir;
    pub use super::download_audio;
}



/// Crée le dossier `data/` à la racine du projet s'il n'existe pas déjà.
///
/// # Exemples
/// 
/// ```
/// let data_path = audio_downloader::ensure_data_dir().unwrap();
/// assert!(data_path.exists());
/// ```
///
/// # Erreurs
/// 
/// Retourne une `std::io::Error` si :
/// - Impossible d'accéder au répertoire courant
/// - Problème de permissions pour créer le dossier
///
/// # Panics
/// 
/// Aucun.
pub fn ensure_data_dir() -> Result<PathBuf, std::io::Error> {
    let mut path = std::env::current_dir()?;  // Répertoire courant (racine du projet)
    path.push("data");
    
    fs::create_dir_all(&path)?;  // Crée data/ si absent, ignore si existe déjà
    Ok(path)
}

/// Télécharge un fichier audio depuis une URL et le sauvegarde dans `data/audio/`.
///
/// # Arguments
/// * `url` - URL complète du fichier audio (ex: "https://ex.com/son.mp3")
///
/// # Exemples
/// ```
/// let url = "https://exemple.com/son.mp3";
/// audio_downloader::download_audio(url).unwrap();
/// ```
///
/// # Erreurs
/// - URL invalide
/// - Échec du téléchargement (404, réseau...)
/// - Problème d'écriture (permissions, disque plein)
pub async fn download_audio(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    
    // Créer data/audio/
    let mut base_dir = ensure_data_dir()?;
    base_dir.push("audio");
    fs::create_dir_all(&base_dir)?;

    // Extraire nom de fichier depuis URL
    let parsed_url = Url::parse(url)?;
    let file_name = parsed_url
        .path_segments()
        .and_then(|segments| segments.last())
        .ok_or("URL sans nom de fichier")?
        .to_string();

    let mut file_path = base_dir;
    file_path.push(file_name);

    // Télécharger et sauvegarder
    let response = reqwest::get(url).await?;
    response.error_for_status_ref()?;  // Vérifie 2xx

    let mut file = fs::File::create(&file_path)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    println!("✅ Téléchargé : {:?}", file_path);
    Ok(file_path)
}


/// Charge un fichier MP3 et affiche ses infos de base
///
/// Retourne un Sink pour jouer le son (optionnel)
pub fn load_mp3(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ouvre le fichier
    let file = File::open(path)?;
    let source = Decoder::new(BufReader::new(file))?;

    // Infos du fichier
    println!("Sample rate : {}", source.sample_rate());
    println!("Canaux      : {}", source.channels());

    // Joue le son (optionnel)
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(source);
    
    println!("✅ MP3 chargé et prêt à jouer ! Appuie Ctrl+C pour arrêter.");
    sink.sleep_until_end();

    Ok(())
}


