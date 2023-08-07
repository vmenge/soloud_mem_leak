use soloud::{AudioExt, LoadExt, Soloud, SoloudFlag, Wav};
use std::{
    fs::File,
    io::{BufReader, Read},
    time::Duration,
};

#[tokio::main]
async fn main() {
    loop {
        let soloud =
            Soloud::new(SoloudFlag::empty(), soloud::Backend::Alsa, 8_000, 512, 1).unwrap();

        let mut buf = BufReader::new(File::open("arrow.wav").unwrap());
        let mut bytes = Vec::new();
        buf.read_to_end(&mut bytes).unwrap();

        let mut wav = Wav::default();
        wav.load_mem(&bytes).unwrap();

        let duration = wav.length() * 1_000.0;
        let duration = Duration::from_millis(duration as u64);

        soloud.play(&wav);
        tokio::time::sleep(duration).await;
    }
}
