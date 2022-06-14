import { quadIn } from 'svelte/easing';
import { crossfade } from 'svelte/transition';
export function selectMark(node: HTMLElement, { delay = 0, duration = 500 }) {
  return {
    delay,
    duration,
    css: function (t: number) {
      return `
        display: block;
        opacity: ${Math.cos(t * Math.PI * 4) + 1};
        transform: scale(${t * 2});
      `;
    },
  };
}
export function searchAside(node: HTMLElement, { delay = 0, duration = 300 }) {
  return {
    delay,
    duration,
    css: function (t: number) {
      const eased = quadIn(t);
      if (node.classList.contains('showSearchResults')) {
        return `
        transform: translateX(${(1 - eased) * 100}%);
      `;
      } else {
        return '';
      }
    },
  };
}
export function outlineAside(node: HTMLElement, { delay = 0, duration = 300 }) {
  return {
    delay,
    duration,
    css: function (t: number) {
      const eased = quadIn(t);
      return `
        transform: translateX(${(1 - eased) * -100}%);
      `;
    },
  };
}
export function paraButtons(node: HTMLElement, { delay = 0, duration = 300 }) {
  return {
    delay,
    duration,
    css: function (t: number) {
      const eased = quadIn(t);
      return `
        opacity: ${t};
      `;
    },
  };
}
