import { describe, it, expect } from 'vitest';
import { isValidVersionString } from '../src/utils';

describe('Utils', () => {
	it('should validate version string', async () => {
		const version = '1.0.0';

		expect(isValidVersionString(version)).toBe(true);
	});

	it("shouldn't validate invalid version string", async () => {
		const invalid = 'this is not a version';

		expect(isValidVersionString(invalid)).toBe(false);
	});
});
