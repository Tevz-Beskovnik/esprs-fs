use std::ffi::CString;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let base_path = CString::new("/spiffs").unwrap();
    let partition = CString::new("storage").unwrap();

    let conf = esp_idf_svc::sys::esp_vfs_spiffs_conf_t {
        base_path: base_path.as_ptr(),
        partition_label: partition.as_ptr(),
        max_files: 5,
        format_if_mount_failed: true,
    };

    unsafe {
        esp_idf_svc::sys::esp_nofail!(esp_idf_svc::sys::esp_vfs_spiffs_register(&conf));
    }

    match std::fs::read_to_string("/spiffs/test.html") {
        Ok(contents) => {
            log::info!("{}", contents);
        }
        Err(err) => {
            log::error!("{}", err.to_string());
        }
    };
}
