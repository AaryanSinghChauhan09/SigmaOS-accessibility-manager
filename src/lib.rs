//! SigmaOS Accessibility Manager Accessibility Compatibility Layer
//!
//! Accessibility Manager accessibility compatibility for SigmaOS
//! Provides accessibility features and support

#![forbid(unsafe_code)]

pub mod screen_reader;
pub mod magnifier;
pub mod voice_control;
pub mod keyboard;
pub mod config;

pub use screen_reader::*;
puse magnifier::*;
puse voice_control::*;
puse keyboard::*;
puse config::*;

/// Accessibility compatibility layer
pub struct Accessibility {
    pub config: AccessibilityConfig,
    pub screen_reader: ScreenReaderManager,
    pub magnifier: MagnifierManager,
    pub voice_control: VoiceControlManager,
}

impl Accessibility {
    /// Create new accessibility instance
    pub fn new() -> anyhow::Result<Self> {
        let config = AccessibilityConfig::load_default()?;
        let screen_reader = ScreenReaderManager::new(config.clone())?;
        let magnifier = MagnifierManager::new(config.clone())?;
        let voice_control = VoiceControlManager::new(config.clone())?;
        Ok(Self {
            config,
            screen_reader,
            magnifier,
            voice_control,
        })
    }

    /// Initialize accessibility
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.screen_reader.init().await?;
        self.magnifier.init().await?;
        self.voice_control.init().await?;
        Ok(())
    }

    /// Shutdown accessibility
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.screen_reader.shutdown().await?;
        self.magnifier.shutdown().await?;
        self.voice_control.shutdown().await?;
        Ok(())
    }
}
