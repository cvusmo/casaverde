// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/sensors.rs

use rustls::DigitallySignedStruct;
use std::sync::Arc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fs, time::{Duration, Instant}};
use std::process::Command;
use uuid::Uuid;
use rustls::RootCertStore;
use rustls_pemfile;
use rustls_pki_types::{UnixTime, CertificateDer, ServerName};

#[derive(Copy, Clone, PartialEq)]
pub enum Sensor {
    Solar,
    Temperature,
    Moisture,
    Humidity,
    Water,
}

impl Sensor {
    pub const ALL: [Sensor; 5] = [
        Sensor::Solar,
        Sensor::Temperature,
        Sensor::Moisture,
        Sensor::Humidity,
        Sensor::Water,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Sensor::Solar => "Solar Sensor",
            Sensor::Temperature => "Temperature Sensor",
            Sensor::Moisture => "Moisture Sensor",
            Sensor::Humidity => "Humidity Sensor",
            Sensor::Water => "Water Sensor",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TempData {
    pub cpu: Option<f32>,
    pub gpu: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct SensorReading {
    pub client_id: String,
    pub temp_data: TempData,
}

#[derive(Clone)]
pub struct SensorData {
    pub states: [bool; Sensor::ALL.len()],
    pub temp_data: TempData,
    last_updated: Instant,
    client: Client,
    server: String,
    client_id: String,
}

impl SensorData {
    pub fn new(server: &str) -> Self {
        let cert_path = "server.crt";
        let cert_data = fs::read(cert_path)
            .map_err(|e| {
                eprintln!("Failed to read server.crt: {e}. Place it in the project directory.");
                e
            })
            .expect("Failed to read server.crt");

        let certs = rustls_pemfile::certs(&mut &*cert_data)
            .map(|result| result.expect("Invalid certificate"))
            .collect::<Vec<CertificateDer<'static>>>();

        let mut root_store = RootCertStore::empty();
        for cert in certs {
            root_store.add(cert).expect("Failed to add certificate to root store");
        }

        let mut tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
            //.build() // <-- line 89 and 90 commented out fixes error
            //.expect("Failed to build TLS configuration");
        tls_config.dangerous().set_certificate_verifier(Arc::new(NoVerifier));

        let client = Client::builder()
            .use_preconfigured_tls(Arc::new(tls_config) as Arc<dyn std::any::Any>)
            .build()
            .expect("Failed to build reqwest client with TLS config");
        
        let mut states = [false; Sensor::ALL.len()];
        states[Sensor::Temperature as usize] = true;

        Self {
            states,
            temp_data: TempData { cpu: None, gpu: None },
            last_updated: Instant::now(),
            client,
            server: server.to_string(),
            client_id: Uuid::new_v4().to_string(),
        }
    }

    pub async fn update_temperatures(&mut self) {
        if !self.states[Sensor::Temperature as usize] {
            return;
        }

        self.temp_data = TempData {
            cpu: get_cpu_temp(),
            gpu: get_gpu_temp(),
        };

        if self.last_updated.elapsed() >= Duration::from_secs(5) {
            let reading = SensorReading {
                client_id: self.client_id.clone(),
                temp_data: self.temp_data.clone(),
            };
            let url = format!("{}/sensor_data", self.server);
            eprintln!("Sending JSON: {:?}", serde_json::to_string(&reading).unwrap_or_default());
            match self.client.post(&url).json(&reading).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!("Successfully sent temperature data to server");
                        self.last_updated = Instant::now();
                    } else {
                        eprintln!("Failed to send data to {}: status {}", url, resp.status());
                        if let Ok(text) = resp.text().await {
                            eprintln!("Server response: {text}");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to send data to {url}: {e}");
                    if e.is_connect() {
                        eprintln!("Connection error: check server availability or network");
                    }
                    if e.is_timeout() {
                        eprintln!("Request timed out");
                    }
                }
            }
        }
    }

    pub async fn get_all_temperatures(&self) -> Option<Vec<(String, TempData)>> {
        let url = format!("{}/sensor_data", self.server);
        match self.client.get(&url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<Vec<(String, TempData)>>().await {
                        Ok(data) => Some(data),
                        Err(e) => {
                            eprintln!("Failed to parse JSON from {url}: {e}");
                            None
                        }
                    }
                } else {
                    eprintln!("Failed to fetch data from {url}: status {}", resp.status());
                    if let Ok(text) = resp.text().await {
                        eprintln!("Server response: {text}");
                    }
                    None
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch data from {url}: {e}");
                None
            }
        }
    }

    pub fn toggle_sensor(&mut self, index: usize) {
        self.states[index] = !self.states[index];
    }
}

// Custom verifier to bypass strict hostname/IP validation
#[derive(Debug)]
struct NoVerifier;

// FIX: ERROR 0046 not all trait items implemented, missing verify_tls12_signature,
// verify_tls13_signature, supported_verify_schemes
impl rustls::client::danger::ServerCertVerifier for NoVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        //_scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
    &self,
    _message: &[u8],
    _cert: &CertificateDer<'_>,
    _dss: &DigitallySignedStruct,
) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
    Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
}

fn verify_tls13_signature(
    &self,
    _message: &[u8],
    _cert: &CertificateDer<'_>,
    _dss: &DigitallySignedStruct,
) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
    Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
}

fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
    vec![
        rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
        rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
        rustls::SignatureScheme::RSA_PSS_SHA256,
        rustls::SignatureScheme::RSA_PSS_SHA384,
        rustls::SignatureScheme::RSA_PKCS1_SHA256,
        rustls::SignatureScheme::RSA_PKCS1_SHA384,
    ]
}
}

fn get_cpu_temp() -> Option<f32> {
    match Command::new("sensors").output() {
        Ok(output) => {
            let sensors_str = String::from_utf8_lossy(&output.stdout);
            for line in sensors_str.lines() {
                if line.contains("Package id 0") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for part in parts {
                        if part.ends_with("°C") {
                            if let Ok(temp) = part.trim_end_matches("°C").trim_start_matches('+').parse::<f32>() {
                                return Some(temp);
                            }
                        }
                    }
                }
            }
            None
        }
        Err(e) => {
            eprintln!("Failed to run sensors: {e}");
            None
        }
    }
}

fn get_gpu_temp() -> Option<f32> {
    match Command::new("nvidia-smi")
        .arg("--query-gpu=temperature.gpu")
        .arg("--format=csv,noheader")
        .output()
    {
        Ok(output) => {
            let nvidia_str = String::from_utf8_lossy(&output.stdout);
            nvidia_str.trim().parse().ok()
        }
        Err(e) => {
            eprintln!("Failed to run nvidia-smi: {e}");
            None
        }
    }
}
