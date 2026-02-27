# My HP — Personal Website & Blog Platform

A production-ready personal website and blog platform built with **SvelteKit** (frontend) and **Rust/Axum** (backend), deployable on AWS EC2 with Docker Compose.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                         EC2 Instance                     │
│  ┌─────────┐   ┌─────────────┐   ┌────────────────────┐ │
│  │ nginx   │──▶│  SvelteKit  │   │   Rust/Axum API    │ │
│  │ (HTTPS) │   │  (SSR)      │──▶│   (port 3000)      │ │
│  │         │   │  (port 3001)│   │                    │ │
│  └─────────┘   └─────────────┘   └─────────┬──────────┘ │
│                                             │            │
│                                   ┌─────────▼──────────┐ │
│                                   │  MySQL 8           │ │
│                                   │  (Docker volume)   │ │
│                                   └────────────────────┘ │
└─────────────────────────────────────────────────────────┘
                                  │
                          ┌───────▼────────┐
                          │   AWS S3       │
                          │ (image store)  │
                          └────────────────┘
```

## Tech Stack

| Layer      | Technology                               |
|------------|------------------------------------------|
| Frontend   | SvelteKit, TailwindCSS, adapter-node     |
| Backend    | Rust, Axum, sqlx (MySQL)                 |
| Database   | MySQL 8                                  |
| Auth       | Session-based (HttpOnly cookies, bcrypt) |
| Markdown   | pulldown-cmark + ammonia (sanitized)     |
| Images     | AWS S3 (private bucket)                  |
| Infra      | Docker Compose, EC2, nginx, Let's Encrypt|
| CI/CD      | GitHub Actions                           |

## Security Design

- **Passwords**: bcrypt with cost >= 12
- **Sessions**: UUID stored in DB, HttpOnly + SameSite=Lax cookies
- **HTML**: Markdown rendered server-side, sanitized with `ammonia`
- **SQL**: sqlx parameterized queries (no injection risk)
- **CORS**: Origin-restricted in production
- **Secrets**: Environment variables only, never committed

## Local Development

### Prerequisites
- Docker & Docker Compose
- Rust (for local backend dev)
- Node.js 20+ (for local frontend dev)

### Quick Start

```bash
cp .env.example .env
# Edit .env with your values

docker compose up -d --build

# Frontend: http://localhost:3001
# Backend:  http://localhost:3000
```

### Backend Only

```bash
cd backend
export DATABASE_URL="mysql://myhp:password@127.0.0.1:3306/myhp"
export SESSION_SECRET="your-secret-here"
cargo run
```

### Frontend Only

```bash
cd frontend
cp .env.example .env
npm install
npm run dev
```

## Database Migrations

Migrations in `backend/migrations/` are applied automatically when MySQL starts.

## API Endpoints

### Public
- `GET /health`
- `GET /articles`, `GET /articles/:slug`
- `GET /books`, `GET /books/:slug`

### Authentication
- `POST /login`, `POST /logout`

### Admin (session required)
- `POST /admin/articles`, `PUT /admin/articles/:id`, `DELETE /admin/articles/:id`
- `POST /admin/books`, `PUT /admin/books/:id`, `DELETE /admin/books/:id`
- `POST /admin/upload-image`

## AWS Deployment

### Required GitHub Secrets
- `EC2_SSH_KEY` — SSH private key
- `EC2_HOST` — EC2 hostname/IP
- `EC2_USER` — SSH user (e.g. `ubuntu`)

## Project Structure

```
.
├── backend/              # Rust/Axum API server
│   ├── src/
│   │   ├── main.rs       # App bootstrap, router
│   │   ├── config.rs     # Config from env vars
│   │   ├── db.rs         # DB pool creation
│   │   ├── models/       # Database models
│   │   ├── routes/       # Route handlers
│   │   ├── middleware/   # Auth middleware
│   │   ├── auth/         # Session management
│   │   └── utils/        # Markdown processing
│   ├── migrations/       # SQL migration files
│   └── Dockerfile        # Multi-stage build
├── frontend/             # SvelteKit app
│   ├── src/
│   │   ├── lib/          # Shared utilities (API client)
│   │   └── routes/       # SvelteKit file-based routes
│   └── Dockerfile        # Multi-stage build
├── docker-compose.yml    # Full stack orchestration
├── .env.example          # Environment variable template
└── .github/
    └── workflows/
        └── deploy.yml    # CI/CD pipeline
```

## Extensibility

Future additions supported by the architecture:
- Paid articles (tier column on articles + membership middleware)
- Membership tiers (memberships table)
- Comments (comments table with article_id FK)
- Stripe integration (/webhook endpoint)
- Multi-user support (user_id FKs on content tables)
