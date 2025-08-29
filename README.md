# A Monorepo with Next.js as forntend and rust as backend

A full-stack todo application built with Rust (Axum + Diesel) backend and Next.js frontend.

## 🏗️ Project Structure

```
bun_nextjs_rust/
├── backend/api/          # Rust API server (Axum + Diesel + PostgreSQL)
├── web/client/           # Next.js frontend (React + TypeScript + Tailwind)
├── package.json          # Monorepo scripts and dependencies
└── README.md            # This file
```

## 🚀 Quick Start

### Prerequisites

- **Bun** (latest version)
- **Rust** (latest stable)
- **PostgreSQL** (running locally)
- **Diesel CLI**: `cargo install diesel_cli --no-default-features --features postgres`

### 1. Clone and Setup

```bash
git clone https://github.com/aditya-raj-panjiyara/bun_nextjs_rust
cd bun_nextjs_rust

# Install all dependencies
bun run setup
```

### 2. Database Setup

```bash
# Make sure PostgreSQL is running
# Create database (if not exists)
createdb test123

# Run database migrations
bun run db:migrate
```

### 3. Run the Application

```bash
# Start both backend and frontend in development mode
bun run dev
```

This will start:

- **Backend API**: http://localhost:8001
- **Frontend**: http://localhost:3000

## 📋 Available Scripts

### Development

- `bun run dev` - Start both backend and frontend in development mode
- `bun run frontend:dev` - Start only the Next.js frontend
- `bun run backend:dev` - Start only the Rust backend

### Production

- `bun run start` - Start both backend and frontend in production mode
- `bun run build` - Build the frontend for production
- `bun run backend:build` - Build the backend for production

### Database

- `bun run db:migrate` - Run database migrations
- `bun run db:reset` - Reset and re-run all migrations

### Setup & Maintenance

- `bun run setup` - Install all dependencies (frontend + backend)
- `bun run clean` - Clean all build artifacts
- `bun run test:api` - Test the API endpoints

## 🛠️ Technology Stack

### Backend (Rust)

- **Axum** - Web framework
- **Diesel** - ORM and query builder
- **PostgreSQL** - Database
- **Tokio** - Async runtime
- **UUID** - Unique identifiers
- **Serde** - Serialization

### Frontend (Next.js)

- **React 19** - UI framework
- **TypeScript** - Type safety
- **Tailwind CSS** - Styling
- **Axios** - HTTP client

## 📝 Environment Variables

Create `backend/api/.env`:

```env
DATABASE_URL=postgres://username:password@localhost:5432/test123
```

## 🐛 Troubleshooting

### Backend Issues

- **Database connection failed**: Make sure PostgreSQL is running and the DATABASE_URL is correct
- **Migrations failed**: Check if the database exists and you have proper permissions
- **Port 8001 in use**: Change the port in `backend/api/src/main.rs`

### Frontend Issues

- **API connection failed**: Make sure the backend is running on port 8001
- **CORS errors**: The backend includes CORS middleware for development

### Development Tips

- Use `bun run dev` to run both services with hot reload
- Check the browser console and terminal for error messages
- The backend logs all requests and errors to the console


## 📚 Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Diesel Documentation](https://diesel.rs/)
- [Next.js Documentation](https://nextjs.org/docs)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

This project is for learning purposes.
