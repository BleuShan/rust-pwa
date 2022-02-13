import upath from 'upath'
import {fileURLToPath} from 'node:url'
export const {dirname, resolve, join, relative, sep} = upath

const __dirname = dirname(fileURLToPath(import.meta.url))

export function resolvePath(...paths) {
  return resolve(__dirname, '..', ...paths)
}

export function resolveRootPath(...paths) {
  return resolve(__dirname, '..', '..', ...paths)
}

export function resolveSrcPath(...paths) {
  return resolvePath('src', ...paths)
}

export function resolveOutputPath(...paths) {
  return resolveRootPath('server', 'static', ...paths)
}

export function resolveRustOutputPath(...paths) {
  return resolveRootPath('target', ...paths)
}

export function resolveAssetsPath(...paths) {
  return resolvePath('assets', ...paths)
}
