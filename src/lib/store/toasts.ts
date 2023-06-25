import { writable, derived } from 'svelte/store'

const TIMEOUT = 2500

type TToastType = 'notification' | 'error'
export type TToast = {
  id: string
  type: TToastType
  message: string
}
export type TToastState = TToast[]

const createToastStore = () => {
  const _toasts = writable<TToastState>([])

  const update = (type: TToastType, message: string) => {
    _toasts.update((state) => {
      return [...state, { id: generateID(), type, message }]
    })
  }

  const toasts = derived<typeof _toasts, TToastState>(
    _toasts,
    ($_toasts, set) => {
      set($_toasts)
      if ($_toasts.length > 0) {
        const timer = setTimeout(() => {
          _toasts.update((state) => {
            state.shift()
            return state
          })
        }, TIMEOUT)
        return () => {
          clearTimeout(timer)
        }
      }
    }
  )

  const { subscribe } = toasts

  return {
    subscribe,
    notifications: (message: string) => update('notification', message),
    error: (message: string) => update('error', message)
  }
}

export const toasts = createToastStore()

const generateID = (length = 6) => {
  return Math.random()
    .toString(36)
    .substring(2, length + 2)
}
