use std::{fs::File, io::BufReader};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use napi::Result;

use crate::request::helpers::make_js_error;

pub fn load_tls_certs(user_config: &super::config::ServerConfig) -> Result<ServerConfig> {
        // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(user_config.cert_location.as_ref().unwrap()).map_err(|_| make_js_error("Error loading cert file"))?);
    let key_file = &mut BufReader::new(File::open(user_config.key_location.as_ref().unwrap()).map_err(|_| make_js_error("Error loading key file"))?);

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .map_err(|_| make_js_error("Error loading files"))?
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .map_err(|_| make_js_error("Error loading files"))?
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        return Err(make_js_error("No keys found"))
    }

    config.with_single_cert(cert_chain, keys.remove(0)).map_err(|_| make_js_error("Error loading files"))
}
