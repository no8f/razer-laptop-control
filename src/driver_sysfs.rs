use lazy_static::lazy_static;

use std::fs;

// Driver path
pub const DRIVER_DIR: &'static str =
    "/sys/module/razercontrol/drivers/hid:razerctrl";

lazy_static! {
    static ref SYSFS_PATH: Option<String> = {
        match fs::read_dir(DRIVER_DIR)
            .unwrap()
            .find(|x| {
                x.as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .starts_with("000")
            })
            .unwrap()
        {
            Ok(p) => Some(String::from(p.path().to_str().unwrap())),
            Err(_) => None,
        }
    };
}

pub fn get_path() -> Option<String> {
    SYSFS_PATH.clone()
}

/// Writes a String to a sysfs file
fn write_to_sysfs(sysfs_name: &str, val_as_str: String) -> bool {
    match fs::write(SYSFS_PATH.clone().unwrap() + "/" + sysfs_name, val_as_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Writes a byte array to a sysfs file
fn write_to_sysfs_raw(sysfs_name: &str, val: Vec<u8>) -> bool {
    match fs::write(SYSFS_PATH.clone().unwrap() + "/" + sysfs_name, val) {
        Ok(_) => true,
        Err(x) => {
            eprintln!("SYSFS write to {} failed! - {}", sysfs_name, x);
            false
        }
    }
}

/// Reads a String from sysfs file (Removing the \n)
fn read_from_sysfs(sysfs_name: &str) -> Option<String> {
    match fs::read_to_string(SYSFS_PATH.clone().unwrap() + "/" + sysfs_name) {
        Ok(s) => Some(s.clone().trim_end_matches('\n').to_string()),
        Err(_) => None,
    }
}

// RGB Map is write only
pub fn write_rgb_map(map: Vec<u8>) -> bool {
    return write_to_sysfs_raw("key_colour_map", map);
}

pub fn write_custom_mode_frame(custom_mode_frame: u8) -> bool {
    return write_to_sysfs("custom_frame_mode", String::from(format!("{}", custom_mode_frame)));
}

// Brightness is read + write
pub fn write_brightness(lvl: u8) -> bool {
    return write_to_sysfs("brightness", String::from(format!("{}", lvl)));
}

pub fn read_brightness() -> u8 {
    return match read_from_sysfs("brightness") {
        Some(x) => x.parse::<u8>().unwrap(),
        None => 0,
    };
}

// Power mode is read + write
pub fn write_power(mode: u8) -> bool {
    return write_to_sysfs("power_mode", String::from(format!("{}", mode)));
}

// cpu_boost read + write
pub fn write_cpu_boost(cpu_boost: u8) -> bool {
    return write_to_sysfs("cpu_boost", String::from(format!("{}", cpu_boost)));
}

//gpu_boost is read + write
pub fn write_gpu_boost(gpu_boost: u8) -> bool {
    return write_to_sysfs("gpu_boost", String::from(format!("{}", gpu_boost)));
}

pub fn write_logo_state(logo_state: u8) -> bool {
    return write_to_sysfs("logo_led_state", String::from(format!("{}", logo_state)));
}

pub fn read_logo_state() -> u8 {
    return match read_from_sysfs("logo_led_state") {
        Some(x) => x.parse::<u8>().unwrap(),
        None => 0,
    };
}

pub fn read_power() -> u8 {
    return match read_from_sysfs("power_mode") {
        Some(x) => x.parse::<u8>().unwrap(),
        None => 0,
    };
}

pub fn read_cpu_boost() -> u8 {
    return match read_from_sysfs("cpu_boost") {
        Some(x) => x.parse::<u8>().unwrap(),
        None => 0,
    };
}

pub fn read_gpu_boost() -> u8 {
    return match read_from_sysfs("gpu_boost") {
        Some(x) => x.parse::<u8>().unwrap(),
        None => 0,
    };
}
/// Writes fan RPM to sysfs, and returns result of the write
/// # Arguments
/// * `rpm` - Fan RPM to write to sysfs. 0 imples back to automatic fan
///             anything else is interpreted as a litteral RPM
///
/// # Example
/// ```
/// write_fan_rpm(0).unwrap(); // Fan RPM Set to Auto
/// match write_fan_rpm(5000) { // Ask fan to spin to 5000 RPM
///     true => println!("Write OK!"),
///     false => println!("Write FAIL!")
/// }
/// ```
pub fn write_fan_rpm(rpm: i32) -> bool {
    return write_to_sysfs("fan_rpm", String::from(format!("{}", rpm)));
}

pub fn read_fan_rpm() -> i32 {
    return match read_from_sysfs("fan_rpm") {
        Some(x) => x.parse::<i32>().unwrap(),
        None => 0,
    };
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PowerSupply {
    /// Machine is plugged int (AC)
    AC,
    /// Machine is running on battery
    BAT,
    UNK
}

/// Returns the current power supply of the laptop
pub fn read_power_source() -> PowerSupply {
    match fs::read_to_string("/sys/class/power_supply/AC0/online") {
        Ok(s) => match s.as_str().trim_end_matches('\n') {
            "1" => PowerSupply::AC,
            "0" => PowerSupply::BAT,
            _ => PowerSupply::UNK,
        }
        Err(_) => PowerSupply::UNK,
    }
}
