import './styles.scss'
import './themes/dark.scss'
import './themes/light.scss'

try {
  const app = await import('../wasm/app.js')

  if ('serviceWorker' in window.navigator) {
    const {Workbox} = await import('workbox-window')
    const workbox = new Workbox('/sw.js')
    await workbox.register()
    workbox.messageSkipWaiting()
    await workbox.active
  }
  app.start()
} catch (error) {
  console.error(error)
}
