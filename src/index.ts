import { Container, getContainer } from '@cloudflare/containers';
import { Hono } from "hono";

export class BgpkitContainer extends Container {
	defaultPort = 3000;
	sleepAfter = '5m';

	// Optional lifecycle hooks
	override onStart() {
		console.log("Container successfully started");
	}

	override onStop() {
		console.log("Container successfully shut down");
	}

	override onError(error: unknown) {
		console.log("Container error:", error);
	}
}

// Create Hono app with proper typing for Cloudflare Workers
const app = new Hono<{
	Bindings: { BGPKIT_CONTAINER: DurableObjectNamespace<BgpkitContainer> };
}>();

// Get a single container instance (singleton pattern)
app.get("/search", async (c) => {
	// make sure the following query parameters are present:
	// collector, prefix, ts_start, ts_end
	if (!c.req.query('collector') || !c.req.query('prefix') || !c.req.query('ts_start') || !c.req.query('ts_end')) {
		return c.json({ error: "Missing required query parameters: collector, prefix, ts_start, ts_end" }, 400);
	}
	const container = getContainer(c.env.BGPKIT_CONTAINER);
	return await container.fetch(c.req.raw);
});

export default app;
