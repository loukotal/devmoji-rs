import { defineDevmojiConfig } from '@loukotal/devmoji-rs';

export default defineDevmojiConfig({
  types: ['wip'],
  devmoji: [
    { code: 'fix', emoji: 'saluting_face' },
    { code: 'wip', gitmoji: 'construction', description: 'work in progress' },
  ],
});
