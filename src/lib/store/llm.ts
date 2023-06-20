import { derived, writable, type Writable } from 'svelte/store'

type TLLMState = {
  runnningModel: string | null
  models: Record<string, TModel>
}

export type TModel = {
  name: string
  size: number
  totalSize: number
}

const initialState: TLLMState = {
  runnningModel: null,
  models: {}
}

const createLLMStateStore = () => {
  const { subscribe, update } = writable(initialState)

  const updateModelList = (payload: TModelPayload) => {
    const { name, size, total_size } = payload

    update((prev) => {
      return {
        ...prev,
        models: {
          ...prev.models,
          [name]: {
            name,
            size,
            totalSize: total_size
          }
        }
      }
    })
  }

  const updateRunningModel = (modelName: string) => {
    update((prev) => {
      return {
        ...prev,
        runnningModel: modelName
      }
    })
  }

  const stopRunningModel = () => {
    update((prev) => {
      return {
        ...prev,
        runnningModel: null
      }
    })
  }

  return {
    subscribe,
    updateModelList,
    updateRunningModel,
    stopRunningModel
  }
}

export const LLMState = createLLMStateStore()

type TModelPayload = {
  name: string
  size: number
  total_size: number
}

type TModelList = Record<string, TModel>

const createLocalModelStore = () => {
  const { subscribe } = derived<typeof LLMState, TModelList>(
    LLMState,
    ($LLMState, set) => {
      const localModels: TModelList = {}

      for (const [key, model] of Object.entries($LLMState.models)) {
        if (model.size === model.totalSize) {
          localModels[key] = model
        }
      }

      set(localModels)
    }
  )

  return { subscribe }
}

export const LocalModels = createLocalModelStore()

const createOtherModelStore = () => {
  const { subscribe } = derived<typeof LLMState, TModelList>(
    LLMState,
    ($LLMState, set) => {
      const otherModels: TModelList = {}

      for (const [key, model] of Object.entries($LLMState.models)) {
        if (model.size !== model.totalSize) {
          otherModels[key] = model
        }
      }

      set(otherModels)
    }
  )

  return { subscribe }
}

export const OtherModels = createOtherModelStore()
