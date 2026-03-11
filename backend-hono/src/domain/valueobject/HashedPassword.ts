import bcrypt from 'bcryptjs';
import { z } from 'zod';

const SALT_ROUNDS = 10;

export const PasswordSchema = z
  .string()
  .min(8, 'Password must be at least 8 characters')
  .max(72, 'Password must be 72 characters or less'); // bcrypt limit

export class HashedPassword {
  private readonly hash: string;

  private constructor(hash: string) {
    this.hash = hash;
  }

  static async fromPlainText(password: string): Promise<HashedPassword> {
    PasswordSchema.parse(password);
    const hash = await bcrypt.hash(password, SALT_ROUNDS);
    return new HashedPassword(hash);
  }

  static fromHash(hash: string): HashedPassword {
    return new HashedPassword(hash);
  }

  async verify(plainPassword: string): Promise<boolean> {
    return bcrypt.compare(plainPassword, this.hash);
  }

  getHash(): string {
    return this.hash;
  }
}
