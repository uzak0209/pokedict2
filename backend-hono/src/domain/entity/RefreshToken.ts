import crypto from 'crypto';

const REFRESH_TOKEN_EXPIRY_DAYS = 30;

export class RefreshToken {
  constructor(
    private readonly tokenId: string,
    private readonly userId: string,
    private readonly tokenHash: string,
    private readonly expiresAt: Date,
    private readonly createdAt: Date,
    private revoked: boolean
  ) {}

  static create(tokenId: string, userId: string, token: string): RefreshToken {
    const tokenHash = crypto.createHash('sha256').update(token).digest('hex');
    const expiresAt = new Date();
    expiresAt.setDate(expiresAt.getDate() + REFRESH_TOKEN_EXPIRY_DAYS);

    return new RefreshToken(tokenId, userId, tokenHash, expiresAt, new Date(), false);
  }

  static fromRepository(
    tokenId: string,
    userId: string,
    tokenHash: string,
    expiresAt: Date,
    createdAt: Date,
    revoked: boolean
  ): RefreshToken {
    return new RefreshToken(tokenId, userId, tokenHash, expiresAt, createdAt, revoked);
  }

  static hashToken(token: string): string {
    return crypto.createHash('sha256').update(token).digest('hex');
  }

  isValid(): boolean {
    return !this.revoked && this.expiresAt > new Date();
  }

  revoke(): void {
    this.revoked = true;
  }

  getTokenId(): string {
    return this.tokenId;
  }

  getUserId(): string {
    return this.userId;
  }

  getTokenHash(): string {
    return this.tokenHash;
  }

  getExpiresAt(): Date {
    return this.expiresAt;
  }

  getCreatedAt(): Date {
    return this.createdAt;
  }

  isRevoked(): boolean {
    return this.revoked;
  }
}
