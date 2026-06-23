# Architecture: V8-Spider Mock Integration

## Overview
Bobzilla's core performance mandate is to close the gap between Mozilla's SpiderMonkey and Google's V8 engine. While embedding a full V8 engine into Gecko is practically impossible (due to deep DOM binding dependencies, garbage collection mismatches, and memory models), this architecture document outlines a "Smart Dispatcher" mocking approach.

## The "Smart Dispatcher" Concept
Instead of replacing SpiderMonkey, Bobzilla will implement a conceptual C++ dispatcher layer inside the Javascript engine pipeline.

```cpp
// Conceptual mock of a dispatcher that intercepts heavy JS workloads
class V8SpiderDispatcher {
public:
    static bool ShouldOffloadToV8(const JS::Handle<JSObject*>& script) {
        // Analyze script payload. If it's pure computational WebAssembly
        // or a heavy math loop without DOM manipulation, route to V8.
        return AnalyzeHeuristics(script) == WorkloadType::HEAVY_COMPUTE;
    }

    static JSRuntime* InitializeHybridRuntime() {
        // Boot standard SpiderMonkey
        JSRuntime* sm = js::NewRuntime();
        // Conceptually spin up isolated V8 isolate for WebWorkers
        v8::Isolate* v8_isolate = v8::Isolate::New();

        return sm; // Fallback
    }
};
```

## Implementation Phases (Mock)

### Phase 1: Flag Enablement
Ensure all extreme performance flags are enabled in `mozconfig.bobzilla`:
- `ac_add_options --enable-lto=cross` (Link Time Optimization)
- `ac_add_options --enable-profile-use=cross` (Profile Guided Optimization)

### Phase 2: WebWorker V8 Offloading
The only safe place to theoretically run V8 inside Firefox is within an isolated WebWorker context, where DOM access is strictly forbidden by the specification.
- A patch will intercept `WorkerPrivate::Start()` in `dom/workers/WorkerPrivate.cpp`.
- If a specific `bobzilla.v8_workers` flag is true, it routes the JS execution environment to an embedded V8 Isolate instead of a SpiderMonkey Context.

### Phase 3: The Javasandbox Convergence
If the `javasandbox` MicroVM architecture (Firecracker) is successfully implemented, the V8 offloading concept may become obsolete. Heavy background computations can simply be routed to a headless JVM instance running inside the Firecracker guest OS, completely bypassing Javascript bottlenecks.
