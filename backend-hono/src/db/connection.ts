import { drizzle } from 'drizzle-orm/postgres-js';
import postgres from 'postgres';
import * as schema from './schema';

// Cache for database connections (per-worker lifecycle)
const dbCache = new Map<string, ReturnType<typeof drizzle>>();

export function getDb(connectionString?: string) {
  // For Node.js environment (development)
  if (!connectionString && typeof process !== 'undefined') {
    connectionString = process.env.DATABASE_URL;
  }

  if (!connectionString) {
    throw new Error('DATABASE_URL is not set');
  }

  // Return cached connection if exists
  if (dbCache.has(connectionString)) {
    return dbCache.get(connectionString)!;
  }

  // Create new connection
  const client = postgres(connectionString, {
    // Cloudflare Workers compatible settings
    prepare: false,
    max: 1, // Single connection for Workers
  });

  const db = drizzle(client, { schema });
  dbCache.set(connectionString, db);

  return db;
}

export type DbType = ReturnType<typeof getDb>;
