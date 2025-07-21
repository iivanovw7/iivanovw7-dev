import { spawn } from "child_process";
import cypress from "cypress";
import waitOn from "wait-on";

const processes = {
	build: null,
	server: null,
	exitCode: 1,
};

/**
 *	@typedef {object} Processes
 *	@property {import("child_process").ChildProcessWithoutNullStreams | null} build
 *		- The build process.
 *	@property {import("child_process").ChildProcessWithoutNullStreams | null} server
 *		- The server process.
 *	@property {number} exitCode - The exit code for the E2E runner.
 */

/**
 *	Starts the server build process and waits for its completion.
 *	Resolves if the build is successful, rejects otherwise.
 *	@async
 *	@returns {Promise<void>} A promise that resolves when the server build completes successfully, or rejects if it fails.
 */
async function startServerBuild() {
	processes.build = spawn("pnpm", ["run", "build:server"], {
		cwd: process.cwd(),
		shell: true,
		stdio: "inherit",
	});

	await new Promise((resolve, reject) => {
		processes.build.on("close", (code) => {
			if (code === 0) {
				console.log("Server build completed successfully.");
				resolve(null);
			} else {
				reject(new Error(`Server build failed with code ${code}`));
			}
		});

		processes.build.on("error", (err) => {
			reject(new Error(`Failed to start build process: ${err.message}`));
		});
	});
}

/**
 *	Starts the server application process.
 *	Sets up an uncaught exception handler to stop the server and exit if an error occurs.
 *	@async
 *	@returns {Promise<void>} A promise that resolves once the server process is spawned.
 */
async function startServer() {
	processes.server = spawn("pnpm", ["run", "build:run"], {
		cwd: process.cwd(),
		shell: true,
		stdio: "inherit",
	});

	process.on("uncaughtException", (err) => {
		console.error("Uncaught exception:", err);
		stopServer();
		process.exit(1);
	});
}

/**
 *	Stops the running server process if it exists and is not already killed.
 */
function stopServer() {
	if (processes.server && !processes.server.killed) {
		console.log("Stopping server process");
		processes.server.kill("SIGINT");
	}
}

/**
 *	Runs Cypress end-to-end tests, waits for the server to be available.
 *	Finally, stops the server regardless of test outcome.
 *	@async
 *	@returns {Promise<number>} The exit code (0 for success, 1 for failure).
 */
async function runCypressTests() {
	await waitOn({
		resources: ["http://localhost:8080"],
		delay: 1000,
		interval: 1000,
		timeout: 60000,
		tcpTimeout: 1000,
		log: true,
	});

	const results = await cypress.run();

	stopServer();

	return results.totalFailed ? 1 : 0;
}

["SIGINT", "SIGUSR1", "SIGUSR2", "SIGTERM"].forEach((eventType) => {
	process.on(eventType, async () => {
		stopServer();
		process.exit(1);
	});
});

try {
	await startServerBuild();
	await startServer();

	processes.exitCode = await runCypressTests();
} catch (error) {
	console.error("An error occurred during E2E test execution:", error.message);
	processes.exitCode = 1;
} finally {
	stopServer();
	process.exit(processes.exitCode);
}
