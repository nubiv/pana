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
  const { subscribe, set, update } = writable(initialHistoryState)

  const addMessage = (message: TMessage) => {
    update((prev) => {
      return [...prev, message]
    })
  }

  const syncHistory = (history: Array<number | string>[]) => {
    const newHistory: TMessage[] = []
    history.forEach((kv) => {
      if (kv[0] === 0) {
        const newMessage: TMessage = {
          role: 'Me',
          text: kv[1] as string
        }

        newHistory.push(newMessage)
      } else {
        const newMessage: TMessage = {
          role: 'Pana',
          text: kv[1] as string
        }

        newHistory.push(newMessage)
      }
    })

    update((prev) => {
      return [...prev, ...newHistory]
    })
  }

  const clearHistory = () => {
    set(initialHistoryState)
  }

  return {
    subscribe,
    addMessage,
    syncHistory,
    clearHistory
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
