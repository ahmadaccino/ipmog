import { geoFromRequest, type GeoInfo } from "../../shared/geo";

const escapeHtml = (value: string) =>
	value.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);

function page(geo: GeoInfo): string {
	const rows: [string, string][] = [
		["City", geo.city],
		["Region", geo.region],
		["Postal code", geo.postalCode],
		["Country", geo.country],
		["ISP", geo.isp],
		["ASN", geo.asn ? `AS${geo.asn}` : "Unknown"],
		["Timezone", geo.timezone],
		["Coordinates", `${geo.latitude.toFixed(4)}, ${geo.longitude.toFixed(4)}`],
	];

	return `<!DOCTYPE html>
<html lang="en" data-theme="dracula">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>ipmog — what is my IP</title>
<meta name="description" content="Mog your IP: geolocation, ISP, ASN and a map pin of where you are." />
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/daisyui@5" />
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/daisyui@5/themes.css" />
<script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
</head>
<body class="min-h-screen bg-base-200">
<div class="mx-auto max-w-5xl p-6 space-y-6">
	<div class="text-center space-y-2">
		<h1 class="text-4xl font-black tracking-tight">ipmog</h1>
		<p class="opacity-70">the epic IP mog checker — no sex pill ads</p>
	</div>

	<div class="card bg-base-100 shadow-xl">
		<div class="card-body items-center text-center gap-4">
			<div class="badge badge-primary badge-lg">Your IP address</div>
			<code id="ip" class="text-2xl md:text-4xl font-mono break-all">${escapeHtml(geo.ip)}</code>
			<button id="copy" class="btn btn-sm btn-outline">Copy</button>
		</div>
	</div>

	<div class="grid gap-6 md:grid-cols-2">
		<div class="card bg-base-100 shadow-xl">
			<div class="card-body">
				<h2 class="card-title">Details</h2>
				<table class="table table-zebra">
					<tbody>
${rows
	.map(
		([label, value]) =>
			`\t\t\t\t\t\t<tr><th class="opacity-70">${label}</th><td class="text-right font-mono break-all">${escapeHtml(value)}</td></tr>`,
	)
	.join("\n")}
					</tbody>
				</table>
			</div>
		</div>

		<div class="card bg-base-100 shadow-xl">
			<div class="card-body">
				<h2 class="card-title">Where you are</h2>
				<div id="map" class="h-80 w-full rounded-box z-0"></div>
			</div>
		</div>
	</div>

	<div class="card bg-base-100 shadow-xl">
		<div class="card-body">
			<h2 class="card-title">Prefer the terminal?</h2>
			<p class="opacity-70">Same data as a TUI world map, or as raw JSON.</p>
			<div class="mockup-code"><pre data-prefix="$"><code>cargo install ipmog &amp;&amp; ipmog</code></pre><pre data-prefix="$"><code>curl https://ip.shnitzel.org</code></pre></div>
		</div>
	</div>
</div>

<script>
const geo = ${JSON.stringify(geo).replace(/</g, "\\u003c")};
document.getElementById("copy").addEventListener("click", async (event) => {
	await navigator.clipboard.writeText(geo.ip);
	event.currentTarget.textContent = "Copied";
});

const map = L.map("map").setView([geo.latitude, geo.longitude], 10);
L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
	maxZoom: 19,
	attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
}).addTo(map);
L.marker([geo.latitude, geo.longitude])
	.addTo(map)
	.bindPopup(geo.city + ", " + geo.country)
	.openPopup();
</script>
</body>
</html>
`;
}

export default {
	async fetch(request: Request): Promise<Response> {
		const url = new URL(request.url);
		const geo = geoFromRequest(request);

		if (url.pathname === "/api" || url.pathname === "/api.json") {
			return new Response(JSON.stringify(geo), {
				headers: { "Content-Type": "application/json", "Cache-Control": "no-store" },
			});
		}

		return new Response(page(geo), {
			headers: { "Content-Type": "text/html; charset=utf-8", "Cache-Control": "no-store" },
		});
	},
};
