import { Username } from '../valueobject/Username';
import { Email } from '../valueobject/Email';
import { HashedPassword } from '../valueobject/HashedPassword';

export class User {
  constructor(
    private readonly userId: string,
    private readonly username: Username,
    private readonly email: Email,
    private readonly hashedPassword: HashedPassword
  ) {}

  static async create(
    userId: string,
    username: string,
    email: string,
    plainPassword: string
  ): Promise<User> {
    const usernameVO = Username.create(username);
    const emailVO = Email.create(email);
    const passwordVO = await HashedPassword.fromPlainText(plainPassword);

    return new User(userId, usernameVO, emailVO, passwordVO);
  }

  static fromRepository(
    userId: string,
    username: string,
    email: string,
    passwordHash: string
  ): User {
    const usernameVO = Username.create(username);
    const emailVO = Email.create(email);
    const passwordVO = HashedPassword.fromHash(passwordHash);

    return new User(userId, usernameVO, emailVO, passwordVO);
  }

  async verifyPassword(plainPassword: string): Promise<boolean> {
    return this.hashedPassword.verify(plainPassword);
  }

  getUserId(): string {
    return this.userId;
  }

  getUsername(): string {
    return this.username.getValue();
  }

  getEmail(): string {
    return this.email.getValue();
  }

  getPasswordHash(): string {
    return this.hashedPassword.getHash();
  }
}
