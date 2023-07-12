use std::process::{Command, Stdio};

use anyhow::Result;
use camera_config::CameraConfig;

mod camera_config;
mod config;


fn main() -> Result<()>{

    kill_gphoto2()?;

    let config = CameraConfig::builder()
    .capturemode(0)
    .burstnumber(1)
    .shutterspeed(10)
    .iso(20000)
    .imagequality(2)
    .build();

    config.set_config()?;

    // get_light_meter_status()?;

    capture_and_rename()?;

    Ok(())
}

fn trigger_capture() -> Result<()> {
    let mut gp = Command::new("gphoto2");

    gp.arg("--capture-image-and-download");
    gp.arg("--force-overwrite");

    let result = gp
        // No output
        .stdout(Stdio::null())
        .status()?;

    // println!("status: {}", result);
    // /main/imgsettings/
    Ok(())
}

fn capture_and_rename() -> Result<()> {
    let mut gp = Command::new("gphoto2");

    gp.arg("--capture-image-and-download");
    gp.arg("--force-overwrite");

    let result = gp
        // No output
        .stdout(Stdio::null())
        .status()?;

    // Get the file named capt0000.jpg and rename it to 'name'
    let mut mv = Command::new("mv");

    mv.arg("capt0000.jpg");
    mv.arg("name.jpg");

    let result = mv
        // No output
        .stdout(Stdio::null())
        .status()?;

    
    // println!("status: {}", result);

    Ok(())
}

fn get_light_meter_status() -> Result<()> {
    let mut gp = Command::new("gphoto2");

    gp.arg("--get-config");
    gp.arg("/main/status/lightmeter");

    let result = gp.status()?;

    println!("status: {}", result);

    Ok(())
}

fn kill_gphoto2() -> Result<()> {

    let ps: std::process::Child = Command::new("ps")
        .arg("aux")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep: std::process::Child = Command::new("grep")
        .arg("[g]photo")
        .stdin(ps.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let awk: std::process::Child = Command::new("awk")
        .arg("{print $2}")
        .stdin(grep.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = awk.wait_with_output().unwrap();
    print!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

