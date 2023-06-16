import { writable, type Writable } from 'svelte/store'

export type TMessage = {
  role: string
  text: string
}

type THistoryState = TMessage[]

const initialState: THistoryState = []

export const HistoryState: Writable<THistoryState> = writable(initialState)

type TStreamState = {
  isStreaming: boolean
  tokens: string
}

const initialStreamState: TStreamState = {
  isStreaming: false,
  tokens: ''
}

export const StreamState: Writable<TStreamState> = writable(initialStreamState)
