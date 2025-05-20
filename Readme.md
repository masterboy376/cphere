# Cphere

Cphere is a real-time chat application featuring user authentication, messaging, and notifications. The backend is implemented in Rust, leveraging async patterns and MongoDB to ensure high performance and scalability. The frontend is built with TypeScript and React (Next.js), providing a responsive and intuitive user interface for seamless communication.

## Features

* **User Authentication**: Secure sign up, log in, and session management using JWTs and session validation middleware.
* **Real-Time Chat**: Persistent one-to-one and group chat support via RESTful APIs and WebSockets.
* **Notifications**: In-app and email notifications for new messages, requests, and system alerts.
* **Media Handling**: Support for sending images and files in chats.
* **WebRTC Signaling**: Peer-to-peer audio/video calls using WebRTC signaling channels.
* **Admin Dashboard**: Management tools for monitoring active sessions, chats, and user metrics.

## Architecture Overview

Cphere follows a modular architecture to separate concerns and enhance maintainability.

### Backend (Rust)

* **Entry Points**: `src/main.rs` initializes the server and routes; `src/lib.rs` exposes core functionality.
* **Configuration (`config/`)**: Application settings and MongoDB connection handling.
* **Models (`models/`)**: Data schemas representing users, chats, messages, sessions, and notifications.
* **Services (`services/`)**: Business logic for authentication, chat operations, session management, and notification delivery.
* **API (`api/`)**: Route handlers mapping HTTP endpoints to service functions.
* **Middleware (`middleware/`)**: Session validation, rate limiting, and request logging.
* **Utilities (`utils/`)**: Helper functions for password hashing, input validation, and environment loading.
* **Database Migrations (`migrations/`)**: Scripts to evolve the database schema.
* **Testing (`tests/`)**: Unit, integration, and acceptance tests ensuring code quality.
* **Deployment (`docker/`)**: Dockerfile and Compose setup for local development and production.
* **CI/CD**: GitHub Actions workflows in `.github/workflows` for automated testing and linting.

### Frontend (React + TypeScript)

* **Framework**: Next.js for server-side rendering and API routes.
* **State Management**: Context API or Redux for global app state.
* **Components**: Reusable UI components in `components/` (ChatBox, MessageList, UserList, etc.).
* **Pages**: Next.js pages in `pages/` for routing (Login, Register, Chat, Dashboard).
* **Services**: API clients for authentication, chat, and signaling.
* **WebSocket Client**: Establishes and manages WebSocket connections for live messaging.
* **Styling**: CSS modules or styled-components for scoped styling.

## Folder Structure

```
├── backend
│   ├── src
│   │   ├── config
│   │   ├── handlers
│   │   ├── middleware
│   │   ├── models
│   │   ├── services
│   │   ├── states
│   │   ├── types
│   │   ├── utils
│   │   └── websocket
│   └── tests
│       ├── acceptance
│       ├── integration
│       └── unit
│           └── handlers
└── frontend
    ├── public
    └── src
        ├── assets
        ├── components
        │   ├── button
        │   ├── chat
        │   ├── common
        │   ├── navigation
        │   └── notification
        ├── constants
        ├── contexts
        ├── hooks
        ├── layouts
        ├── pages
        │   ├── auth
        │   ├── chat
        │   ├── search
        │   └── video
        ├── services
        │   ├── auth
        │   ├── chat
        │   ├── user
        │   ├── video
        │   └── ws
        ├── styles
        ├── tests
        │   └── mocks
        ├── types
        ├── utils
        └── wrappers
```

## Getting Started

### Prerequisites

* Rust (>=1.60)
* Node.js (>=16) and npm or Yarn
* Docker & Docker Compose

### Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/masterboy376/cphere.git
   cd cphere
   ```

2. **Setup environment variables**:

   ```bash
   cp backend/.env.example backend/.env
   cp frontend/.env.example frontend/.env
   # Edit values in .env files (MongoDB URI, JWT secret, etc.)
   ```

3. **Run with Docker Compose**:

   ```bash
   docker-compose up --build
   ```

4. **Or run services independently**:

   * **Backend**:

     ```bash
     cd backend
     cargo run
     ```
   * **Frontend**:

     ```bash
     cd ../frontend
     npm install
     npm run dev
     ```

## Usage

* Navigate to `http://localhost:3000` to access the frontend.
* Register or log in to start chatting in real-time.
