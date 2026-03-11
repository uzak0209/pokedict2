import jwt from 'jsonwebtoken';

const ACCESS_TOKEN_EXPIRY = '15m'; // 15 minutes
const REFRESH_TOKEN_EXPIRY = '30d'; // 30 days

export interface TokenClaims {
  sub: string; // user_id
  exp: number;
  iat: number;
  tokenType: 'access' | 'refresh';
}

export interface TokenPair {
  accessToken: string;
  refreshToken: string;
  tokenType: string;
  expiresIn: number;
}

export class JwtService {
  constructor(private readonly secret: string) {}

  generateAccessToken(userId: string): string {
    return jwt.sign({ sub: userId, tokenType: 'access' }, this.secret, {
      expiresIn: ACCESS_TOKEN_EXPIRY,
    });
  }

  generateRefreshToken(userId: string): string {
    return jwt.sign({ sub: userId, tokenType: 'refresh' }, this.secret, {
      expiresIn: REFRESH_TOKEN_EXPIRY,
    });
  }

  generatePair(userId: string): TokenPair {
    const accessToken = this.generateAccessToken(userId);
    const refreshToken = this.generateRefreshToken(userId);

    return {
      accessToken,
      refreshToken,
      tokenType: 'Bearer',
      expiresIn: 15 * 60, // 15 minutes in seconds
    };
  }

  verify(token: string): TokenClaims {
    try {
      const decoded = jwt.verify(token, this.secret) as TokenClaims;
      return decoded;
    } catch (error) {
      if (error instanceof jwt.TokenExpiredError) {
        throw new Error('Token expired');
      }
      if (error instanceof jwt.JsonWebTokenError) {
        throw new Error('Invalid token');
      }
      throw error;
    }
  }

  decode(token: string): TokenClaims | null {
    const decoded = jwt.decode(token) as TokenClaims | null;
    return decoded;
  }
}
