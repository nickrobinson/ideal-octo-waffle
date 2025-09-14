package com.example.rustjemalloc

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.ScrollView
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import java.io.File
import java.text.SimpleDateFormat
import java.util.*

class MainActivity : AppCompatActivity() {
    
    private lateinit var outputTextView: TextView
    private lateinit var sizeEditText: EditText
    private lateinit var scrollView: ScrollView
    
    companion object {
        init {
            System.loadLibrary("rust_jemalloc_core")
        }
    }
    
    // Native method declarations
    private external fun nativeHello(): String
    private external fun nativeGetMemoryStats(): String
    private external fun nativeDumpHeapProfile(path: String): String
    private external fun nativeAllocateAndLeak(sizeMB: Int): String
    private external fun nativeClearLeakedMemory(): String
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        
        outputTextView = findViewById(R.id.outputTextView)
        sizeEditText = findViewById(R.id.sizeEditText)
        scrollView = findViewById(R.id.scrollView)
        
        findViewById<Button>(R.id.btnHello).setOnClickListener {
            appendOutput(nativeHello())
        }
        
        findViewById<Button>(R.id.btnMemStats).setOnClickListener {
            appendOutput(nativeGetMemoryStats())
        }
        
        findViewById<Button>(R.id.btnDumpHeap).setOnClickListener {
            dumpHeapProfile()
        }
        
        findViewById<Button>(R.id.btnAllocateLeak).setOnClickListener {
            allocateAndLeak()
        }
        
        findViewById<Button>(R.id.btnClearLeak).setOnClickListener {
            appendOutput(nativeClearLeakedMemory())
        }
        
        findViewById<Button>(R.id.btnClearOutput).setOnClickListener {
            outputTextView.text = ""
        }
    }
    
    private fun appendOutput(text: String) {
        val timestamp = SimpleDateFormat("HH:mm:ss", Locale.getDefault()).format(Date())
        outputTextView.append("[$timestamp] $text\n\n")
        scrollView.post {
            scrollView.fullScroll(ScrollView.FOCUS_DOWN)
        }
    }
    
    private fun dumpHeapProfile() {
        val dir = File(getExternalFilesDir(null), "heap_dumps")
        if (!dir.exists()) {
            dir.mkdirs()
        }
        
        val timestamp = SimpleDateFormat("yyyyMMdd_HHmmss", Locale.getDefault()).format(Date())
        val filename = "heap_profile_$timestamp.prof"
        val file = File(dir, filename)
        
        val result = nativeDumpHeapProfile(file.absolutePath)
        appendOutput(result)
    }
    
    private fun allocateAndLeak() {
        val sizeText = sizeEditText.text.toString()
        val sizeMB = sizeText.toIntOrNull() ?: 10
        appendOutput(nativeAllocateAndLeak(sizeMB))
    }
}