#!/usr/bin/env python3
"""Build the static site with only Python's standard library and Cargo."""
from pathlib import Path
import html
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / 'dist'

def inline(value):
    value = html.escape(value)
    value = re.sub(r'`([^`]+)`', r'<code>\1</code>', value)
    value = re.sub(r'\*\*([^*]+)\*\*', r'<strong>\1</strong>', value)
    def link(match):
        label, url = match.groups()
        if url.endswith('.md'):
            url = url[:-3] + '.html'
        return f'<a href="{url}">{label}</a>'
    return re.sub(r'\[([^\]]+)\]\(([^)]+)\)', link, value)

def markdown(text):
    lines = text.splitlines()
    out, i = [], 0
    while i < len(lines):
        line = lines[i]
        if not line.strip():
            i += 1
            continue
        if line.startswith('```'):
            code = []
            i += 1
            while i < len(lines) and not lines[i].startswith('```'):
                code.append(lines[i]); i += 1
            out.append('<pre><code>' + html.escape('\n'.join(code)) + '</code></pre>')
            i += 1
        elif line.startswith('#'):
            match = re.match(r'^(#{1,6}) (.*)$', line)
            if not match:
                raise ValueError(f'Unsupported heading: {line}')
            level, title = len(match[1]), match[2]
            out.append(f'<h{level}>{inline(title)}</h{level}>'); i += 1
        elif line.startswith('|'):
            rows = []
            while i < len(lines) and lines[i].startswith('|'):
                row = [c.strip() for c in lines[i].strip('|').split('|')]
                if not all(re.fullmatch(r':?-+:?', c) for c in row):
                    rows.append(row)
                i += 1
            out.append('<div class="table-scroll"><table>')
            for j, row in enumerate(rows):
                tag = 'th' if j == 0 else 'td'
                out.append('<tr>' + ''.join(f'<{tag}>{inline(c)}</{tag}>' for c in row) + '</tr>')
            out.append('</table></div>')
        elif line.startswith('- '):
            out.append('<ul>')
            while i < len(lines) and lines[i].startswith('- '):
                out.append('<li>' + inline(lines[i][2:]) + '</li>'); i += 1
            out.append('</ul>')
        else:
            paragraph = [line]
            i += 1
            while i < len(lines) and lines[i].strip() and not lines[i].startswith(('#', '|', '```', '- ')):
                paragraph.append(lines[i]); i += 1
            out.append('<p>' + inline(' '.join(paragraph)) + '</p>')
    return '\n'.join(out)

subprocess.run(['cargo', 'build', '--locked', '--release', '--lib', '--target', 'wasm32-unknown-unknown'], cwd=ROOT, check=True)
DIST.mkdir(exist_ok=True)
shutil.copytree(ROOT / 'web', DIST, dirs_exist_ok=True)
shutil.copytree(ROOT / 'examples', DIST / 'examples', dirs_exist_ok=True)
shutil.copyfile(ROOT / 'target/wasm32-unknown-unknown/release/qrunap2.wasm', DIST / 'qrunap2.wasm')
(DIST / '.nojekyll').touch()
(DIST / 'docs').mkdir(exist_ok=True)
for path in (ROOT / 'docs').glob('*.md'):
    source = path.read_text()
    title = html.escape(source.splitlines()[0].lstrip('# '))
    french = path.stem.endswith('.fr')
    language = 'fr' if french else 'en'
    home = '../?lang=fr' if french else '../'
    home_label = 'Ouvrir le simulateur' if french else 'Open the simulator'
    page = f'''<!doctype html><html lang="{language}"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{title} · QRUNAP2</title><link rel="stylesheet" href="../style.css"></head><body>
<header><a class="brand" href="{home}">Q<span>RUN</span>AP2</a><nav><a href="{home}">{home_label} ↗</a><a href="https://github.com/Medialoco/QRUNAP2">GitHub ↗</a></nav></header>
<main class="doc"><nav class="doc-nav" aria-label="Documentation"><a href="guide.html">English guide</a><a href="guide.fr.html">Guide français</a><a href="compatibility.html">Compatibility</a><a href="sources.html">Sources</a><a href="language-inventory.html">Inventory</a><a href="grammar.html">Grammar</a><a href="semantics.html">Semantics</a></nav>{markdown(source)}</main></body></html>'''
    (DIST / 'docs' / (path.stem + '.html')).write_text(page)
    shutil.copyfile(path, DIST / 'docs' / path.name)
print(f'Static site ready: {DIST}')
