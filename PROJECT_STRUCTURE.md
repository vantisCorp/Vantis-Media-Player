# 🏗️ Vantis Media Player - Struktura Projektu (Feature-Sliced Design)

## 📁 Root Layout
```
vantis-media-player/
├── .github/                    # GitHub Actions & Workflows
│   ├── workflows/              # CI/CD Pipelines
│   ├── ISSUE_TEMPLATE/         # Formularze zgłoszeń (YAML)
│   └── PULL_REQUEST_TEMPLATE/  # Szablony PR
│
├── apps/                       # Aplikacje (Monorepo - Turborepo)
│   ├── cli/                    # Command Line Interface
│   ├── desktop/                # Electron/Tauri Desktop App
│   ├── web/                    # Web App (Next.js)
│   └── mobile/                 # Flutter Mobile App
│
├── packages/                   # Udostępnione paczki
│   ├── core/                   # Główna logika
│   ├── ui/                     # UI Components
│   ├── utils/                  # Narzędzia
│   ├── types/                  # TypeScript Types
│   └── config/                 # Konfiguracje
│
├── docs/                       # Dokumentacja (Docusaurus)
│   ├── docs/                   # Strony dokumentacji
│   ├── blog/                   # Blog posty
│   ├── i18n/                   # Tłumaczenia (I18n)
│   └── static/                 # Statyczne assety
│
├── scripts/                    # Narzędzia i skrypty
│   ├── setup.sh                # Inicjalizacja projektu
│   ├── build.sh                # Budowanie
│   ├── test.sh                 # Testy
│   └── release.sh              # Automatyczne releasy
│
├── tools/                      # Narzędzia developerskie
│   ├── devcontainer/           # GitHub Codespaces config
│   ├── ai-agents/              # AI Code Generation Agents
│   └── chaos-monkey/           # Chaos Engineering
│
├── infra/                      # Infrastructure as Code
│   ├── terraform/              # Konfiguracja chmury
│   ├── docker/                 # Docker Compose
│   └── kubernetes/             # K8s manifests
│
├── assets/                     # Assety (Zero Trust)
│   ├── logos/                  # SVG Logotypy
│   ├── banners/                # Bannery i grafiki
│   └── fonts/                  # Fonty
│
├── .devcontainer/              # DevContainer config
├── .github/workflows/          # GitHub Actions
├── turbo.json                 # Turborepo config
├── package.json               # Root package.json
├── pnpm-workspace.yaml        # PNPM workspace
├── EditorConfig               # EditorConfig
├── .gitignore                 # Git ignore
├── .gitattributes             # Git attributes
├── .prettierrc                # Prettier config
├── .eslintrc                  # ESLint config
├── LICENSE                    # AGPLv3 + Commercial
├── SECURITY.md                # Security Policy
├── CONTRIBUTING.md            # Contributing guide
├── README.md                  # Main README
├── CHANGELOG.md               # Changelog
└── CITATION.cff              # Citation file
```

## 🎯 Feature-Sliced Design (FSD)

Struktura kodu według funkcji biznesowych:

```
apps/desktop/src/
├── app/                      # Entry point
├── pages/                    # Strony
├── widgets/                  # Widżety UI
├── features/                 # Funkcje biznesowe
│   ├── auth/                # Autoryzacja
│   ├── playback/            # Odtwarzanie
│   ├── playlist/            # Playlista
│   ├── settings/            # Ustawienia
│   └── library/             # Biblioteka
├── entities/                # Encje biznesowe
│   ├── media/              # Media
│   ├── user/                # Użytkownik
│   └── sync/                # Synchronizacja
├── shared/                  # Wspólne elementy
│   ├── ui/                 # UI Components
│   ├── config/             # Konfiguracja
│   └── api/                # API Client
└── processes/               # Procesy systemowe
```

## 🔐 Zero Trust Architecture

Każda warstwa jest izolowana:
- **Network**: Firewall, WAF, DDoS Protection
- **Application**: Input validation, Rate limiting
- **Data**: Encryption at rest & in transit
- **Infrastructure**: IaC, Immutable deployments
