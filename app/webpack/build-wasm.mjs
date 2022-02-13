import {resolvePath, resolveRustOutputPath} from './paths.mjs'
import {spawn, SpawnError} from './child-process.mjs'

export async function buildWasm(mode = 'production') {
  try {
    const bindgenArgs = ['--weak-refs', '--browser', '--debug']
    const cargoArgs = []
    let targetDirectory = 'debug'
    if (mode === 'production') {
      cargoArgs.push('--release')
      targetDirectory = 'release'
      bindgenArgs.splice(-1, 1)
    }
    await spawn(
      'cargo',
      'build',
      '--target',
      'wasm32-unknown-unknown',
      ...cargoArgs
    )
    await spawn(
      'wasm-bindgen',
      ...bindgenArgs,
      '--out-name',
      'app',
      '--out-dir',
      resolvePath('wasm'),
      resolveRustOutputPath(
        'wasm32-unknown-unknown',
        targetDirectory,
        'rust_pwa_app.wasm'
      )
    )
  } catch (error) {
    if (error instanceof SpawnError) {
      if (error.exitCode < 0) {
        console.error(error)
      }
      process.exit(error.exitCode)
    }
  }
}
