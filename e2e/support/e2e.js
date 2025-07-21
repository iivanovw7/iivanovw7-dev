import "@cypress/code-coverage/support";

import "./commands";

beforeEach(async () => {
	cy.window().then((win) => {
		win.localStorage.setItem("dark-mode", "false");
	});
});
