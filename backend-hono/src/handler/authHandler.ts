import { Context } from 'hono';
import { setCookie, deleteCookie, getCookie } from 'hono/cookie';
import { AuthService } from '../usecase/AuthService';
import { RegisterRequestSchema, LoginRequestSchema } from '../usecase/dto';
import { AuthEnv } from '../middleware/auth';

const REFRESH_TOKEN_COOKIE_NAME = 'refresh_token';
const COOKIE_MAX_AGE = 30 * 24 * 60 * 60; // 30 days in seconds

export const register = async (c: Context<AuthEnv>) => {
  const body = await c.req.json();
  const validated = RegisterRequestSchema.parse(body);

  const authService = c.get('authService');
  const result = await authService.register(validated);

  // Return response with snake_case field names
  return c.json(
    {
      user_id: result.userId,
      username: result.username,
      email: result.email,
    },
    201
  );
};

export const login = async (c: Context<AuthEnv>) => {
  const body = await c.req.json();
  const validated = LoginRequestSchema.parse(body);

  const authService = c.get('authService');
  const result = await authService.login(validated);

  // Set refresh token as HTTPOnly cookie
  setCookie(c, REFRESH_TOKEN_COOKIE_NAME, result.refreshToken, {
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'Lax',
    maxAge: COOKIE_MAX_AGE,
    path: '/',
  });

  // Return access token and user info (not refresh token)
  return c.json({
    user_id: result.userId,
    username: result.username,
    email: result.email,
    access_token: result.accessToken,
    token_type: result.tokenType,
    expires_in: result.expiresIn,
  });
};

export const refresh = async (c: Context<AuthEnv>) => {
  const refreshToken = getCookie(c, REFRESH_TOKEN_COOKIE_NAME);

  if (!refreshToken) {
    return c.json({ error: 'Refresh token not found' }, 401);
  }

  const authService = c.get('authService');
  const result = await authService.refresh(refreshToken);

  // Update refresh token cookie
  setCookie(c, REFRESH_TOKEN_COOKIE_NAME, result.refreshToken, {
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'Lax',
    maxAge: COOKIE_MAX_AGE,
    path: '/',
  });

  return c.json({
    access_token: result.accessToken,
    token_type: result.tokenType,
    expires_in: result.expiresIn,
  });
};

export const logout = async (c: Context<AuthEnv>) => {
  const refreshToken = getCookie(c, REFRESH_TOKEN_COOKIE_NAME);

  if (refreshToken) {
    const authService = c.get('authService');
    await authService.logout(refreshToken);
  }

  // Delete cookie
  deleteCookie(c, REFRESH_TOKEN_COOKIE_NAME);

  return c.json({ message: 'Logged out successfully' });
};
