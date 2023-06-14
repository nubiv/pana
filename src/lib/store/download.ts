import { writable, type Writable } from 'svelte/store'

type TDownloadState = {
  currentDownload: string | null
  progress: number
}

const initialState: TDownloadState = {
  currentDownload: null,
  progress: 0
}

export const DownloadState: Writable<TDownloadState> = writable(initialState)
