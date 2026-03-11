import { Hono } from 'hono';
import { logger } from 'hono/logger';
import { cors } from 'hono/cors';
import { getDb } from './db/connection';
import { PostgresUserRepository } from './repository/postgres/PostgresUserRepository';
import { PostgresRefreshTokenRepository } from './repository/postgres/PostgresRefreshTokenRepository';
import { AuthService } from './usecase/AuthService';
import { JwtService } from './domain/valueobject/JWT';
import { authMiddleware, AuthEnv } from './middleware/auth';
import { errorHandler } from './middleware/error';
import * as authHandler from './handler/authHandler';

// Cloudflare Workers environment
type Bindings = {
  DATABASE_URL: string;
  JWT_SECRET: string;
  GEMINI_API_KEY?: string;
  ALLOWED_ORIGIN: string;
  ENVIRONMENT: string;
};

type Variables = AuthEnv['Variables'];

const app = new Hono<{ Bindings: Bindings; Variables: Variables }>();

// Global middleware
app.use('*', logger());
app.use('*', async (c, next) => {
  const allowedOrigin = c.env.ALLOWED_ORIGIN || 'http://localhost:5173';

  return cors({
    origin: allowedOrigin,
    credentials: true,
    allowMethods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
    allowHeaders: ['Authorization', 'Content-Type'],
    maxAge: 3600,
  })(c, next);
});

// Initialize dependencies per request
app.use('*', async (c, next) => {
  const db = getDb(c.env.DATABASE_URL);
  const userRepository = new PostgresUserRepository(db);
  const refreshTokenRepository = new PostgresRefreshTokenRepository(db);
  const jwtService = new JwtService(c.env.JWT_SECRET);
  const authService = new AuthService(userRepository, refreshTokenRepository, jwtService);

  c.set('authService', authService);
  await next();
});

// Health check
app.get('/health', (c) => {
  return c.json({ status: 'ok', environment: c.env.ENVIRONMENT || 'development' });
});

// Auth routes
app.post('/api/auth/register', authHandler.register);
app.post('/api/auth/login', authHandler.login);
app.post('/api/auth/refresh', authHandler.refresh);
app.post('/api/auth/logout', authHandler.logout);

// Protected routes
app.get('/api/users/me', authMiddleware, (c) => {
  const userId = c.get('userId');
  return c.json({ userId });
});

// Error handling
app.onError(errorHandler);

export default app;
