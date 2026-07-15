import DefaultTheme from 'vitepress/theme';
import HeroStats from './components/HeroStats.vue';
import Forum from './components/Forum.vue';
import './style.css';
import pkg from '../../../package.json' with { type: 'json' };
import { h } from 'vue';
import { library } from '@fortawesome/fontawesome-svg-core';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { faArrowUpRightFromSquare } from '@fortawesome/free-solid-svg-icons';
import { EnhanceAppContext } from 'vitepress';

library.add(faArrowUpRightFromSquare);

export default {
  extends: DefaultTheme,
  enhanceApp({ app }: EnhanceAppContext) {
    app.component('HeroStats', HeroStats);
    app.component('Forum', Forum);
    app.component('font-awesome-icon', FontAwesomeIcon);
  },
  Layout() {
    return h(DefaultTheme.Layout, null, {
      'sidebar-nav-before': () =>
        h('div', { class: 'custom-sidebar-top' }, [
          h('div', { class: 'sidebar-meta' }, [
            h('p', { class: 'version-text' }, `v${pkg.version}`),
            h(
              'a',
              {
                href: 'https://github.com/soteenstudio/lightvm/releases',
                target: '_blank',
                class: 'release-link',
              },
              [h(FontAwesomeIcon, { icon: 'arrow-up-right-from-square' })],
            ),
          ]),
        ]),
    });
  },
};
