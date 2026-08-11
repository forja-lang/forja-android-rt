// Forja Android RT — Servidor Nativo de Hot-Reload sobre TCP/ADB/Wi-Fi
//
// Permite que la aplicación Android en ejecución reciba parches de código .fa
// en tiempo real sobre Wi-Fi o ADB port forwarding (puerto 7355) sin reiniciar
// la actividad ni perder el estado.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub const DEFAULT_HOT_RELOAD_PORT: u16 = 7355;
const MAGIC_HEADER: &[u8; 4] = b"FHR1";

/// Servidor de Hot-Reload nativo para Android
pub struct HotReloadServer {
    listening: Arc<AtomicBool>,
    latest_code: Arc<Mutex<Option<String>>>,
}

impl HotReloadServer {
    /// Inicia el servidor TCP de Hot-Reload en segundo plano
    pub fn start(port: Option<u16>) -> Self {
        let listen_port = port.unwrap_or(DEFAULT_HOT_RELOAD_PORT);
        let listening = Arc::new(AtomicBool::new(true));
        let latest_code = Arc::new(Mutex::new(None));

        let listening_clone = Arc::clone(&listening);
        let latest_code_clone = Arc::clone(&latest_code);

        thread::spawn(move || {
            let addr = format!("0.0.0.0:{}", listen_port);
            match TcpListener::bind(&addr) {
                Ok(listener) => {
                    log::info!(
                        "[Hot-Reload] Servidor activo escuchando en 0.0.0.0:{}",
                        listen_port
                    );
                    listener.set_nonblocking(true).ok();

                    while listening_clone.load(Ordering::Relaxed) {
                        match listener.accept() {
                            Ok((mut stream, peer_addr)) => {
                                log::info!("[Hot-Reload] Conexión entrante desde: {}", peer_addr);
                                Self::handle_client(&mut stream, &latest_code_clone);
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                thread::sleep(std::time::Duration::from_millis(100));
                            }
                            Err(e) => {
                                log::warn!("[Hot-Reload] Error al aceptar conexión: {}", e);
                                thread::sleep(std::time::Duration::from_millis(200));
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "[Hot-Reload] No se pudo vincular servidor en puerto {}: {}",
                        listen_port,
                        e
                    );
                }
            }
        });

        Self {
            listening,
            latest_code,
        }
    }

    /// Maneja la recepción de parches de código desde el CLI (`forja transmitir`)
    fn handle_client(stream: &mut TcpStream, latest_code: &Arc<Mutex<Option<String>>>) {
        // Cabecera: 4 bytes Magic ("FHR1") + 4 bytes Big-Endian u32 (longitud del payload)
        let mut header = [0u8; 8];
        if stream.read_exact(&mut header).is_err() {
            log::warn!("[Hot-Reload] Cabecera inválida recibida");
            return;
        }

        if &header[0..4] != MAGIC_HEADER {
            log::warn!("[Hot-Reload] Cabecera no coincide (esperado FHR1)");
            return;
        }

        let payload_len = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as usize;
        if payload_len == 0 || payload_len > 10_000_000 {
            log::warn!(
                "[Hot-Reload] Tamaño de código inválido: {} bytes",
                payload_len
            );
            return;
        }

        let mut buffer = vec![0u8; payload_len];
        if stream.read_exact(&mut buffer).is_err() {
            log::error!("[Hot-Reload] Error al leer el payload del parche");
            return;
        }

        match String::from_utf8(buffer) {
            Ok(code) => {
                log::info!(
                    "🚀 [Hot-Reload] Parche de código recibido exitosamente ({} bytes)",
                    payload_len
                );
                if let Ok(mut guard) = latest_code.lock() {
                    *guard = Some(code);
                }
                let _ = stream.write_all(b"OK");
            }
            Err(e) => {
                log::error!("[Hot-Reload] El código recibido no es UTF-8 válido: {}", e);
                let _ = stream.write_all(b"ERR_UTF8");
            }
        }
    }

    /// Consulta si hay un nuevo parche de código disponible enviado desde el cliente
    pub fn poll_latest_code(&self) -> Option<String> {
        if let Ok(mut guard) = self.latest_code.lock() {
            guard.take()
        } else {
            None
        }
    }

    /// Detiene el servidor de Hot-Reload
    pub fn stop(&self) {
        self.listening.store(false, Ordering::Relaxed);
    }
}
