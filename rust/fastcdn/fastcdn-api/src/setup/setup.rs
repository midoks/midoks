use rust_embed::RustEmbed;

#[derive(RustEmbed, Debug)]
#[folder = "src/setup/db_files/"]
struct DbFiles;

pub fn install_db() {
    let install_json = DbFiles::get("install.json").unwrap();
    // println!("{:?}", install_json);
    let content = std::str::from_utf8(install_json.data.as_ref()).unwrap();
    println!("{:?}", content);
}
