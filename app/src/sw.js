import * as navigationPreload from 'workbox-navigation-preload'
import {cacheNames, setCacheNameDetails} from 'workbox-core'
import {registerGoogleFontRoute} from './service-worker/gfonts.js'
import './service-worker/messages.js'
import {precacheAndRoute} from './service-worker/precaching.js'
import {registerRoute} from 'workbox-routing'

setCacheNameDetails({
  prefix: 'BlazorSyncStudy',
  suffix: `v${__APP_VERSION__}`,
})
navigationPreload.enable()
precacheAndRoute()
registerGoogleFontRoute()

registerRoute(
  ({request}) => {
    const url = new URL(request.url)
    return url.origin === self.origin && request.mode === 'navigate'
  },
  {
    async handle({event, request}) {
      let response
      try {
        if (event.preloadResponse) {
          response = await event.preloadResponse
        }
        if (navigator.connection.type !== 'none') {
          const networkResponse = await fetch('/index.html')
          const cache = await caches.open(
            `${cacheNames.prefix}-navigation-${cacheNames.suffix}`
          )
          cache.put('/index.html', networkResponse.clone())
          if (response == null) {
            response = networkResponse
          }
        }
      } catch (error) {
        console.error(error)
      }

      if (response == null) {
        response = await caches.match('/index.html')
      }

      return response
    },
  }
)
