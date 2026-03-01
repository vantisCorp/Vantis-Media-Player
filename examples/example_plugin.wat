;; Example WASM Plugin for Vantis Media Player
;; This demonstrates the plugin API

(module
  ;; Import host functions
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
  
  ;; Memory
  (memory (export "memory") 1)
  
  ;; Export plugin info
  (global $plugin_name (mut i32) (i32.const 0))
  (global $plugin_version (mut i32) (i32.const 20))
  
  ;; Data section
  (data (i32.const 0) "Vantis Example Plugin")
  (data (i32.const 20) "1.0.0")
  (data (i32.const 26) "Plugin initialized!")
  (data (i32.const 46) "Plugin shutting down...")
  
  ;; Export init function
  (func (export "init") (result i32)
    ;; Call log_info with initialization message
    (call $log_info
      (i32.const 26)  ;; Pointer to "Plugin initialized!"
      (i32.const 21)   ;; Length
    )
    
    (i32.const 0)  ;; Return success
  )
  
  ;; Export shutdown function
  (func (export "shutdown") (result i32)
    ;; Call log_info with shutdown message
    (call $log_info
      (i32.const 46)  ;; Pointer to "Plugin shutting down..."
      (i32.const 23)   ;; Length
    )
    
    (i32.const 0)  ;; Return success
  )
  
  ;; Export tick function (called periodically)
  (func (export "tick") (result i32)
    ;; Get current time
    (local $time i64)
    (local.set $time (call $get_time))
    
    ;; Do periodic work here
    
    (i32.const 0)  ;; Return success
  )
)