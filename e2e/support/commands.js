import { Selectors } from "./utils";

Cypress.Commands.add("toggleTheme", () => {
	cy.get(Selectors.common.navbar).get(Selectors.common.themeSwitch).click();
});

Cypress.Commands.add("verifyTheme", (theme) => {
	cy.get("html").should("have.attr", "data-theme", theme);
});
