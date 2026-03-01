;; Vantis Media Player - WAT Plugin Template
;; 
;; This is a template for creating plugins in WebAssembly Text format (WAT)
;; WAT is the human-readable version of WebAssembly

(module
  ;; Import host functions from Vantis
  (import "vantis" "log" (func $log (param i32 i32)))
  (import "vantis" "log_info" (func $log_info (param i32 i32)))
  (import "vantis" "log_warn" (func $log_warn (param i32 i32)))
  (import "vantis" "log_error" (func $log_error (param i32 i32)))
  (import "vantis" "play" (func $play))
  (import "vantis" "pause" (func $pause))
  (import "vantis" "stop" (func $stop))
  (import "vantis" "seek" (func $seek (param i64)))
  (import "vantis" "get_time" (func $get_time (result i64)))
  (import "vantis" "sleep" (func $sleep (param i32)))
  
  ;; Export linear memory
  (memory (export "memory") 2)
  
  ;; Global state
  (global $initialized (mut i32) (i32.const 0))
  (global $tick_count (mut i32) (i32.const 0))
  
  ;; Data section - strings
  (data (i32.const 0) "Vantis WAT Plugin")
  (data (i32.const 18) "1.0.0")
  (data (i32.const 24) "Plugin initialized successfully!")
  (data (i32.const 52) "Plugin shutting down...")
  (data (i32.const 74) "Tick function called")
  (data (i32.const 94) "Plugin not initialized")
  
  ;; === Required exports ===
  
  ;; Initialize the plugin
  ;; Returns 0 on success, non-zero on error
  (func (export "init") (result i32)
    ;; Log initialization
    (call $log_info
      (i32.const 24)  ;; Pointer to "Plugin initialized successfully!"
      (i32.const 29)   ;; Length
    )
    
    ;; Set initialized flag
    (global.set $initialized (i32.const 1))
    
    ;; Return success
    (i32.const 0)
  )
  
  ;; Shutdown the plugin
  ;; Returns 0 on success, non-zero on error
  (func (export "shutdown") (result i32)
    ;; Check if initialized
    (if (i32.eqz (global.get $initialized))
      (then
        (return (i32.const 1))  ;; Error: not initialized
      )
    )
    
    ;; Log shutdown
    (call $log_info
      (i32.const 52)  ;; Pointer to "Plugin shutting down..."
      (i32.const 23)   ;; Length
    )
    
    ;; Reset initialized flag
    (global.set $initialized (i32.const 0))
    
    ;; Return success
    (i32.const 0)
  )
  
  ;; Tick function - called periodically
  ;; Returns 0 on success, non-zero on error
  (func (export "tick") (result i32)
    ;; Check if initialized
    (if (i32.eqz (global.get $initialized))
      (then
        (call $log_warn
          (i32.const 94)  ;; Pointer to "Plugin not initialized"
          (i32.const 21)   ;; Length
        )
        (return (i32.const 1))
      )
    )
    
    ;; Increment tick counter
    (global.set $tick_count
      (i32.add
        (global.get $tick_count)
        (i32.const 1)
      )
    )
    
    ;; Log tick (every 100 ticks to avoid spam)
    (if (i32.eq
      (i32.rem_u (global.get $tick_count) (i32.const 100))
      (i32.const 0)
    )
      (then
        (call $log_info
          (i32.const 74)  ;; Pointer to "Tick function called"
          (i32.const 19)   ;; Length
        )
      )
    )
    
    ;; Return success
    (i32.const 0)
  )
  
  ;; === Optional exports ===
  
  ;; Get plugin name
  ;; Returns pointer and length of name string
  (func (export "get_name") (result i32 i32)
    (return
      (i32.const 0)   ;; Pointer to "Vantis WAT Plugin"
      (i32.const 17)   ;; Length
    )
  )
  
  ;; Get plugin version
  ;; Returns pointer and length of version string
  (func (export "get_version") (result i32 i32)
    (return
      (i32.const 18)  ;; Pointer to "1.0.0"
      (i32.const 5)    ;; Length
    )
  )
  
  ;; Get plugin description
  ;; Returns pointer and length of description string
  (func (export "get_description") (result i32 i32)
    (local $desc_ptr i32)
    
    ;; Allocate space for description
    (local.set $desc_ptr (i32.const 120))
    
    ;; Write description string
    (data.drop (data (i32.const 120) "Example plugin created with WAT"))
    
    (return
      (local.get $desc_ptr)
      (i32.const 35)  ;; Length
    )
  )
  
  ;; Get tick count
  ;; Returns the number of times tick() has been called
  (func (export "get_tick_count") (result i32)
    (global.get $tick_count)
  )
  
  ;; Custom function: Trigger playback
  (func (export "trigger_play")
    (if (i32.eqz (global.get $initialized))
      (then
        (return)
      )
    )
    
    (call $play)
  )
  
  ;; Custom function: Pause playback
  (func (export "trigger_pause")
    (if (i32.eqz (global.get $initialized))
      (then
        (return)
      )
    )
    
    (call $pause)
  )
  
  ;; Custom function: Seek to position (in milliseconds)
  (func (export "trigger_seek" (param $position_ms i64))
    (if (i32.eqz (global.get $initialized))
      (then
        (return)
      )
    )
    
    (call $seek (local.get $position_ms))
  )
  
  ;; Custom function: Get current time from host
  (func (export "get_current_time") (result i64)
    (call $get_time)
  )
  
  ;; Custom function: Sleep for specified milliseconds
  (func (export "plugin_sleep" (param $ms i32))
    (call $sleep (local.get $ms))
  )
)