import * as precaching from 'workbox-precaching'
export function precacheAndRoute() {
  precaching.cleanupOutdatedCaches()
  precaching.precacheAndRoute(self.__WB_MANIFEST)
}
