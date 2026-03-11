import { User } from '../../domain/entity/User';

export class UserRepositoryError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'UserRepositoryError';
  }
}

export interface UserRepository {
  save(user: User): Promise<void>;
  findById(userId: string): Promise<User | null>;
  findByEmail(email: string): Promise<User | null>;
  findByUsername(username: string): Promise<User | null>;
  existsByEmail(email: string): Promise<boolean>;
  existsByUsername(username: string): Promise<boolean>;
}
