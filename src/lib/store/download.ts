import { writable, type Writable } from 'svelte/store'

type TDownloadState = {
  currentDownload: string | null
  progress: string | null
  totalSize: number | null
}

const initialState: TDownloadState = {
  currentDownload: null,
  progress: null,
  totalSize: null
}

const creatDownloadStateStore = () => {
  const { subscribe, update } = writable(initialState)

  const startDownload = (
    modelName: string,
    progress: string,
    totalSize: number
  ) => {
    update((prev) => {
      return {
        ...prev,
        currentDownload: modelName,
        progress,
        totalSize
      }
    })
  }

  const stopDownload = () => {
    update((prev) => {
      return {
        ...prev,
        currentDownload: null,
        progress: null,
        totalSize: null
      }
    })
  }

  const updateProgress = async (payload: TDownloadPayload) => {
    const { size } = payload

    update((prev) => {
      const totalSize = prev.totalSize

      const progress = ((size / totalSize!) * 100.0).toFixed(2)

      return {
        ...prev,
        progress
      }
    })

    return false
  }

  return {
    subscribe,
    startDownload,
    stopDownload,
    updateProgress
  }
}

export const DownloadState = creatDownloadStateStore()

type TDownloadPayload = {
  size: number
}
