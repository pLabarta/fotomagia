use std::process::{Command, Stdio};

use crate::config::Config;

use anyhow::Error;

#[derive(Debug, PartialEq)]
pub struct CameraConfig {
    capturemode: Config,
    burstnumber: Config,
    shutterspeed: Config,
    iso: Config,
    imagequality: Config,
}

impl CameraConfig {
    // This method will help users to discover the builder
    pub fn builder() -> CameraConfigBuilder {
        CameraConfigBuilder::default()
    }

    pub fn set_config(&self) -> Result<(),Error> {
        let mut gp = Command::new("gphoto2");

        gp.arg("--set-config");
        gp.arg(self.capturemode.to_command_string());
        gp.arg("--set-config");
        gp.arg(self.burstnumber.to_command_string());
        gp.arg("--set-config");
        gp.arg(self.shutterspeed.to_command_string());
        gp.arg("--set-config");
        gp.arg(self.iso.to_command_string());
        gp.arg("--set-config");
        gp.arg(self.imagequality.to_command_string());

        let result = gp
            // No output
            .stdout(Stdio::null()) 
            .status()?;

        // println!("status: {}", result);

        Ok(())
    }
}

pub struct CameraConfigBuilder {
    pub capturemode: Config,
    pub burstnumber: Config,
    pub shutterspeed: Config,
    pub iso: Config,
    pub imagequality: Config,
}

impl Default for CameraConfigBuilder {
    fn default() -> Self {
        Self {
            capturemode: Config {
                name: "Capture Mode".to_string(),
                key: "/main/capturesettings/capturemode".to_string(),
                value: 0,
            },
            burstnumber: Config {
                name: "Burst Number".to_string(),
                key: "/main/capturesettings/burstnumber".to_string(),
                value: 1,
            },
            shutterspeed: Config {
                name: "Shutter Speed".to_string(),
                key: "/main/capturesettings/shutterspeed".to_string(),
                value: 0,
            },
            iso: Config {
                name: "ISO".to_string(),
                key: "/main/imgsettings/iso".to_string(),
                value: 0,
            },
            imagequality: Config {
                name: "Image Quality".to_string(),
                key: "imagequality".to_string(),
                value: 0,
            },
        }
    }
}

impl CameraConfigBuilder {
    pub fn capturemode(mut self, value: u8) -> Self {
        self.capturemode.value = value;
        self
    }

    pub fn burstnumber(mut self, value: u8) -> Self {
        self.burstnumber.value = value;
        self
    }

    pub fn shutterspeed(mut self, value: u8) -> Self {
        self.shutterspeed.value = value;
        self
    }

    pub fn iso(mut self, value: u32) -> Self {

        fn get_closest_iso_index(iso: u32) -> usize {
            let iso_settings = vec![100, 125, 160, 200, 250, 320, 400, 500, 640, 800, 1000, 1250, 1600, 2000, 2500, 3200, 4000, 5000, 6400, 8000, 10000, 12800, 25600];
            let mut closest_index = 0;
            let mut closest_value = 0;
        
            for (index, value) in iso_settings.iter().enumerate() {
                if (iso as i32 - value.clone() as i32).abs() < (iso as i32 - closest_value as i32).abs() {
                    closest_index = index;
                    closest_value = value.clone();
                }
            }

            closest_index
        }
            
        let closest_index = get_closest_iso_index(value);

        self.iso.value = closest_index as u8;
        self
    }

    pub fn imagequality(mut self, value: u8) -> Self {
        self.imagequality.value = value;
        self
    }

    pub fn build(self) -> CameraConfig {
        CameraConfig {
            capturemode: self.capturemode,
            burstnumber: self.burstnumber,
            shutterspeed: self.shutterspeed,
            iso: self.iso,
            imagequality: self.imagequality,
        }
    }
}