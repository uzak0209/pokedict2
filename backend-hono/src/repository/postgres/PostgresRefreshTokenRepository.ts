import { eq, lt } from 'drizzle-orm';
import { DbType } from '../../db/connection';
import { refreshTokens } from '../../db/schema';
import { RefreshToken } from '../../domain/entity/RefreshToken';
import {
  RefreshTokenRepository,
  RefreshTokenRepositoryError,
} from '../interface/RefreshTokenRepository';

export class PostgresRefreshTokenRepository implements RefreshTokenRepository {
  constructor(private readonly db: DbType) {}

  async save(token: RefreshToken): Promise<void> {
    try {
      await this.db.insert(refreshTokens).values({
        tokenId: token.getTokenId(),
        userId: token.getUserId(),
        tokenHash: token.getTokenHash(),
        expiresAt: token.getExpiresAt(),
        revoked: token.isRevoked(),
      });
    } catch (error) {
      throw new RefreshTokenRepositoryError(`Failed to save refresh token: ${error}`);
    }
  }

  async findByHash(tokenHash: string): Promise<RefreshToken | null> {
    try {
      const result = await this.db
        .select()
        .from(refreshTokens)
        .where(eq(refreshTokens.tokenHash, tokenHash))
        .limit(1);

      if (result.length === 0) {
        return null;
      }

      const row = result[0];
      return RefreshToken.fromRepository(
        row.tokenId,
        row.userId,
        row.tokenHash,
        row.expiresAt,
        row.createdAt,
        row.revoked
      );
    } catch (error) {
      throw new RefreshTokenRepositoryError(`Failed to find refresh token: ${error}`);
    }
  }

  async revoke(tokenId: string): Promise<void> {
    try {
      await this.db
        .update(refreshTokens)
        .set({ revoked: true })
        .where(eq(refreshTokens.tokenId, tokenId));
    } catch (error) {
      throw new RefreshTokenRepositoryError(`Failed to revoke refresh token: ${error}`);
    }
  }

  async cleanupExpired(): Promise<void> {
    try {
      await this.db.delete(refreshTokens).where(lt(refreshTokens.expiresAt, new Date()));
    } catch (error) {
      throw new RefreshTokenRepositoryError(`Failed to cleanup expired tokens: ${error}`);
    }
  }
}
