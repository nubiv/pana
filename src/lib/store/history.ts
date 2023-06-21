import { writable, type Writable } from 'svelte/store'

export type TMessage = {
  role: string
  text: string
}

type THistoryState = TMessage[]

const initialHistoryState: THistoryState = []

type TStreamState = {
  isStreaming: boolean
  isFeedingPrompt: boolean
  tokens: string
}

const initialStreamState: TStreamState = {
  isStreaming: false,
  isFeedingPrompt: false,
  tokens: ''
}

const createHistoryStore = () => {
  const { subscribe, update } = writable(initialHistoryState)

  const addMessage = (message: TMessage) => {
    update((prev) => {
      return [...prev, message]
    })
  }

  return {
    subscribe,
    addMessage
  }
}

export const HistoryState = createHistoryStore()

const createStreamStore = () => {
  const { subscribe, update } = writable(initialStreamState)

  const feedPrompt = () => {
    update((prev) => {
      return {
        ...prev,
        isFeedingPrompt: true
      }
    })
  }

  const startStream = () => {
    update((prev) => {
      return {
        ...prev,
        isFeedingPrompt: false,
        isStreaming: true
      }
    })
  }

  const stopStream = () => {
    update((prev) => {
      const newMessage: TMessage = {
        text: prev.tokens,
        role: 'Pana'
      }

      HistoryState.addMessage(newMessage)

      return {
        ...prev,
        isStreaming: false,
        tokens: ''
      }
    })
  }

  const setTokens = (token: string) => {
    update((prev) => {
      return {
        ...prev,
        tokens: prev.tokens.concat(token)
      }
    })
  }

  return {
    subscribe,
    feedPrompt,
    startStream,
    stopStream,
    setTokens
  }
}

export const StreamState = createStreamStore()
