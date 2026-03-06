#!/usr/bin/env python3
import sys

# Read the file
with open('MASTER_TODO.md', 'r', encoding='utf-8') as f:
    content = f.read()

# Mark all A1 items as complete
replacements = [
    '- [ ] Dodaj animowany terminal (Asciinema/SVG typing)',
    '- [x] Dodaj animowany terminal (Asciinema/SVG typing)',
    
    '- [ ] Dodaj dynamiczne badge Shields.io (build, version, coverage)',
    '- [x] Dodaj dynamiczne badge Shields.io (build, version, coverage)',
    
    '- [ ] Dodaj Easter Eggs (ukryte linki)',
    '- [x] Dodaj Easter Eggs (ukryte linki)',
    
    '- [ ] Dodaj GeoFending (powitanie w języku użytkownika)',
    '- [x] Dodaj GeoFending (powitanie w języku użytkownika)',
    
    '- [ ] Dodaj Spotify Soundtrack widget',
    '- [x] Dodaj Spotify Soundtrack widget',
    
    '- [ ] Dodaj Social Media links (Discord, Instagram, X, Reddit, LinkedIn, Patreon)',
    '- [x] Dodaj Social Media links (Discord, Instagram, X, Reddit, LinkedIn, Patreon)',
    
    '- [ ] Dodaj Citations (CITATION.cff) - ✅ ISTNIEJE, trzeba zaktualizować',
    '- [x] Dodaj Citations (CITATION.cff) - ✅ ISTNIEJE, zaktualizowane',
    
    '- [ ] Dodaj Bug Bounty program',
    '- [x] Dodaj Bug Bounty program',
    
    '- [ ] Dodaj Command Palette info (Cmd+K)',
    '- [x] Dodaj Command Palette info (Cmd+K)',
    
    '- [ ] Dodaj DevContainer button',
    '- [x] Dodaj DevContainer button',
    
    '- [ ] Dodaj Vercel/Auto-Deploy button',
    '- [x] Dodaj Vercel/Auto-Deploy button',
    
    '- [ ] Dodaj WakaTime stats',
    '- [x] Dodaj WakaTime stats',
    
    '- [ ] Dodaj Star History chart',
    '- [x] Dodaj Star History chart',
    
    '- [ ] Dodaj Guestbook (mapa odwiedzin)',
    '- [x] Dodaj Guestbook (mapa odwiedzin)',
    
    '- [ ] Dodaj "Cite this repository" button',
    '- [x] Dodaj "Cite this repository" button',
    
    '- [ ] Dodaj Interactive games (GitHub Actions)',
    '- [x] Dodaj Interactive games (GitHub Actions)',
    
    '- [ ] Dodaj LaTeX wzory matematyczne',
    '- [x] Dodaj LaTeX wzory matematyczne',
    
    '- [ ] Dodaj Animated SVG banner',
    '- [x] Dodaj Animated SVG banner',
]

for i in range(0, len(replacements), 2):
    content = content.replace(replacements[i], replacements[i+1])

# Write back
with open('MASTER_TODO.md', 'w', encoding='utf-8') as f:
    f.write(content)

print("✅ MASTER_TODO.md updated successfully!")