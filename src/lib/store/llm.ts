import { writable, type Writable } from 'svelte/store'

type TLLMState = {
  runnningModel: string | null
  localModels: Record<string, TLocalModel>
  otherModels: Record<string, TOtherModel>
}

export type TLocalModel = {
  name: string
  size: number
  totalSize: number
}

export type TOtherModel = {
  name: String
  size: number
  totalSize: number
}

const initialState: TLLMState = {
  runnningModel: null,
  localModels: {},
  otherModels: {}
}

export const LLMState: Writable<TLLMState> = writable(initialState)
