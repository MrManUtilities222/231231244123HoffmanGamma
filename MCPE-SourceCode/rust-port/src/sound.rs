/// Sound engine stub ported from client/sound/
/// SoundEngine.h, Sound.h, SoundRepository.h, platform/audio/*

use std::collections::HashMap;
#[cfg(feature = "audio")]
use std::path::Path;
#[cfg(feature = "audio")]
use std::fs::File;
#[cfg(feature = "audio")]
use std::io::BufReader;
#[cfg(feature = "audio")]
use std::sync::Mutex;

#[cfg(feature = "audio")]
mod backend_impl {
    use super::*;
    use rodio::{OutputStream, OutputStreamHandle, Sink, Decoder};

    pub struct SoundBackend {
        pub _stream: OutputStream,
        pub handle: OutputStreamHandle,
        pub sinks: Mutex<Vec<Sink>>,
    }

    impl SoundBackend {
        pub fn try_new() -> Option<Self> {
            match OutputStream::try_default() {
                Ok((stream, handle)) => Some(Self { _stream: stream, handle, sinks: Mutex::new(Vec::new()) }),
                Err(_) => None,
            }
        }

        pub fn play_file(&self, path: &Path, volume: f32) {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                if let Ok(source) = Decoder::new(reader) {
                    if let Ok(sink) = Sink::try_new(&self.handle) {
                        sink.set_volume(volume);
                        sink.append(source);
                        sink.play();
                        let mut s = self.sinks.lock().unwrap();
                        s.push(sink);
                        s.retain(|snk| !snk.empty());
                    }
                }
            }
        }
    }
}

#[cfg(feature = "audio")]
use backend_impl::SoundBackend;

#[derive(Clone, Debug)]
pub struct SoundDesc {
    pub name: String,
    pub path: String,
}

pub struct SoundRepository {
    sounds: HashMap<String, Vec<SoundDesc>>,
}

impl SoundRepository {
    pub fn new() -> Self {
        Self { sounds: HashMap::new() }
    }

    pub fn add(&mut self, category: &str, name: &str, path: &str) {
        self.sounds
            .entry(category.to_string())
            .or_default()
            .push(SoundDesc { name: name.to_string(), path: path.to_string() });
    }

    pub fn get_random(&self, category: &str) -> Option<&SoundDesc> {
        self.sounds.get(category).and_then(|v| {
            if v.is_empty() { None }
            else { Some(&v[rand::random::<usize>() % v.len()]) }
        })
    }
}

pub struct SoundEngine {
    pub enabled: bool,
    pub master_volume: f32,
    pub sound_volume: f32,
    pub music_volume: f32,
    pub listener_x: f32,
    pub listener_y: f32,
    pub listener_z: f32,
    pub listener_yaw: f32,
    pub max_distance: f32,
    repository: SoundRepository,
    #[cfg(feature = "audio")]
    backend: Option<SoundBackend>,
}

impl SoundEngine {
    pub fn new(max_distance: f32) -> Self {
        let mut repo = SoundRepository::new();
        // Register default sound categories matching MCPE's SoundRepository
        for &cat in &[
            "step.grass", "step.gravel", "step.stone", "step.wood", "step.cloth",
            "mob.zombie.say", "mob.zombie.hurt", "mob.zombie.death",
            "mob.skeleton.say", "mob.skeleton.hurt", "mob.skeleton.death",
            "mob.spider.say", "mob.spider.hurt", "mob.spider.death",
            "mob.creeper.say", "mob.creeper.hurt", "mob.creeper.death",
            "mob.pig.say", "mob.pig.hurt", "mob.pig.death",
            "mob.sheep.say", "mob.sheep.hurt", "mob.sheep.death",
            "mob.cow.say", "mob.cow.hurt", "mob.cow.death",
            "mob.chicken.say", "mob.chicken.hurt", "mob.chicken.death",
            "mob.zombiepig.zpig", "mob.zombiepig.zpighurt", "mob.zombiepig.zpigdeath",
            "random.click", "random.pop", "random.explode", "random.splash",
            "random.bow", "random.bowhit",
            "dig.grass", "dig.gravel", "dig.stone", "dig.wood", "dig.cloth",
        ] {
            repo.add(cat, cat, &format!("sounds/{}.ogg", cat.replace('.', "/")));
        }

        Self {
            enabled: true,
            master_volume: 1.0,
            sound_volume: 1.0,
            music_volume: 1.0,
            listener_x: 0.0,
            listener_y: 0.0,
            listener_z: 0.0,
            listener_yaw: 0.0,
            max_distance,
            repository: repo,
            #[cfg(feature = "audio")]
            backend: SoundBackend::try_new(),
        }
    }

    pub fn update_listener(&mut self, x: f32, y: f32, z: f32, yaw: f32) {
        self.listener_x = x;
        self.listener_y = y;
        self.listener_z = z;
        self.listener_yaw = yaw;
    }

    pub fn play(&self, name: &str, x: f32, y: f32, z: f32, volume: f32, _pitch: f32) {
        if !self.enabled { return; }
        let dx = x - self.listener_x;
        let dy = y - self.listener_y;
        let dz = z - self.listener_z;
        let dist = (dx*dx + dy*dy + dz*dz).sqrt();
        if dist > self.max_distance { return; }

        let vol_mult = 1.0 - (dist / self.max_distance);
        let _final_vol = volume * vol_mult * self.sound_volume * self.master_volume;
        // Try to play a matching sound file using backend (if enabled).
        #[cfg(feature = "audio")]
        {
            if let Some(backend) = &self.backend {
                // category paths in repository are like "step.grass" -> "sounds/step/grass.ogg"
                let rel = name.replace('.', "/");
                // Try common base paths
                let candidates = [
                    format!("sounds/{}.ogg", rel),
                    format!("sounds/{}.m4a", rel),
                    format!("data/sounds/{}.ogg", rel),
                    format!("data/sounds/{}.m4a", rel),
                    format!("data/sound/{}.ogg", rel),
                    format!("data/sound/{}.m4a", rel),
                    format!("data/sound/aac/{}.m4a", rel),
                    format!("{}/sounds/{}.ogg", std::env::current_dir().unwrap_or_else(|_|".".into()).display(), rel),
                    format!("{}/sounds/{}.m4a", std::env::current_dir().unwrap_or_else(|_|".".into()).display(), rel),
                ];
                for c in &candidates {
                    let p = Path::new(c);
                    if p.exists() {
                        backend.play_file(p, _final_vol as f32);
                        return;
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        if false { eprintln!("SND: {} at ({},{},{}) vol={:.2}", name, x, y, z, _final_vol); }
    }

    pub fn play_ui(&self, name: &str, volume: f32, _pitch: f32) {
        if !self.enabled { return; }
        let _final_vol = volume * self.sound_volume * self.master_volume;
        #[cfg(feature = "audio")]
        if let Some(backend) = &self.backend {
            let rel = name.replace('.', "/");
            let candidates = [
                format!("sounds/{}.ogg", rel),
                format!("sounds/{}.m4a", rel),
                format!("data/sounds/{}.ogg", rel),
                format!("data/sounds/{}.m4a", rel),
                format!("data/sound/{}.ogg", rel),
                format!("data/sound/{}.m4a", rel),
                format!("data/sound/aac/{}.m4a", rel),
            ];
            for c in &candidates {
                let p = Path::new(c);
                if p.exists() {
                    backend.play_file(p, _final_vol as f32);
                    return;
                }
            }
        }
    }

    pub fn enable(&mut self, status: bool) {
        self.enabled = status;
    }
}
