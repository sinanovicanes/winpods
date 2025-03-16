import { GithubAPI } from './github_api';
import { isValidVersionString } from './utils';

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

async function createLatestReleaseInfo(repo: string): Promise<ReleaseInfo> {
	const latestRelease = await GithubAPI.getLatestRelease(repo);
	const url = latestRelease.assets.find((asset: any) => asset.name.endsWith('x64_en-US.msi.zip'))?.browser_download_url;
	const signatureUrl = latestRelease.assets.find((asset: any) => asset.name.endsWith('x64_en-US.msi.zip.sig'))?.browser_download_url;

	if (!url || !signatureUrl) {
		throw new Error('Failed to find download url');
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

async function getLatestReleaseInfo(env: Env): Promise<ReleaseInfo> {
	const CACHE_KEY = env.CACHE_KEY || 'winpods:latest-release';
	const CACHE_TTL = env.CACHE_TTL || 300;
	const kv = env.KV;

	const cachedRelease = await kv.get(CACHE_KEY);

	if (cachedRelease) {
		return JSON.parse(cachedRelease);
	}

	const release = await createLatestReleaseInfo(env.GITHUB_REPO);

	await kv.put(CACHE_KEY, JSON.stringify(release), { expirationTtl: CACHE_TTL });

	return release;
}

export default {
	async fetch(request, env, ctx): Promise<Response> {
		if (request.method !== 'GET') {
			return new Response('Method not allowed', { status: 405 });
		}

		const url = new URL(request.url);
		const path = url.pathname;
		const version = path.replace('/', '').replace('v', '');

		if (!isValidVersionString(version)) {
			return new Response('Invalid version', { status: 400 });
		}

		try {
			const release = await getLatestReleaseInfo(env);

			if (release.version.replace('v', '') === version) {
				return new Response('No updates available', { status: 204 });
			}

			return new Response(JSON.stringify(release), {
				headers: {
					'Content-Type': 'application/json',
				},
			});
		} catch {
			return new Response('Failed to fetch latest release', { status: 500 });
		}
	},
} satisfies ExportedHandler<Env>;
