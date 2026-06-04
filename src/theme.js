'use strict';

const root = document.documentElement;
const saved = localStorage.getItem('tsv-theme');
if (saved) root.dataset.theme = saved;

