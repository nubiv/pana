import { writable, type Writable } from 'svelte/store'

type TLLMState = {
  isRunning: boolean
  runnningModel: string | null
  localModels: TLocalModels
}

type TModel = {
  name: string
  isDownloaded: boolean
  downloadProgress: number
}

type TLocalModels = TModel[]

const initialState: TLLMState = {
  isRunning: false,
  runnningModel: null,
  localModels: []
}

export const LLMState: Writable<TLLMState> = writable(initialState)
