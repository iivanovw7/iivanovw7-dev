import { defineConfig } from "cypress";

const VIEWPORT = {
	height: 1080,
	width: 1920,
};

const baseUrl = `http://${process.env.SERVER || "localhost:8080"}`;

// eslint-disable-next-line import/no-default-export
export default defineConfig({
	e2e: {
		browser: "chromium",
		config: {
			video: true,
			viewportHeight: VIEWPORT.height,
			viewportWidth: VIEWPORT.width,
		},
		env: {
			baseUrl,
		},
		exit: true,
		fixturesFolder: "e2e/fixtures",
		headed: false,
		headless: true,
		reporterOptions: {
			html: true,
			json: true,
			overwrite: false,
			reportDir: "e2e/reports",
		},
		screenshotsFolder: "e2e/screenshots",
		setupNodeEvents() {},
		specPattern: "e2e/specs/**/*.cy.js",
		supportFile: "e2e/support/e2e.js",
		videosFolder: "e2e/videos",
	},
});
