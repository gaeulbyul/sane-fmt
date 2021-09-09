import { WASI } from '@wasmer/wasi'
import bindings from '@wasmer/wasi/lib/bindings/node'
import fs from 'fs'
import path from 'path'

export const wasmFile = require.resolve('../sane-fmt.wasm')

export interface Process {
  readonly argv: readonly string[]
  readonly cwd: () => string
}

export async function main(process: Process): Promise<void> {
  const args = ['sane-fmt', ...process.argv.slice(2)]
  const wdir = process.cwd()

  const wasi = new WASI({
    args,
    preopenDirectories: {
      '/': path.parse(wdir).root,
      '.': '.',
    },
    bindings,
  })

  const wasmBytes = fs.readFileSync(wasmFile)
  const wasmModule = await WebAssembly.compile(wasmBytes)
  const wasmInstance = await WebAssembly.instantiate(wasmModule, {
    ...wasi.getImports(wasmModule),
  })

  wasi.start(wasmInstance)
}
