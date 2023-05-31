import { writable } from 'svelte/store'

const createThemeStore = () => {
  const { subscribe, set } = writable('light')

  const toggleDarkMode = () => {
    console.log('dark mode')
    set('dark')
    document.documentElement.classList.add('dark')
  }

  const toggleLightMode = () => {
    console.log('light mode')
    set('light')
    document.documentElement.classList.remove('dark')
  }

  return {
    subscribe,
    toggleDarkMode,
    toggleLightMode
  }
}

export const themeStore = createThemeStore()

// // On page load or when changing themes, best to add inline in `head` to avoid FOUC
// if (
//   localStorage.theme === 'dark' ||
//   (!('theme' in localStorage) &&
//     window.matchMedia('(prefers-color-scheme: dark)').matches)
// ) {
//   document.documentElement.classList.add('dark')
// } else {
//   document.documentElement.classList.remove('dark')
// }

// // Whenever the user explicitly chooses light mode
// localStorage.theme = 'light'

// // Whenever the user explicitly chooses dark mode
// localStorage.theme = 'dark'

// // Whenever the user explicitly chooses to respect the OS preference
// localStorage.removeItem('theme')
