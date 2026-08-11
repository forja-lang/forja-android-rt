// Forja Android RT — NativeRegistry específico para Android
//
// Registra e implementa las funciones nativas para stdlib/android.fa
// (Toast, Notificaciones, Almacenamiento KeyStore, Ubicación, Cámara, Biometría, etc.)

use forja::native_registry::NativeRegistry;
use forja::vm_fast::{ErrFast, ForjaFast, ValorFast};
use std::sync::Arc;

/// Registra las funciones nativas Android en un NativeRegistry.
pub fn registrar_nativas_android(registry: &mut NativeRegistry) {
    // 1. UI & Háptica
    registry.registrar("_android_toast", android_toast);
    registry.registrar("_android_vibrar", android_vibrar);
    registry.registrar("_android_efecto_haptico", android_efecto_haptico);
    registry.registrar("_android_pantalla_encendida", android_pantalla_encendida);
    registry.registrar("_android_pantalla_orientacion", android_pantalla_orientacion);

    // 2. Notificaciones
    registry.registrar("_android_notificacion_canal_crear", android_notificacion_canal_crear);
    registry.registrar("_android_notificacion", android_notificacion);
    registry.registrar("_android_notificacion_cancelar", android_notificacion_cancelar);

    // 3. Almacenamiento & KeyStore
    registry.registrar("_android_almacenamiento_guardar", android_almacenamiento_guardar);
    registry.registrar("_android_almacenamiento_leer", android_almacenamiento_leer);
    registry.registrar("_android_almacenamiento_eliminar", android_almacenamiento_eliminar);
    registry.registrar("_android_guardar_seguro", android_guardar_seguro);
    registry.registrar("_android_leer_seguro", android_leer_seguro);

    // 4. Ubicación GPS
    registry.registrar("_android_ubicacion_actual", android_ubicacion_actual);

    // 5. Batería y Sensores
    registry.registrar("_android_bateria_nivel", android_bateria_nivel);
    registry.registrar("_android_bateria_cargando", android_bateria_cargando);
    registry.registrar("_android_sensor_acelerometro", android_sensor_acelerometro);
    registry.registrar("_android_sensor_giroscopio", android_sensor_giroscopio);

    // 6. Cámara & QR
    registry.registrar("_android_camara_tomar_foto", android_camara_tomar_foto);
    registry.registrar("_android_camara_escanear_qr", android_camara_escanear_qr);

    // 7. Permisos & Biometría
    registry.registrar("_android_permiso_verificar", android_permiso_verificar);
    registry.registrar("_android_permiso_solicitar", android_permiso_solicitar);
    registry.registrar("_android_biometria_autenticar", android_biometria_autenticar);

    // 8. Intents & Sistema
    registry.registrar("_android_compartir_texto", android_compartir_texto);
    registry.registrar("_android_abrir_url", android_abrir_url);
    registry.registrar("_android_clipboard_copiar", android_clipboard_copiar);
    registry.registrar("_android_clipboard_pegar", android_clipboard_pegar);
}

// Helpers para alojar cadenas y arreglos en la VM
fn texto_val(vm: &mut ForjaFast, s: &str) -> ValorFast {
    let idx = vm.alloc_str(Arc::from(s));
    ValorFast::texto(idx)
}

fn extraer_texto(vm: &ForjaFast, val: ValorFast) -> String {
    if val.es_texto() {
        vm.get_str(val.indice_texto()).to_string()
    } else {
        String::new()
    }
}

// ─── Implementaciones Nativas de Funciones Android ───────────────

fn android_toast(vm: &mut ForjaFast, args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let msg = if let Some(&arg) = args.first() {
        extraer_texto(vm, arg)
    } else {
        String::new()
    };
    log::info!("[Android Toast] {}", msg);
    Ok(ValorFast::nulo())
}

fn android_vibrar(_vm: &mut ForjaFast, args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let ms = if let Some(&arg) = args.first() {
        if arg.es_entero() { arg.a_entero() } else { 100 }
    } else {
        100
    };
    log::info!("[Android Vibrar] {} ms", ms);
    Ok(ValorFast::nulo())
}

fn android_efecto_haptico(vm: &mut ForjaFast, args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let tipo = if let Some(&arg) = args.first() {
        extraer_texto(vm, arg)
    } else {
        "clic".to_string()
    };
    log::info!("[Android Efecto Háptico] {}", tipo);
    Ok(ValorFast::nulo())
}

fn android_pantalla_encendida(_vm: &mut ForjaFast, args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let _mantener = if let Some(&arg) = args.first() {
        arg.es_booleano() && arg.a_booleano()
    } else {
        true
    };
    Ok(ValorFast::nulo())
}

fn android_pantalla_orientacion(vm: &mut ForjaFast, args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let _modo = if let Some(&arg) = args.first() {
        extraer_texto(vm, arg)
    } else {
        "auto".to_string()
    };
    Ok(ValorFast::nulo())
}

fn android_notificacion_canal_crear(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::nulo())
}

fn android_notificacion(vm: &mut ForjaFast, args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let titulo = if args.len() > 2 {
        extraer_texto(vm, args[2])
    } else {
        "Notificación".to_string()
    };
    let msg = if args.len() > 3 {
        extraer_texto(vm, args[3])
    } else {
        String::new()
    };
    log::info!("[Android Notification] {}: {}", titulo, msg);
    Ok(ValorFast::nulo())
}

fn android_notificacion_cancelar(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::nulo())
}

fn android_almacenamiento_guardar(vm: &mut ForjaFast, args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let clave = if !args.is_empty() { extraer_texto(vm, args[0]) } else { String::new() };
    let valor = if args.len() > 1 { extraer_texto(vm, args[1]) } else { String::new() };
    log::info!("[Android SharedPreferences] Guardado {}={}", clave, valor);
    Ok(ValorFast::nulo())
}

fn android_almacenamiento_leer(vm: &mut ForjaFast, args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let defecto = if args.len() > 1 { extraer_texto(vm, args[1]) } else { String::new() };
    Ok(texto_val(vm, &defecto))
}

fn android_almacenamiento_eliminar(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::nulo())
}

fn android_guardar_seguro(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::nulo())
}

fn android_leer_seguro(vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(texto_val(vm, ""))
}

fn android_ubicacion_actual(vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let arr_idx = vm.alloc_arr(vec![
        ValorFast::flotante(0.0),
        ValorFast::flotante(0.0),
        ValorFast::flotante(0.0),
        ValorFast::flotante(10.0),
    ]);
    Ok(ValorFast::arreglo(arr_idx))
}

fn android_bateria_nivel(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::entero(100))
}

fn android_bateria_cargando(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::booleano(true))
}

fn android_sensor_acelerometro(vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let arr_idx = vm.alloc_arr(vec![
        ValorFast::flotante(0.0),
        ValorFast::flotante(9.81),
        ValorFast::flotante(0.0),
    ]);
    Ok(ValorFast::arreglo(arr_idx))
}

fn android_sensor_giroscopio(vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    let arr_idx = vm.alloc_arr(vec![
        ValorFast::flotante(0.0),
        ValorFast::flotante(0.0),
        ValorFast::flotante(0.0),
    ]);
    Ok(ValorFast::arreglo(arr_idx))
}

fn android_camara_tomar_foto(vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(texto_val(vm, ""))
}

fn android_camara_escanear_qr(vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(texto_val(vm, ""))
}

fn android_permiso_verificar(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::booleano(true))
}

fn android_permiso_solicitar(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::nulo())
}

fn android_biometria_autenticar(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::booleano(true))
}

fn android_compartir_texto(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::nulo())
}

fn android_abrir_url(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::nulo())
}

fn android_clipboard_copiar(_vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(ValorFast::nulo())
}

fn android_clipboard_pegar(vm: &mut ForjaFast, _args: &[ValorFast]) -> Result<ValorFast, ErrFast> {
    Ok(texto_val(vm, ""))
}


/// Crea un NativeRegistry con todas las funciones Android.
pub fn crear_registry_android() -> NativeRegistry {
    let mut registry = NativeRegistry::new();
    registrar_nativas_android(&mut registry);
    registry
}
