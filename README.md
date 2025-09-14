# Rust Android Jemalloc Memory Profiling Example

This project demonstrates how to integrate a Rust library with Android using JNI and enable native memory profiling with tikv-jemalloc. This is the industry-standard approach for debugging memory leaks in Rust libraries used by Android applications.

## Features

- Rust library with tikv-jemalloc as the global allocator
- Memory profiling capabilities (heap dumps, statistics)
- JNI integration between Rust and Android
- Example functions to test memory allocation and leaks
- Android UI to interact with the native library

## Project Structure

```
rust-android-jemalloc/
├── core/                   # Rust library
│   ├── Cargo.toml         # Rust dependencies
│   ├── src/
│   │   └── lib.rs         # Main library implementation
│   └── build.rs           # Build configuration
├── android/               # Android application
│   ├── app/
│   │   ├── build.gradle   # App build configuration
│   │   └── src/
│   │       └── main/
│   │           ├── java/com/example/rustjemalloc/
│   │           │   └── MainActivity.kt
│   │           └── res/
│   ├── build.gradle       # Project build configuration
│   ├── settings.gradle
│   └── gradle.properties
└── README.md
```

## Prerequisites

1. **Rust**: Install from [rustup.rs](https://rustup.rs/)
2. **Android Studio**: Latest version with NDK support
3. **Android NDK**: Version 25.x or later
4. **cargo-ndk**: Install with `cargo install cargo-ndk`

## Setup

### 1. Install Rust targets for Android

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

### 2. Configure Android SDK and NDK

Create `android/local.properties` file (copy from `local.properties.template`):

```properties
sdk.dir=/path/to/your/Android/sdk
ndk.dir=/path/to/your/Android/sdk/ndk/25.2.9519653
```

Or set environment variables:
```bash
export ANDROID_SDK_HOME=/path/to/your/Android/sdk
export ANDROID_NDK_HOME=/path/to/your/Android/sdk/ndk/25.2.9519653
```

### 3. Build and Run

From the `android` directory:

```bash
# Build debug APK
./gradlew assembleDebug

# Install on connected device
./gradlew installDebug

# Build release APK
./gradlew assembleRelease
```

## Memory Profiling Features

### 1. Memory Statistics

The app can display current memory statistics including:
- Allocated bytes
- Resident memory
- Active memory
- Mapped memory

### 2. Heap Dumps

Generate heap profiles in jemalloc format that can be analyzed with:
- `jeprof` tool
- pprof format converters
- Custom analysis tools

Heap dumps are saved to: `/Android/data/com.example.rustjemalloc/files/heap_dumps/`

### 3. Leak Testing

The app includes functions to:
- Allocate and intentionally leak memory
- Clear leaked memory
- Monitor memory growth

## Analyzing Heap Dumps

### Using jeprof

```bash
# Pull heap dump from device
adb pull /sdcard/Android/data/com.example.rustjemalloc/files/heap_dumps/heap_profile_*.prof

# Analyze with jeprof
jeprof --text heap_profile_*.prof
```

### Converting to pprof format

For better visualization, you can use tools like [rust-jemalloc-pprof](https://github.com/polarsignals/rust-jemalloc-pprof) to convert jemalloc profiles to pprof format.

## Technical Details

### Jemalloc Configuration

The library configures jemalloc with:
```rust
pub static malloc_conf: &[u8] = b"prof:true,prof_active:true,lg_prof_sample:19\0";
```

- `prof:true` - Enable profiling
- `prof_active:true` - Start profiling immediately
- `lg_prof_sample:19` - Sample every ~512KB (2^19 bytes)

### JNI Functions

The library exposes these JNI functions:
- `nativeHello()` - Basic test function
- `nativeGetMemoryStats()` - Get current memory statistics
- `nativeDumpHeapProfile(path)` - Dump heap profile to file
- `nativeAllocateAndLeak(sizeMB)` - Allocate and leak memory (for testing)
- `nativeClearLeakedMemory()` - Clear leaked memory

### Build Process

The cargo-ndk gradle plugin handles:
1. Building Rust library for all Android architectures
2. Copying .so files to correct jniLibs directories
3. Packaging native libraries with the APK

## Troubleshooting

### Common Issues

1. **NDK not found**: Ensure `ANDROID_NDK_HOME` is set or `ndk.dir` is in local.properties

2. **Build failures**: Check that all Rust targets are installed

3. **Library not loading**: Verify library name matches in:
   - Cargo.toml (`[lib] name = "rust_jemalloc_core"`)
   - MainActivity.kt (`System.loadLibrary("rust_jemalloc_core")`)
   - build.gradle (`librariesNames = ["librust_jemalloc_core.so"]`)

4. **Permission denied**: Ensure app has storage permissions for heap dumps

### Debug Tips

1. Check native crashes in logcat:
```bash
adb logcat | grep -E "rust|jemalloc|native"
```

2. Verify library is included:
```bash
unzip -l app/build/outputs/apk/debug/app-debug.apk | grep .so
```

## Production Considerations

1. **Profiling overhead**: Jemalloc profiling adds ~5-10% overhead. Disable in production builds.

2. **Binary size**: Jemalloc adds ~300KB to the binary size.

3. **Security**: Heap dumps may contain sensitive data. Implement proper access controls.

4. **Storage**: Heap dumps can be large. Implement cleanup policies.

## References

- [tikv-jemallocator](https://github.com/tikvm/jemallocator)
- [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- [Android NDK](https://developer.android.com/ndk)
- [jemalloc profiling](https://github.com/jemalloc/jemalloc/wiki/Use-Case:-Heap-Profiling)

## License

This example is provided as-is for educational purposes.