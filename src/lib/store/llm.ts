import { writable, type Writable } from 'svelte/store'

type TLLMState = {
  runnningModel: string | null
  localModels: Record<string, TModel>
  otherModels: Record<string, TModel>
}

export type TModel = {
  name: string
  size: number
  totalSize: number
}

const initialState: TLLMState = {
  runnningModel: null,
  localModels: {},
  otherModels: {}
}

export const LLMState: Writable<TLLMState> = writable(initialState)
