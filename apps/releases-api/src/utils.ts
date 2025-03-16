export function isValidVersionString(version: string): boolean {
	return /^\d+\.\d+\.\d+$/.test(version);
}
