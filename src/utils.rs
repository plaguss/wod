use chrono::Local;

pub fn today() -> String {
    Local::now().format("%d-%m-%Y").to_string()
}

pub fn default_folder() -> String {
    format!("wod-{}", &today())
}

pub fn default_filename() -> String {
    format!("wod-{}.txt", today())
}
