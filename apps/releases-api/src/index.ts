import { ReleaseService } from './release.service';
import { isValidVersionString } from './utils';

export default {
	async fetch(request, env, ctx): Promise<Response> {
		if (request.method !== 'GET') {
			return new Response('Method not allowed', { status: 405 });
		}

		const url = new URL(request.url);
		const path = url.pathname;
		const version = path.replace('/', '').replace('v', '');

		if (!isValidVersionString(version)) {
			return new Response('Not found', { status: 404 });
		}

		try {
			const releaseService = new ReleaseService(env);
			const release = await releaseService.getLatestReleaseInfo();

			if (release.version.replace('v', '') === version) {
				console.log('No updates available for version: ' + version);
				return new Response('No updates available', { status: 204 });
			}

			return new Response(JSON.stringify(release), {
				headers: {
					'Content-Type': 'application/json',
				},
			});
		} catch (e) {
			console.error(e);
			return new Response('Failed to fetch latest release', { status: 500 });
		}
	},
} satisfies ExportedHandler<Env>;
