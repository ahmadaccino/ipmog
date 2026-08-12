export type GeoInfo = {
	ip: string;
	city: string;
	region: string;
	postalCode: string;
	country: string;
	isp: string;
	asn: number;
	timezone: string;
	latitude: number;
	longitude: number;
};

export function geoFromRequest(request: Request): GeoInfo {
	const cf = request.cf as Record<string, unknown> | undefined;
	const text = (value: unknown) => (value ? String(value) : "Unknown");

	return {
		ip: request.headers.get("CF-Connecting-IP") || "unknown",
		city: text(cf?.city),
		region: text(cf?.region),
		postalCode: text(cf?.postalCode),
		country: text(cf?.country),
		isp: text(cf?.asOrganization),
		asn: cf?.asn ? Number(cf.asn) : 0,
		timezone: text(cf?.timezone),
		latitude: cf?.latitude ? Number(cf.latitude) : 0,
		longitude: cf?.longitude ? Number(cf.longitude) : 0,
	};
}
