import { RefreshToken } from '../../domain/entity/RefreshToken';

export class RefreshTokenRepositoryError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'RefreshTokenRepositoryError';
  }
}

export interface RefreshTokenRepository {
  save(token: RefreshToken): Promise<void>;
  findByHash(tokenHash: string): Promise<RefreshToken | null>;
  revoke(tokenId: string): Promise<void>;
  cleanupExpired(): Promise<void>;
}
