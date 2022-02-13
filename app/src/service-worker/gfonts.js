import {registerRoute} from 'workbox-routing'
import {CacheFirst} from 'workbox-strategies'
import {CacheableResponsePlugin} from 'workbox-cacheable-response'
import {ExpirationPlugin} from 'workbox-expiration'

const GOOGLE_FONTS_ORIGIN_REGEX =
  /https?:\/\/fonts\.(googleapis|gstatic)\.com/iu

export function registerGoogleFontRoute() {
  registerRoute(
    ({url}) => GOOGLE_FONTS_ORIGIN_REGEX.test(url.origin),
    new CacheFirst({
      cacheName: 'google-fonts-webfonts',
      plugins: [
        new CacheableResponsePlugin({
          statuses: [0, 200],
        }),
        new ExpirationPlugin({
          maxAgeSeconds: 3600 * 24 * 365,
          maxEntries: 30,
        }),
      ],
    })
  )
}
