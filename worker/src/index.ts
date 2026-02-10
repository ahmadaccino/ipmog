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

		const cf = request.cf as Record<string, unknown> | undefined;

		const data = {
			ip: request.headers.get("CF-Connecting-IP") || "unknown",
			city: cf?.city || "Unknown",
			region: cf?.region || "Unknown",
			postalCode: cf?.postalCode || "Unknown",
			country: cf?.country || "Unknown",
			isp: cf?.asOrganization || "Unknown",
			asn: cf?.asn ? Number(cf.asn) : 0,
			timezone: cf?.timezone || "Unknown",
			latitude: cf?.latitude ? Number(cf.latitude) : 0,
			longitude: cf?.longitude ? Number(cf.longitude) : 0,
		};

		return new Response(JSON.stringify(data), {
			headers: {
				"Content-Type": "application/json",
				"Access-Control-Allow-Origin": "*",
			},
		});
	},
};
