let engine;
self.onmessage = async ({ data }) => {
  try {
    if (!engine) {
      const response = await fetch("qrunap2.wasm");
      if (!response.ok)
        throw new Error(`WASM download failed: HTTP ${response.status}`);
      engine = (await WebAssembly.instantiate(await response.arrayBuffer(), {}))
        .instance.exports;
    }
    const bytes = new TextEncoder().encode(data.source);
    if (bytes.length > 65536) throw new Error("Source exceeds 64 KiB");
    const ptr = engine.input_buffer(bytes.length);
    if (!ptr) throw new Error("Cannot allocate input");
    new Uint8Array(engine.memory.buffer, ptr, bytes.length).set(bytes);
    engine.execute(data.seed);
    const text = new TextDecoder().decode(
      new Uint8Array(
        engine.memory.buffer,
        engine.output_pointer(),
        engine.output_length(),
      ),
    );
    self.postMessage(JSON.parse(text));
  } catch (error) {
    self.postMessage({ ok: false, error: String(error.message || error) });
  }
};
