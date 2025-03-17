// test/index.spec.ts
import { env, createExecutionContext, waitOnExecutionContext, SELF } from 'cloudflare:test';
import { describe, it, expect } from 'vitest';
import worker from '../src/index';

// For now, you'll need to do something like this to get a correctly-typed
// `Request` to pass to `worker.fetch()`.
const IncomingRequest = Request<unknown, IncomingRequestCfProperties>;

describe('Releases Worker', () => {
	it("responds with 400 if version isn't valid (unit style)", async () => {
		const request = new IncomingRequest('http://example.com/1.0');
		const ctx = createExecutionContext();
		const response = await worker.fetch(request, env, ctx);

		await waitOnExecutionContext(ctx);

		expect(response.status).toBe(400);
		expect(await response.text()).toMatchInlineSnapshot(`"Invalid version"`);
	});

	it("responds with 405 if method isn't GET (unit style)", async () => {
		const request = new IncomingRequest('http://example.com/1.0', { method: 'POST' });
		const ctx = createExecutionContext();
		const response = await worker.fetch(request, env, ctx);

		await waitOnExecutionContext(ctx);
		expect(response.status).toBe(405);
		expect(await response.text()).toMatchInlineSnapshot(`"Method not allowed"`);
	});

	it("responds with 400 if version isn't valid (integration style)", async () => {
		const response = await SELF.fetch('https://example.com');
		expect(response.status).toBe(400);
		expect(await response.text()).toMatchInlineSnapshot(`"Invalid version"`);
	});

	it("responds with 405 if method isn't GET (unit style)", async () => {
		const response = await SELF.fetch('https://example.com', { method: 'POST' });
		expect(response.status).toBe(405);
		expect(await response.text()).toMatchInlineSnapshot(`"Method not allowed"`);
	});
});
