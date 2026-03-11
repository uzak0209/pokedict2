import { Context, Next } from 'hono';
import { AuthService } from '../usecase/AuthService';

export interface AuthEnv {
  Variables: {
    userId: string;
    authService: AuthService;
  };
}

export const authMiddleware = async (c: Context<AuthEnv>, next: Next) => {
  const authHeader = c.req.header('Authorization');

  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return c.json({ error: 'Missing or invalid authorization header' }, 401);
  }

  const token = authHeader.substring(7);
  const authService = c.get('authService');

  try {
    const userId = authService.validateAccessToken(token);
    c.set('userId', userId);
    await next();
  } catch (error) {
    if (error instanceof Error) {
      return c.json({ error: error.message }, 401);
    }
    return c.json({ error: 'Authentication failed' }, 401);
  }
};

export const optionalAuthMiddleware = async (c: Context<AuthEnv>, next: Next) => {
  const authHeader = c.req.header('Authorization');

  if (authHeader && authHeader.startsWith('Bearer ')) {
    const token = authHeader.substring(7);
    const authService = c.get('authService');

    try {
      const userId = authService.validateAccessToken(token);
      c.set('userId', userId);
    } catch {
      // Ignore authentication errors for optional auth
    }
  }

  await next();
};
