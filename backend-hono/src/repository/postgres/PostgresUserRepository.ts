import { eq } from 'drizzle-orm';
import { DbType } from '../../db/connection';
import { users } from '../../db/schema';
import { User } from '../../domain/entity/User';
import { UserRepository, UserRepositoryError } from '../interface/UserRepository';

export class PostgresUserRepository implements UserRepository {
  constructor(private readonly db: DbType) {}

  async save(user: User): Promise<void> {
    try {
      await this.db.insert(users).values({
        userId: user.getUserId(),
        username: user.getUsername(),
        email: user.getEmail(),
        passwordHash: user.getPasswordHash(),
      });
    } catch (error) {
      throw new UserRepositoryError(`Failed to save user: ${error}`);
    }
  }

  async findById(userId: string): Promise<User | null> {
    try {
      const result = await this.db.select().from(users).where(eq(users.userId, userId)).limit(1);

      if (result.length === 0) {
        return null;
      }

      const row = result[0];
      return User.fromRepository(row.userId, row.username, row.email, row.passwordHash);
    } catch (error) {
      throw new UserRepositoryError(`Failed to find user by ID: ${error}`);
    }
  }

  async findByEmail(email: string): Promise<User | null> {
    try {
      const result = await this.db
        .select()
        .from(users)
        .where(eq(users.email, email.toLowerCase()))
        .limit(1);

      if (result.length === 0) {
        return null;
      }

      const row = result[0];
      return User.fromRepository(row.userId, row.username, row.email, row.passwordHash);
    } catch (error) {
      throw new UserRepositoryError(`Failed to find user by email: ${error}`);
    }
  }

  async findByUsername(username: string): Promise<User | null> {
    try {
      const result = await this.db
        .select()
        .from(users)
        .where(eq(users.username, username))
        .limit(1);

      if (result.length === 0) {
        return null;
      }

      const row = result[0];
      return User.fromRepository(row.userId, row.username, row.email, row.passwordHash);
    } catch (error) {
      throw new UserRepositoryError(`Failed to find user by username: ${error}`);
    }
  }

  async existsByEmail(email: string): Promise<boolean> {
    try {
      const result = await this.db
        .select({ userId: users.userId })
        .from(users)
        .where(eq(users.email, email.toLowerCase()))
        .limit(1);

      return result.length > 0;
    } catch (error) {
      throw new UserRepositoryError(`Failed to check email existence: ${error}`);
    }
  }

  async existsByUsername(username: string): Promise<boolean> {
    try {
      const result = await this.db
        .select({ userId: users.userId })
        .from(users)
        .where(eq(users.username, username))
        .limit(1);

      return result.length > 0;
    } catch (error) {
      throw new UserRepositoryError(`Failed to check username existence: ${error}`);
    }
  }
}
