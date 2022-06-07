import { quadIn } from 'svelte/easing';
export function selectMark(node: HTMLElement, { delay = 0, duration = 500 }) {
  node.scrollIntoView({
    behavior: 'smooth',
    block: 'start',
    inline: 'nearest',
  });
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
      return `
        transform: translateX(${(1 - eased) * 100}%);
      `;
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
