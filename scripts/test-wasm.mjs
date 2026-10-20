import { readFileSync, readdirSync } from "node:fs";
import { execFileSync } from "node:child_process";
import assert from "node:assert/strict";
const {
  instance: { exports: e },
} = await WebAssembly.instantiate(readFileSync("dist/qrunap2.wasm"), {});
function run(source, seed = 42) {
  const bytes = new TextEncoder().encode(source);
  const ptr = e.input_buffer(bytes.length);
  assert.ok(ptr);
  new Uint8Array(e.memory.buffer, ptr, bytes.length).set(bytes);
  e.execute(seed);
  return JSON.parse(
    new TextDecoder().decode(
      new Uint8Array(e.memory.buffer, e.output_pointer(), e.output_length()),
    ),
  );
}
function close(a, b, path = "result") {
  if (typeof a === "number") {
    assert.ok(
      Math.abs(a - b) <= 1e-8 * Math.max(1, Math.abs(a)),
      `${path}: ${a} != ${b}`,
    );
  } else if (a && typeof a === "object") {
    assert.deepEqual(Object.keys(a).sort(), Object.keys(b).sort());
    for (const key of Object.keys(a)) close(a[key], b[key], `${path}.${key}`);
  } else assert.equal(a, b, path);
}
for (const dir of ["original", "historical"])
  for (const file of readdirSync(`examples/${dir}`).filter((f) =>
    f.endsWith(".qnp"),
  )) {
    const path = `examples/${dir}/${file}`,
      source = readFileSync(path, "utf8");
    const wasm = run(source);
    assert.equal(wasm.ok, true, wasm.error);
    const native = JSON.parse(
      execFileSync(
        "cargo",
        ["run", "--locked", "--quiet", "--", "run", path, "42"],
        { encoding: "utf8" },
      ),
    );
    close(wasm.report, native);
    assert.deepEqual(run(source), wasm);
    console.log(`PASS native/WASM parity: ${file}`);
  }
assert.equal(run("/DECLARE/ QUEUE BROKEN;").ok, false);
assert.equal(
  run(readFileSync("examples/original/first-queue.qnp", "utf8"), 0).ok,
  false,
);
console.log("PASS WASM diagnostics and repeated calls");
