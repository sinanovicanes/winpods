import { GithubAPI } from '@winpods/github-api';

interface ReleasePlatform {
	signature: string;
	url: string;
}

interface ReleaseInfo {
	version: string;
	notes: string;
	pub_date: string;
	platforms: {
		['windows-x86_64']: ReleasePlatform;
	};
}

export class ReleaseService {
	constructor(private readonly env: Env) {}

	private async createLatestReleaseInfo(repo: string): Promise<ReleaseInfo> {
		const latestRelease = await GithubAPI.getLatestRelease(repo);
		const url = latestRelease.assets.find((asset) => asset.name.endsWith('x64_en-US.msi.zip'))?.browser_download_url;
		const signatureUrl = latestRelease.assets.find((asset) => asset.name.endsWith('x64_en-US.msi.zip.sig'))?.browser_download_url;

		if (!url || !signatureUrl) {
			throw new Error('Failed to find download url or signature url');
		}

		const signature = await fetch(signatureUrl).then((res) => res.text());

		if (!signature) {
			throw new Error('Failed to fetch signature');
		}

		return {
			version: latestRelease.tag_name,
			notes: latestRelease.body,
			pub_date: latestRelease.published_at,
			platforms: {
				'windows-x86_64': {
					signature,
					url,
				},
			},
		};
	}

	async getLatestReleaseInfo(): Promise<ReleaseInfo> {
		const CACHE_KEY = this.env.CACHE_KEY || 'winpods:latest-release';
		const CACHE_TTL = this.env.CACHE_TTL || 300;
		const kv = this.env.KV;

		const cachedRelease = await kv.get(CACHE_KEY);

		if (cachedRelease) {
			console.log('Returning cached release');
			return JSON.parse(cachedRelease);
		}

		const release = await this.createLatestReleaseInfo(this.env.GITHUB_REPO);
		console.log('Caching release');
		await kv.put(CACHE_KEY, JSON.stringify(release), {
			expirationTtl: CACHE_TTL,
		});

		return release;
	}
}
