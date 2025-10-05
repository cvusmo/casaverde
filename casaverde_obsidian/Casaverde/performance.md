How to improve performance for running everything on one device?Since this is an early concept and everything runs on one device (e.g., Raspberry Pi), the HTTP-based flow adds unnecessary network overhead (even on localhost). Current setup is functional but not optimized. Here's how to improve, starting simple:Short-term (Minimal Changes):Use localhost for all: Set server = "https://127.0.0.1:3003" in all configs. This avoids external network latency (current is "10.0.0.6", which might be LAN).
Reduce intervals: Controller's 5s tick and app's 1s sleep are fine, but align them (e.g., 5s everywhere) to minimize fetches. Add cache expiry in server (e.g., ignore data >30s old) to prevent stale displays.
Fix serial reading: Modify read_sensor_data to read lines iteratively (use a loop with read_until('\n') or split the buffer by '\n' and parse each). This handles multiple Arduino prints without failing.
Disable unnecessary sends: In app's update_devices, remove the POST if >5s—it's redundant on one device.
Profile: Use logs/timings (add Instant::elapsed()) in loops to identify bottlenecks (e.g., serial reads or HTTP).

Medium-term (Refactor for Efficiency):Switch to local IPC: Replace HTTP with Tokio channels or shared memory (e.g., Arc<Mutex<HashMap>> across binaries via a shared utils crate). Controller writes to shared cache; app/server read from it. This eliminates TLS/HTTP overhead.
Merge binaries: Run controller logic as a thread in the app (since app has TUI). Use tokio::spawn for background sensor reading/caching. Server could be optional for multi-device scaling later.
Async optimizations: Use non-blocking serial (e.g., tokio-serial crate) instead of blocking serialport. Batch commands/reads.
Data-oriented design: As you mentioned, structure data as pure structs (e.g., expand DeviceReading with timestamps/errors). Use vectors/arrays over hashes where possible for cache locality. Avoid clones (use refs/Arc).

Long-term (Scalability):Add persistence: Use a lightweight DB like SQLite for cache (instead of in-memory) to survive restarts.
Error handling: Add retries for serial/HTTP failures.
Monitoring: Integrate Prometheus or simple metrics for CPU/network usage.
Test on hardware: Simulate load (e.g., multiple fake sensors) to benchmark before/after.


