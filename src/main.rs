use hound::WavReader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    
    // Chemin vers ton fichier wav
    let path = "/home/n765/workspace/miami_wire/1_Whisper_on_Biscayne/data/miami_scenario1_whisper.wav";

    // Ouvre le fichier et lit l’en-tête
    let mut reader = WavReader::open(path)?;
    println!("Sample rate : {}", reader.spec().sample_rate);
    println!("Canaux      : {}", reader.spec().channels);
    println!("Bits/sample : {}", reader.spec().bits_per_sample);

    // Charge tous les échantillons dans un vecteur de i16 (pour un WAV PCM 16 bits)
    let samples: Result<Vec<i16>, _> = reader.samples::<i16>().collect();
    let samples = samples?;

    println!("Nombre d'échantillons : {}", samples.len());

    // Exemple : utilise le premier échantillon
    if let Some(first) = samples.first() {
        println!("Premier échantillon : {}", first);
    }

    Ok(())
}
