export class MessageDispatcher {
  #handlers = new Set();

  *handlers(predicate) {
    for (const handler of this.#handlers) {
      const valid = predicate?.(handler) ?? true
      if (!valid) continue
      yield handler
    }
  }

  register(handler) {
    this.#handlers.add(handler)
    return this
  }

  unregister(handler) {
    this.#handlers.delete(handler)
    return this
  }
}

export class SkipWaitingMessageHandler {
  canExecute(event) {
    const data =
      event instanceof ExtendableMessageEvent ? event.data : undefined
    if (data == null || typeof data !== 'object') return false
    const payloadType = Reflect.get(data, 'type')
    return payloadType === 'SKIP_WAITING'
  }

  async execute(event) {
    await self.skipWaiting()
  }
}

const DEFAULT_DISPATCHER = new MessageDispatcher()
DEFAULT_DISPATCHER.register(new SkipWaitingMessageHandler())

self.addEventListener('message', (messageEvent) => {
  const promises = []
  for (const handler of DEFAULT_DISPATCHER.handlers((candidate) =>
    candidate.canExecute(messageEvent)
  )) {
    const maybePromise = handler.execute(messageEvent)
    if (maybePromise != null) {
      promises.push(maybePromise)
    }
  }

  if (promises.length) {
    messageEvent.waitUntil(Promise.allSettled(promises))
  }
})

export default DEFAULT_DISPATCHER
