import DefaultTheme from 'vitepress/theme'
import HeroStats from './components/HeroStats.vue'

export default {
  ...DefaultTheme,
  enhanceApp({ app }) {
    app.component('HeroStats', HeroStats)
  }
}
