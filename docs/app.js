import init, { lint_workbook, get_workbook_stats, get_rules_definition, init_panic_hook } from './pkg/sheetrs_wasm.js';

let wasmReady = false;
let currentTool = 'lint';
let currentFileData = null;
let currentFileName = '';
let rulesMetadata = [];

async function run() {
    try {
        await init();
        init_panic_hook();
        wasmReady = true;
        updateStatus('WASM Loaded');

        rulesMetadata = get_rules_definition();
        buildRulesUI();
    } catch (e) {
        console.error("Initialization error:", e);
        updateStatus('Initialization Error');
    }
}

const fileZone = document.getElementById('file-zone');
const fileInput = document.getElementById('file-input');
const output = document.getElementById('output');
const statusText = document.getElementById('status');
const toolBtns = document.querySelectorAll('.tool-btn');
const playBtn = document.getElementById('play-btn');
const copyBtn = document.getElementById('copy-btn');
const downloadBtn = document.getElementById('download-btn');
const clearBtn = document.getElementById('clear-btn');
const fileInfo = document.getElementById('file-info');
const rulesAccordion = document.getElementById('rules-accordion');

const lintSetup = document.getElementById('lint-setup');
const statsSetup = document.getElementById('stats-setup');

// Tool Selection
toolBtns.forEach(btn => {
    btn.addEventListener('click', () => {
        toolBtns.forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        currentTool = btn.dataset.tool;

        // Switch setup panels
        lintSetup.style.display = currentTool === 'lint' ? 'block' : 'none';
        statsSetup.style.display = currentTool === 'stats' ? 'block' : 'none';
    });
});

// Build dynamic Rules UI
function buildRulesUI() {
    if (!rulesMetadata || !Array.isArray(rulesMetadata)) return;

    rulesAccordion.innerHTML = '';

    // Group by category
    const categories = {};
    rulesMetadata.forEach(rule => {
        if (!categories[rule.category]) categories[rule.category] = [];
        categories[rule.category].push(rule);
    });

    Object.keys(categories).sort().forEach(cat => {
        const catDiv = document.createElement('div');
        catDiv.className = 'accordion-category';

        const header = document.createElement('div');
        header.className = 'category-header';
        header.innerHTML = `<span>${cat}</span> <span>▾</span>`;

        const content = document.createElement('div');
        content.className = 'category-content';

        categories[cat].forEach(rule => {
            const item = document.createElement('div');
            item.className = 'rule-item';
            item.innerHTML = `
                <input type="checkbox" id="rule-${rule.id}" data-id="${rule.id}" ${rule.is_default ? 'checked' : ''}>
                <label for="rule-${rule.id}" class="rule-name" title="${rule.id}">${rule.name}</label>
            `;
            content.appendChild(item);
        });

        header.addEventListener('click', () => {
            content.classList.toggle('active');
            header.querySelector('span:last-child').textContent = content.classList.contains('active') ? '▴' : '▾';
        });

        catDiv.appendChild(header);
        catDiv.appendChild(content);
        rulesAccordion.appendChild(catDiv);
    });
}

// File Handling
fileZone.addEventListener('click', () => fileInput.click());
fileZone.addEventListener('dragover', (e) => {
    e.preventDefault();
    fileZone.classList.add('dragover');
});
fileZone.addEventListener('dragleave', () => fileZone.classList.remove('dragover'));
fileZone.addEventListener('drop', (e) => {
    e.preventDefault();
    fileZone.classList.remove('dragover');
    if (e.dataTransfer.files.length > 0) {
        handleFile(e.dataTransfer.files[0]);
    }
});

fileInput.addEventListener('change', (e) => {
    if (e.target.files.length > 0) {
        handleFile(e.target.files[0]);
    }
});

function handleFile(file) {
    try {
        currentFileName = file.name;
        fileInfo.textContent = `File: ${file.name} (${formatSize(file.size)})`;

        const reader = new FileReader();
        reader.onload = (e) => {
            currentFileData = new Uint8Array(e.target.result);
            if (playBtn) {
                playBtn.disabled = false;
            }
            updateStatus('File loaded and ready');
        };
        reader.onerror = (err) => {
            console.error("FileReader error:", err);
            updateStatus('File Read Error');
        };
        reader.readAsArrayBuffer(file);
    } catch (err) {
        console.error("Error in handleFile:", err);
    }
}

// Processing
playBtn.addEventListener('click', processFile);

function processFile() {
    if (!wasmReady || !currentFileData) return;

    const extension = currentFileName.split('.').pop().toLowerCase();

    // Clear and show processing line
    output.textContent = `Processing ${currentFileName}...\n`;
    updateStatus('Running...');

    const startTime = performance.now();

    try {
        let result;
        if (currentTool === 'lint') {
            const configToml = buildConfigToml();
            result = lint_workbook(currentFileData, extension, configToml);
        } else if (currentTool === 'stats') {
            result = get_workbook_stats(currentFileData, extension);
        }

        const endTime = performance.now();
        const duration = (endTime - startTime).toFixed(2);

        output.textContent += JSON.stringify(result, null, 2);
        output.textContent += `\n\n----------------------\nFinished in ${duration}ms`;
        updateStatus('Success');
    } catch (e) {
        output.textContent += `\nError: ${e}`;
        updateStatus('Failure');
        console.error("Processing error:", e);
    }
}

function buildConfigToml() {
    const enabled = [];
    const disabled = [];

    const checkboxes = rulesAccordion.querySelectorAll('input[type="checkbox"]');
    checkboxes.forEach(cb => {
        if (cb.checked) {
            enabled.push(cb.dataset.id);
        } else {
            disabled.push(cb.dataset.id);
        }
    });

    return `[global]\nenabled_rules = ${JSON.stringify(enabled)}\ndisabled_rules = ${JSON.stringify(disabled)}`;
}

// Utility Actions
copyBtn.addEventListener('click', () => {
    navigator.clipboard.writeText(output.textContent);
    updateStatus('Copied to clipboard');
});

downloadBtn.addEventListener('click', () => {
    const blob = new Blob([output.textContent], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${currentFileName}_${currentTool}.json`;
    a.click();
    URL.revokeObjectURL(url);
});

clearBtn.addEventListener('click', () => {
    output.textContent = '';
    updateStatus('Cleared');
});

function updateStatus(msg) {
    statusText.textContent = msg;
}

function formatSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

run();
