'use strict';
const { invoke } = window.__TAURI__.core;

/* ── data generation ── */
const SERIES_LABELS = ['Overall', 'North', 'South', 'East', 'West', 'Central'];
const SERIES_COLORS = [
    'var(--series-0)', 'var(--series-1)', 'var(--series-2)',
    'var(--series-3)', 'var(--series-4)', 'var(--series-5)',
];

const VALUE_THRESHOLDS = [
    { min: 0,   max: 50,       color: '#30b94d', name: 'Green' },
    { min: 50,  max: 100,      color: '#a4cc35', name: 'Lime' },
    { min: 100, max: 200,      color: '#e8c820', name: 'Yellow' },
    { min: 200, max: 300,      color: '#ff9500', name: 'Orange' },
    { min: 300, max: 400,      color: '#ff3b30', name: 'Red' },
    { min: 400, max: Infinity,  color: '#8b4513', name: 'Brown' },
];

function valueColor(v) {
    for (const t of VALUE_THRESHOLDS) {
        if (v <= t.max) return t.color;
    }
    return VALUE_THRESHOLDS.at(-1).color;
}

let DATA;
//let DATA = generateData();

function generateData() {
    const base = new Date('2026-04-26T00:00:00');
    const rows = [];
    const vals = [120, 180, 80, 250, 350, 420];
    for (let i = 0; i < 30; i++) {
        const row = { timestamp: new Date(base.getTime() + i * 3600_000).toISOString().slice(0,-1) };
        for (let s = 0; s < SERIES_LABELS.length; s++) {
            vals[s] = Math.max(0, vals[s] + (Math.random() - .48 + s * .005) * 40);
            row[SERIES_LABELS[s]] = parseFloat(vals[s].toFixed(1));
        }
        rows.push(row);
    }
    return rows;
}

// null = all
let activeSeries = null;

/* ── SVG helpers ── */
const SVG_NS = 'http://www.w3.org/2000/svg';

function svgEl(tag, attrs = {}) {
    const el = document.createElementNS(SVG_NS, tag);
    for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, v);
    return el;
}

function svgText(content, attrs = {}) {
    const el = svgEl('text', attrs);
    el.textContent = content;
    return el;
}

/* ── chart constants ── */
const VB = { w: 800, h: 400, padT: 20, padR: 20, padB: 65, padL: 58 };
const plotW = VB.w - VB.padL - VB.padR;
const plotH = VB.h - VB.padT - VB.padB;

const tooltip = document.getElementById('tooltip');

function niceTicks(min, max, count = 5) {
    const range = max - min || 1;
    const step = Math.pow(10, Math.floor(Math.log10(range / count)));
    const candidates = [step, step * 2, step * 2.5, step * 5, step * 10];
    const interval = candidates.find(c => range / c <= count) || candidates.at(-1);
    const lo = Math.floor(min / interval) * interval;
    const ticks = [];
    for (let t = lo; t <= max || ticks.length === 0; t += interval) {
        ticks.push(parseFloat(t.toFixed(10)));
    }
    if (ticks[ticks.length - 1] < max) {
        ticks.push(parseFloat((ticks[ticks.length - 1] + interval).toFixed(10)));
    }
    return ticks;
}

/* ── build chart ── */
function buildChart() {
    const svg = document.getElementById('chart');
    while (svg.firstChild) svg.removeChild(svg.firstChild);

    const showAll = activeSeries === null;
    const visible = showAll
        ? SERIES_LABELS.map((_, i) => i)
        : [activeSeries];

    const times = DATA.map(r => new Date(r.timestamp).getTime());
    const tMin = times[0], tMax = times[times.length - 1];

    let vMax = -Infinity;
    for (const row of DATA) {
        for (const key of SERIES_LABELS) {
            const v = row[key];
            if (v > vMax) vMax = v;
        }
    }

    const vTicks = niceTicks(0, vMax);
    const vLo = vTicks[0], vHi = vTicks[vTicks.length - 1];

    const xScale = t => VB.padL + ((t - tMin) / (tMax - tMin)) * plotW;
    const yScale = v => VB.padT + plotH - ((v - vLo) / (vHi - vLo)) * plotH;

    for (const t of vTicks) {
        const y = yScale(t);
        svg.appendChild(svgEl('line', {
            class: 'grid-line',
            x1: VB.padL, y1: y, x2: VB.padL + plotW, y2: y,
        }));
        svg.appendChild(svgText(String(t), {
            class: 'axis-label',
            x: VB.padL - 6, y: y + 4,
            'text-anchor': 'end',
        }));
    }

    const xStep = Math.ceil(DATA.length / 6);
    const lastIdx = DATA.length - 1;
    const xTickIndices = [];
    for (let i = 0; i < DATA.length; i += xStep) xTickIndices.push(i);
    if (xTickIndices[xTickIndices.length - 1] !== lastIdx) xTickIndices.push(lastIdx);

    for (const i of xTickIndices) {
        const x = xScale(times[i]);
        const dt = new Date(DATA[i].timestamp);
        const label = i === lastIdx
            ? 'Now'
            : dt.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
        svg.appendChild(svgText(label, {
            class: 'axis-label',
            x, y: VB.padT + plotH + 16,
            'text-anchor': 'middle',
        }));
        svg.appendChild(svgEl('line', {
            class: 'grid-line',
            x1: x, y1: VB.padT, x2: x, y2: VB.padT + plotH,
            'stroke-dasharray': '0',
            'stroke-opacity': '.4',
        }));
    }

    // axis titles
    svg.appendChild(svgText('Time', {
        class: 'axis-title',
        x: VB.padL + plotW / 2,
        y: VB.h - 4,
        'text-anchor': 'middle',
    }));
    svg.appendChild(svgText('Value', {
        class: 'axis-title',
        x: 0, y: 0,
        'text-anchor': 'middle',
        transform: `translate(14, ${VB.padT + plotH / 2}) rotate(-90)`,
    }));

    // draw background series first (B-F when "All"), then primary (A) on top
    const drawOrder = showAll
        ? [...visible.filter(i => i !== 0), 0]
        : [...visible];

    for (const si of drawOrder) {
        const key = SERIES_LABELS[si];
        const color = SERIES_COLORS[si];
        const values = DATA.map(r => r[key]);
        const isPrimary = !showAll || si === 0;

        const areaPoints = [
            `${xScale(times[0])},${VB.padT + plotH}`,
            ...times.map((t, i) => `${xScale(t)},${yScale(values[i])}`),
            `${xScale(times[times.length - 1])},${VB.padT + plotH}`,
        ].join(' ');

        if (isPrimary) {
            svg.appendChild(svgEl('polygon', {
                class: 'chart-area',
                points: areaPoints,
                fill: color,
            }));
        }

        const d = times.map((t, i) =>
            `${i === 0 ? 'M' : 'L'}${xScale(t)},${yScale(values[i])}`
        ).join(' ');

        const lineAttrs = {
            class: 'chart-line',
            d,
            stroke: color,
            'stroke-width': isPrimary ? '2.5' : '1.2',
            'stroke-opacity': isPrimary ? '1' : '.25',
        };
        if (!isPrimary) lineAttrs['stroke-dasharray'] = '6 4';

        svg.appendChild(svgEl('path', lineAttrs));
    }

    const crosshairGroup = svgEl('g', { visibility: 'hidden' });
    crosshairGroup.appendChild(svgEl('line', {
        class: 'crosshair-line',
        id: 'ch-x', x1: 0, y1: VB.padT, x2: 0, y2: VB.padT + plotH,
    }));

    const dots = visible.map(si => {
        const dot = svgEl('circle', {
            class: 'dot-hover', r: 5, fill: SERIES_COLORS[si],
        });
        crosshairGroup.appendChild(dot);
        return { si, dot };
    });

    svg.appendChild(crosshairGroup);

    const overlayEl = svgEl('rect', {
        x: VB.padL, y: VB.padT,
        width: plotW, height: plotH,
        fill: 'transparent',
    });

    overlayEl.addEventListener('mousemove', ev => {
        const rect = svg.getBoundingClientRect();
        const scaleX = VB.w / rect.width;
        const svgX = (ev.clientX - rect.left) * scaleX;
        const pctX = Math.max(0, Math.min(1, (svgX - VB.padL) / plotW));
        const idx = Math.min(DATA.length - 1, Math.round(pctX * (DATA.length - 1)));

        const cx = xScale(times[idx]);
        crosshairGroup.setAttribute('visibility', 'visible');
        crosshairGroup.querySelector('#ch-x').setAttribute('x1', cx);
        crosshairGroup.querySelector('#ch-x').setAttribute('x2', cx);

        for (const { si, dot } of dots) {
            const cy = yScale(DATA[idx][SERIES_LABELS[si]]);
            dot.setAttribute('cx', cx);
            dot.setAttribute('cy', cy);
        }

        const row = DATA[idx];
        const dt = new Date(row.timestamp);
        const dateStr = dt.toLocaleString([], {
            month: 'short', day: 'numeric',
            hour: '2-digit', minute: '2-digit',
        });

        let html = `<div class="tt-ts">${dateStr}</div>`;
        for (const si of visible) {
            const key = SERIES_LABELS[si];
            html += `<div class="tt-row">
        <span class="tt-swatch" style="background:${SERIES_COLORS[si]}"></span>
        <span class="tt-label">${key}:</span>
        <span class="tt-value">${row[key]}</span>
      </div>`;
        }
        tooltip.innerHTML = html;
        tooltip.style.left = (ev.clientX - rect.left + 16) + 'px';
        tooltip.style.top = (ev.clientY - rect.top - 10) + 'px';
        tooltip.classList.remove('hidden');
    });

    overlayEl.addEventListener('mouseleave', () => {
        crosshairGroup.setAttribute('visibility', 'hidden');
        tooltip.classList.add('hidden');
    });

    svg.appendChild(overlayEl);
}

/* ── table (newest first) ── */
function buildTable() {
    const table = document.getElementById('data-table');
    while (table.firstChild) table.removeChild(table.firstChild);

    const thead = document.createElement('thead');
    const headRow = document.createElement('tr');

    const thTs = document.createElement('th');
    thTs.textContent = 'Timestamp';
    headRow.appendChild(thTs);

    for (let si = 0; si < SERIES_LABELS.length; si++) {
        const th = document.createElement('th');
        th.textContent = SERIES_LABELS[si];
        headRow.appendChild(th);
    }
    thead.appendChild(headRow);
    table.appendChild(thead);

    const tbody = document.createElement('tbody');
    const sorted = [...DATA].reverse();

    for (const row of sorted) {
        const tr = document.createElement('tr');

        const tdTs = document.createElement('td');
        tdTs.className = 'col-ts';
        const dt = new Date(row.timestamp);
        tdTs.textContent = dt.toLocaleString([], {
            month: 'short', day: 'numeric',
            hour: '2-digit', minute: '2-digit',
        });
        tr.appendChild(tdTs);

        for (const key of SERIES_LABELS) {
            const td = document.createElement('td');
            td.textContent = row[key];
            td.style.color = valueColor(row[key]);
            tr.appendChild(td);
        }
        tbody.appendChild(tr);
    }

    table.appendChild(tbody);
}

/* ── series selector ── */
function buildSeriesSelector() {
    const container = document.getElementById('series-selector');
    while (container.firstChild) container.removeChild(container.firstChild);

    const allBtn = document.createElement('button');
    allBtn.type = 'button';
    allBtn.dataset.idx = 'all';
    allBtn.classList.add('series-btn');
    if (activeSeries === null) allBtn.classList.add('active');
    allBtn.textContent = 'All';
    allBtn.addEventListener('click', () => {
        activeSeries = null;
        refreshChart();
    });
    container.appendChild(allBtn);

    SERIES_LABELS.forEach((k, i) => {
        const btn = document.createElement('button');
        btn.type = 'button';
        btn.dataset.idx = i;
        btn.classList.add('series-btn');
        if (activeSeries === i) btn.classList.add('active');
        btn.textContent = k;
        btn.addEventListener('click', () => {
            activeSeries = i;
            refreshChart();
        });
        container.appendChild(btn);
    });
}

function refreshChart() {
    buildSeriesSelector();
    buildChart();
}

/* ── summary view ── */
function buildSummary() {
    const container = document.getElementById('summary-content');
    while (container.firstChild) container.removeChild(container.firstChild);

    const latest = DATA[DATA.length - 1];
    const dt = new Date(latest.timestamp);
    const dateStr = dt.toLocaleString([], {
        month: 'short', day: 'numeric',
        hour: '2-digit', minute: '2-digit', second: '2-digit',
    });

    document.getElementById('summary-title').textContent = `Latest Reading — ${dateStr}`;

    SERIES_LABELS.forEach((k, i) => {
        const card = document.createElement('div');
        card.className = 'summary-card';

        const label = document.createElement('div');
        label.className = 'summary-label';
        label.textContent = k;
        label.style.color = 'var(--text-sub)';
        card.appendChild(label);

        const val = document.createElement('div');
        val.className = 'summary-value';
        val.textContent = latest[k];
        val.style.color = valueColor(latest[k]);
        card.appendChild(val);

        container.appendChild(card);
    });
}

function buildLegend() {
    const table = document.getElementById('legend-table');
    while (table.firstChild) table.removeChild(table.firstChild);

    const thead = document.createElement('thead');
    const headRow = document.createElement('tr');
    for (const h of ['Color', 'Name', 'Range']) {
        const th = document.createElement('th');
        th.textContent = h;
        headRow.appendChild(th);
    }
    thead.appendChild(headRow);
    table.appendChild(thead);

    const tbody = document.createElement('tbody');
    for (const t of VALUE_THRESHOLDS) {
        const tr = document.createElement('tr');

        const tdSwatch = document.createElement('td');
        const swatch = document.createElement('span');
        swatch.style.cssText = `display:inline-block;width:14px;height:14px;border-radius:3px;background:${t.color};vertical-align:middle`;
        tdSwatch.appendChild(swatch);
        tr.appendChild(tdSwatch);

        const tdName = document.createElement('td');
        tdName.textContent = t.name;
        tdName.style.color = t.color;
        tdName.style.fontWeight = '600';
        tr.appendChild(tdName);

        const tdRange = document.createElement('td');
        tdRange.textContent = t.max === Infinity
            ? `> ${t.min}`
            : `${t.min} – ${t.max}`;
        tr.appendChild(tdRange);

        tbody.appendChild(tr);
    }
    table.appendChild(tbody);
}

/* ── tabs ── */
document.querySelectorAll('.tab-btn').forEach(btn => {
    btn.addEventListener('click', () => {
        document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');

        const tab = btn.dataset.tab;
        document.getElementById('view-chart').classList.toggle('hidden', tab !== 'chart');
        document.getElementById('view-table').classList.toggle('hidden', tab !== 'table');
        document.getElementById('view-summary').classList.toggle('hidden', tab !== 'summary');
    });
});

/* ── theme toggle ── */
(function initTheme() {
    const root = document.documentElement;
    const saved = localStorage.getItem('tsv-theme');
    if (saved) root.dataset.theme = saved;

    document.getElementById('theme-toggle').addEventListener('click', () => {
        const next = root.dataset.theme === 'dark' ? 'light' : 'dark';
        root.dataset.theme = next;
        localStorage.setItem('tsv-theme', next);
    });
})();

/* ── regenerate data ── */
document.getElementById('regen-btn').addEventListener('click', () => {
    refreshAll();
});

/* ── auto-refresh interval (ms) ── */
const REFRESH_INTERVAL = 60*1000; // Refresh data every minute

async function refreshAll() {
    await update();
    refreshChart();
    buildTable();
    buildSummary();
}


async function update() {
    DATA = await invoke("update");
}

window.addEventListener("DOMContentLoaded", () => {
    /* ── init ── */
    buildLegend();
    refreshAll();
    setInterval(refreshAll, REFRESH_INTERVAL);
});
