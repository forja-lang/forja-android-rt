# forja-android-rt

Runtime nativo de **Forja (fa)** para Android: puente JNI + `NativeActivity`
para ejecutar aplicaciones Forja en el dispositivo.

[![CI](https://github.com/forja-lang/forja-android-rt/actions/workflows/ci.yml/badge.svg)](https://github.com/forja-lang/forja-android-rt/actions/workflows/ci.yml)

## Qué incluye

- **Puente JNI** (`src/jni_bridge.rs`, `src/lib.rs`) para crear sesiones de la VM
  de Forja, compilar a bytecode, ejecutar código y evaluar expresiones.
- **API Kotlin** (`android/src/com/forja/`) con `ForjaRuntime`, `ForjaSession`,
  `ForjaResult` y errores tipados (`ForjaCompileError`, `ForjaTimeoutError`, …).
- **Hot-Reload** por TCP/ADB/Wi-Fi en el puerto `7355` (`src/hot_reload.rs`).
- **Runtime GUI nativo** vía `forja-gui-rt` y `NativeActivity`
  (`android_main` en `src/lib.rs`).

El crate es un `cdylib`: produce `libforja_android_rt.so`, no una librería Rust
reutilizable.

## Uso desde Kotlin

```kotlin
import com.forja.ForjaRuntime

val sesion = ForjaRuntime.crearSession()
try {
    val resultado = sesion.ejecutar("""escribir("Hola Android!")""")
    resultado.output.forEach { Log.d("Forja", it) }
} finally {
    sesion.destruir()
}
```

El AAR expone `com.forja.ForjaRuntime` y se carga con
`System.loadLibrary("forja_android_rt")`. `minSdkVersion` es 26.

## Compilar el AAR

Requiere el Android NDK (r25+) y los targets de Rust para Android.

```bash
# Desde la raíz del repo forja
bash scripts/build-aar.sh                    # 4 ABIs + dist/forja-android-rt-<version>.aar
bash scripts/build-aar.sh --package-only     # reempaqueta .so ya compilados
```

También se puede compilar un solo ABI (lo usa el CI en paralelo):

```bash
bash scripts/build-aar.sh --target aarch64-linux-android --build-only
```

Los `.so` quedan en `dist/jni/<abi>/libforja_android_rt.so` y el AAR incluye
`arm64-v8a`, `x86_64`, `armeabi-v7a` y `x86`.

Perfil de Cargo: `dist-android` (opt-level `z`, `panic = "abort"`, strip,
ThinLTO). Se puede cambiar con `FORJA_AAR_PROFILE`.

## Tests

Los tests unitarios corren en el host desde el workspace padre:

```bash
cargo test -p forja-android-rt --locked
```

El CI del repo compila el crate dentro del workspace Forja y ejecuta
`fmt`, `clippy` y `test`. La validación de los `.so` por ABI la hace el
workflow `cd.yml` del repo padre.

## Estado

Versión `0.9.5`; el consumo es por ruta desde el workspace Forja.

## Licencia

GPL-3.0-or-later. Ver `LICENSE.md` en [forja-lang/forja](https://github.com/forja-lang/forja).
