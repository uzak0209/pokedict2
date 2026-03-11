import { Context } from 'hono';
import { ZodError } from 'zod';
import { AuthError } from '../usecase/AuthService';

export const errorHandler = (err: Error, c: Context) => {
  console.error('Error:', err);

  // Zod validation errors
  if (err instanceof ZodError) {
    return c.json(
      {
        error: 'Validation error',
        details: err.errors.map((e) => ({
          path: e.path.join('.'),
          message: e.message,
        })),
      },
      400
    );
  }

  // Auth errors
  if (err instanceof AuthError) {
    const statusCode = getAuthErrorStatus(err.code) as 401 | 404 | 409 | 500;
    return c.json(
      {
        error: err.message,
        code: err.code,
      },
      statusCode
    );
  }

  // Default error
  return c.json(
    {
      error: 'Internal server error',
    },
    500
  );
};

function getAuthErrorStatus(code: string): number {
  switch (code) {
    case 'INVALID_CREDENTIALS':
    case 'INVALID_TOKEN':
    case 'TOKEN_EXPIRED':
    case 'TOKEN_REVOKED':
      return 401;
    case 'USERNAME_EXISTS':
    case 'EMAIL_EXISTS':
      return 409;
    case 'USER_NOT_FOUND':
      return 404;
    default:
      return 500;
  }
}
