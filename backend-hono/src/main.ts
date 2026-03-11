// Node.js development server
// This file is used for local development with Node.js
// For Cloudflare Workers deployment, use src/index.ts

import { serve } from '@hono/node-server';
import { env } from './config/env';
import app from './index';

// Start server
const port = parseInt(env.PORT);
console.log(`Server is running on http://${env.HOST}:${port}`);

serve({
  fetch: app.fetch,
  port,
  hostname: env.HOST,
});
