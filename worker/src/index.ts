import { geoFromRequest } from "../../shared/geo";

export default {
	async fetch(request: Request): Promise<Response> {
		if (request.method === "OPTIONS") {
			return new Response(null, {
				headers: {
					"Access-Control-Allow-Origin": "*",
					"Access-Control-Allow-Methods": "GET, OPTIONS",
					"Access-Control-Allow-Headers": "Content-Type",
				},
			});
		}

		return new Response(JSON.stringify(geoFromRequest(request)), {
			headers: {
				"Content-Type": "application/json",
				"Access-Control-Allow-Origin": "*",
				"Cache-Control": "no-store",
			},
		});
	},
};
