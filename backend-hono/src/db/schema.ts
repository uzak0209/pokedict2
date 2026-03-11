import { pgTable, uuid, varchar, timestamp, boolean, integer, text } from 'drizzle-orm/pg-core';

export const users = pgTable('users', {
  userId: uuid('user_id').primaryKey(),
  username: varchar('username', { length: 20 }).notNull().unique(),
  email: varchar('email', { length: 254 }).notNull().unique(),
  passwordHash: varchar('password_hash', { length: 60 }).notNull(),
  createdAt: timestamp('created_at').notNull().defaultNow(),
  updatedAt: timestamp('updated_at').notNull().defaultNow(),
});

export const refreshTokens = pgTable('refresh_tokens', {
  tokenId: uuid('token_id').primaryKey(),
  userId: uuid('user_id')
    .notNull()
    .references(() => users.userId),
  tokenHash: varchar('token_hash', { length: 255 }).notNull().unique(),
  expiresAt: timestamp('expires_at').notNull(),
  createdAt: timestamp('created_at').notNull().defaultNow(),
  revoked: boolean('revoked').notNull().default(false),
});

export const userPokemon = pgTable('user_pokemon', {
  pokemonId: uuid('pokemon_id').primaryKey(),
  userId: uuid('user_id')
    .notNull()
    .references(() => users.userId),
  formId: integer('form_id').notNull(),
  nickname: varchar('nickname', { length: 100 }),
  nature: varchar('nature', { length: 20 }).notNull(),
  ability: varchar('ability', { length: 100 }).notNull(),
  item: varchar('item', { length: 100 }),
  teraType: varchar('tera_type', { length: 20 }).notNull(),

  // Moves
  move1: varchar('move1', { length: 100 }).notNull(),
  move2: varchar('move2', { length: 100 }),
  move3: varchar('move3', { length: 100 }),
  move4: varchar('move4', { length: 100 }),

  // EVs (0-252)
  evHp: integer('ev_hp').notNull().default(0),
  evAttack: integer('ev_attack').notNull().default(0),
  evDefense: integer('ev_defense').notNull().default(0),
  evSpecialAttack: integer('ev_special_attack').notNull().default(0),
  evSpecialDefense: integer('ev_special_defense').notNull().default(0),
  evSpeed: integer('ev_speed').notNull().default(0),

  // IVs (0-31)
  ivHp: integer('iv_hp').notNull().default(31),
  ivAttack: integer('iv_attack').notNull().default(31),
  ivDefense: integer('iv_defense').notNull().default(31),
  ivSpecialAttack: integer('iv_special_attack').notNull().default(31),
  ivSpecialDefense: integer('iv_special_defense').notNull().default(31),
  ivSpeed: integer('iv_speed').notNull().default(31),

  createdAt: timestamp('created_at').notNull().defaultNow(),
});

export const teams = pgTable('teams', {
  teamId: uuid('team_id').primaryKey(),
  ownerId: uuid('owner_id')
    .notNull()
    .references(() => users.userId),
  teamName: varchar('team_name', { length: 50 }).notNull(),
  createdAt: timestamp('created_at').notNull().defaultNow(),
  updatedAt: timestamp('updated_at').notNull().defaultNow(),
});

export const teamPokemon = pgTable('team_pokemon', {
  teamId: uuid('team_id')
    .notNull()
    .references(() => teams.teamId),
  slot: integer('slot').notNull(), // 0-5
  formId: integer('form_id').notNull(),
  terastalType: varchar('terastal_type', { length: 20 }).notNull(),
});

// Master data tables (already exist in the database)
export const pokemonForms = pgTable('pokemon_forms', {
  formId: integer('form_id').primaryKey(),
  speciesId: integer('species_id').notNull(),
  name: text('name').notNull(),
  nameJp: text('name_jp').notNull(),
  type1: varchar('type1', { length: 20 }).notNull(),
  type2: varchar('type2', { length: 20 }),
});
