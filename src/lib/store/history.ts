import { writable, type Writable } from 'svelte/store'

export type TMessage = {
  role: string
  text: string
}

type THistoryState = TMessage[]

const initialHistoryState: THistoryState = []

type TStreamState = {
  isStreaming: boolean
  tokens: string
}

const initialStreamState: TStreamState = {
  isStreaming: false,
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

  const startStream = () => {
    update((prev) => {
      return {
        ...prev,
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
    startStream,
    stopStream,
    setTokens
  }
}

export const StreamState = createStreamStore()
