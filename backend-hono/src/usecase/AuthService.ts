import { v4 as uuidv4 } from 'uuid';
import { User } from '../domain/entity/User';
import { RefreshToken } from '../domain/entity/RefreshToken';
import { UserRepository } from '../repository/interface/UserRepository';
import { RefreshTokenRepository } from '../repository/interface/RefreshTokenRepository';
import { JwtService, TokenPair } from '../domain/valueobject/JWT';

export class AuthError extends Error {
  constructor(
    message: string,
    public readonly code: string
  ) {
    super(message);
    this.name = 'AuthError';
  }

  static invalidCredentials(): AuthError {
    return new AuthError('Invalid email or password', 'INVALID_CREDENTIALS');
  }

  static invalidToken(): AuthError {
    return new AuthError('Invalid token', 'INVALID_TOKEN');
  }

  static tokenExpired(): AuthError {
    return new AuthError('Token expired', 'TOKEN_EXPIRED');
  }

  static tokenRevoked(): AuthError {
    return new AuthError('Token has been revoked', 'TOKEN_REVOKED');
  }

  static userNotFound(): AuthError {
    return new AuthError('User not found', 'USER_NOT_FOUND');
  }
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface RegisterResponse {
  userId: string;
  username: string;
  email: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  userId: string;
  username: string;
  email: string;
  accessToken: string;
  refreshToken: string;
  tokenType: string;
  expiresIn: number;
}

export class AuthService {
  constructor(
    private readonly userRepository: UserRepository,
    private readonly refreshTokenRepository: RefreshTokenRepository,
    private readonly jwtService: JwtService
  ) {}

  async register(request: RegisterRequest): Promise<RegisterResponse> {
    // Check if username or email already exists
    const usernameExists = await this.userRepository.existsByUsername(request.username);
    if (usernameExists) {
      throw new AuthError('Username already exists', 'USERNAME_EXISTS');
    }

    const emailExists = await this.userRepository.existsByEmail(request.email);
    if (emailExists) {
      throw new AuthError('Email already exists', 'EMAIL_EXISTS');
    }

    // Create user
    const userId = uuidv4();
    const user = await User.create(userId, request.username, request.email, request.password);

    // Save user
    await this.userRepository.save(user);

    return {
      userId: user.getUserId(),
      username: user.getUsername(),
      email: user.getEmail(),
    };
  }

  async login(request: LoginRequest): Promise<LoginResponse> {
    // Find user by email
    const user = await this.userRepository.findByEmail(request.email);
    if (!user) {
      throw AuthError.invalidCredentials();
    }

    // Verify password
    const isValid = await user.verifyPassword(request.password);
    if (!isValid) {
      throw AuthError.invalidCredentials();
    }

    // Generate tokens
    const tokenPair = this.jwtService.generatePair(user.getUserId());

    // Save refresh token
    const tokenId = uuidv4();
    const refreshToken = RefreshToken.create(tokenId, user.getUserId(), tokenPair.refreshToken);
    await this.refreshTokenRepository.save(refreshToken);

    return {
      userId: user.getUserId(),
      username: user.getUsername(),
      email: user.getEmail(),
      accessToken: tokenPair.accessToken,
      refreshToken: tokenPair.refreshToken,
      tokenType: tokenPair.tokenType,
      expiresIn: tokenPair.expiresIn,
    };
  }

  async refresh(refreshTokenString: string): Promise<TokenPair> {
    // Hash the refresh token
    const tokenHash = RefreshToken.hashToken(refreshTokenString);

    // Find refresh token in database
    const refreshToken = await this.refreshTokenRepository.findByHash(tokenHash);
    if (!refreshToken) {
      throw AuthError.invalidToken();
    }

    // Validate token
    if (!refreshToken.isValid()) {
      if (refreshToken.isRevoked()) {
        throw AuthError.tokenRevoked();
      }
      throw AuthError.tokenExpired();
    }

    // Generate new token pair
    const tokenPair = this.jwtService.generatePair(refreshToken.getUserId());

    // Revoke old refresh token
    await this.refreshTokenRepository.revoke(refreshToken.getTokenId());

    // Save new refresh token
    const newTokenId = uuidv4();
    const newRefreshToken = RefreshToken.create(
      newTokenId,
      refreshToken.getUserId(),
      tokenPair.refreshToken
    );
    await this.refreshTokenRepository.save(newRefreshToken);

    return tokenPair;
  }

  async logout(refreshTokenString: string): Promise<void> {
    const tokenHash = RefreshToken.hashToken(refreshTokenString);
    const refreshToken = await this.refreshTokenRepository.findByHash(tokenHash);

    if (refreshToken) {
      await this.refreshTokenRepository.revoke(refreshToken.getTokenId());
    }
  }

  validateAccessToken(token: string): string {
    try {
      const claims = this.jwtService.verify(token);

      if (claims.tokenType !== 'access') {
        throw AuthError.invalidToken();
      }

      return claims.sub;
    } catch (error) {
      if (error instanceof Error) {
        if (error.message === 'Token expired') {
          throw AuthError.tokenExpired();
        }
        throw AuthError.invalidToken();
      }
      throw error;
    }
  }
}
