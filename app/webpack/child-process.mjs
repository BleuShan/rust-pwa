import {spawn as nodeSpawn} from 'node:child_process'
import {resolvePath} from './paths.mjs'

export function spawn(command, ...args) {
  let spawnOpts
  if (command != null && typeof command === 'object') {
    const {name, ...opts} = command
    command = name
    spawnOpts = opts
  }
  const childProcess = nodeSpawn(command, args, {
    cwd: resolvePath(),
    stdio: 'inherit',
    ...spawnOpts,
  })
  const pid = childProcess.pid
  console.info(`process ${command}(${pid}) started.`)
  return new Promise((resolve, reject) => {
    childProcess.on('close', (code) => {
      if (code !== 0) {
        reject(new SpawnError(command, pid, code))
        return
      }
      console.info(`process ${command}(${pid}) terminated succesfully`)
      resolve()
    })
  })
}

export class SpawnError extends Error {
  #exitCode
  get exitCode() {
    return this.#exitCode
  }

  constructor(command, pid, exitCode) {
    super(`Process ${command}(${pid}) exited with code: ${exitCode}`)
    Object.setPrototypeOf(this, new.target.prototype)
    this.#exitCode = exitCode
  }
}
