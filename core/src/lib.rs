use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::CString;
use std::fs;
use std::path::Path;

// Configure jemalloc as the global allocator with profiling enabled
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

// Configure jemalloc with profiling options
#[allow(non_upper_case_globals)]
#[export_name = "malloc_conf"]
pub static malloc_conf: &[u8] = b"prof:true,prof_active:true,lg_prof_sample:19\0";

// Basic test function exposed to Android
#[no_mangle]
pub extern "C" fn Java_com_example_rustjemalloc_MainActivity_nativeHello(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    // Allocate some memory to test jemalloc
    let mut test_vec: Vec<u8> = Vec::with_capacity(1024 * 1024); // 1MB
    for i in 0..1024 * 1024 {
        test_vec.push((i % 256) as u8);
    }
    
    let output = format!("Hello from Rust! Allocated {} bytes", test_vec.len());
    let output_jstring = env.new_string(output).expect("Couldn't create java string!");
    
    // Keep the allocation alive but don't leak it
    drop(test_vec);
    
    output_jstring.into_raw()
}

// Get memory statistics
#[no_mangle]
pub extern "C" fn Java_com_example_rustjemalloc_MainActivity_nativeGetMemoryStats(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    #[cfg(not(target_env = "msvc"))]
    {
        use tikv_jemalloc_ctl::{epoch, stats};
        
        // Update statistics
        let _ = epoch::advance();
        
        let allocated = stats::allocated::read().unwrap_or(0);
        let resident = stats::resident::read().unwrap_or(0);
        let active = stats::active::read().unwrap_or(0);
        let mapped = stats::mapped::read().unwrap_or(0);
        
        let output = format!(
            "Memory Stats:\nAllocated: {} bytes\nResident: {} bytes\nActive: {} bytes\nMapped: {} bytes",
            allocated, resident, active, mapped
        );
        
        let output_jstring = env.new_string(output).expect("Couldn't create java string!");
        output_jstring.into_raw()
    }
    
    #[cfg(target_env = "msvc")]
    {
        let output = "Memory profiling not available on this platform";
        let output_jstring = env.new_string(output).expect("Couldn't create java string!");
        output_jstring.into_raw()
    }
}

// Dump heap profile
#[no_mangle]
pub extern "C" fn Java_com_example_rustjemalloc_MainActivity_nativeDumpHeapProfile(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jstring {
    #[cfg(not(target_env = "msvc"))]
    {
        use tikv_jemalloc_ctl::prof;
        
        let dump_path: String = env
            .get_string(&path)
            .expect("Couldn't get java string!")
            .into();
        
        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(&dump_path).parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        // Dump the heap profile
        match prof::dump::write(|buf| {
            fs::write(&dump_path, buf).map_err(|e| {
                eprintln!("Failed to write heap dump: {}", e);
                std::io::Error::new(std::io::ErrorKind::Other, "Write failed")
            })
        }) {
            Ok(_) => {
                let output = format!("Heap profile dumped to: {}", dump_path);
                let output_jstring = env.new_string(output).expect("Couldn't create java string!");
                output_jstring.into_raw()
            }
            Err(e) => {
                let output = format!("Failed to dump heap profile: {:?}", e);
                let output_jstring = env.new_string(output).expect("Couldn't create java string!");
                output_jstring.into_raw()
            }
        }
    }
    
    #[cfg(target_env = "msvc")]
    {
        let output = "Heap profiling not available on this platform";
        let output_jstring = env.new_string(output).expect("Couldn't create java string!");
        output_jstring.into_raw()
    }
}

// Allocate and hold memory (for testing leaks)
static mut LEAK_VECTOR: Option<Vec<u8>> = None;

#[no_mangle]
pub extern "C" fn Java_com_example_rustjemalloc_MainActivity_nativeAllocateAndLeak(
    mut env: JNIEnv,
    _class: JClass,
    size_mb: i32,
) -> jstring {
    let size_bytes = (size_mb as usize) * 1024 * 1024;
    
    unsafe {
        // Leak the previous allocation if any
        if LEAK_VECTOR.is_some() {
            LEAK_VECTOR = None;
        }
        
        // Create new allocation
        let mut vec = Vec::with_capacity(size_bytes);
        for i in 0..size_bytes {
            vec.push((i % 256) as u8);
        }
        
        LEAK_VECTOR = Some(vec);
    }
    
    let output = format!("Allocated and leaked {} MB", size_mb);
    let output_jstring = env.new_string(output).expect("Couldn't create java string!");
    output_jstring.into_raw()
}

// Clear leaked memory
#[no_mangle]
pub extern "C" fn Java_com_example_rustjemalloc_MainActivity_nativeClearLeakedMemory(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    unsafe {
        if let Some(vec) = LEAK_VECTOR.take() {
            let size = vec.len();
            drop(vec);
            let output = format!("Cleared {} bytes of leaked memory", size);
            let output_jstring = env.new_string(output).expect("Couldn't create java string!");
            output_jstring.into_raw()
        } else {
            let output = "No leaked memory to clear";
            let output_jstring = env.new_string(output).expect("Couldn't create java string!");
            output_jstring.into_raw()
        }
    }
}