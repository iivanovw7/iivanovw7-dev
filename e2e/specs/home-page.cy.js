import { Selectors } from "../support/utils";

describe("[Home] page", () => {
	beforeEach(() => {
		cy.visit(Cypress.env("baseUrl"));
	});

	it("Should have the correct title", () => {
		cy.title().should("include", "Igor Ivanov | Frontend developer");
	});

	it("Should render navbar with controls", () => {
		cy.get(Selectors.common.navbar).first().should("be.visible");

		cy.get(Selectors.common.linkIcon("terminal"))
			.first()
			.should("exist")
			.should("have.attr", "href", "/")
			.should("have.attr", "target", "_self");

		cy.get(Selectors.common.linkIcon("github"))
			.first()
			.should("exist")
			.should("have.attr", "href", "https://github.com/iivanovw7/iivanovw7-dev")
			.should("have.attr", "target", "_blank");
	});

	it("Should toggle theme", () => {
		// prettier-ignore
		cy.verifyTheme("light")
			.toggleTheme()
			.verifyTheme("dark")
			.toggleTheme()
			.verifyTheme("light");
	});

	it("Should contain slides", () => {
		// prettier-ignore
		cy.get(Selectors.home.jobs)
			.get(Selectors.home.slider)
			.get(Selectors.home.slide)
			.should("have.length.gt", 2);
	});

	it("Should have social links", () => {
		cy.get(Selectors.home.jobs)
			.get(Selectors.home.sliderHeaderBottom)
			.get(Selectors.common.linkIcon())
			.should("have.length.gt", 4);
	});

	it("Should switch slide and switch back", () => {
		cy.get(Selectors.home.jobs).get(Selectors.home.slide).first().as("firstSlide");

		cy.get(Selectors.home.jobs)
			.find(Selectors.home.buttons)
			.find("#prevButton")
			.as("prevButton")
			.should("be.disabled");

		cy.get("@firstSlide").should("have.css", "transform", "matrix(1, 0, 0, 1, 0, 0)");

		cy.get(Selectors.home.jobs)
			.find(Selectors.home.buttons)
			.find("#nextButton")
			.as("nextButton")
			.should("not.be.disabled")
			.click();

		cy.get("@firstSlide").should("have.css", "transform", "matrix(1, 0, 0, 1, -563.999, 0)");

		cy.get("@prevButton").should("be.visible").and("not.be.disabled");

		cy.get("@prevButton").should("be.visible").click();

		cy.get("@firstSlide").should("have.css", "transform", "matrix(1, 0, 0, 1, 0, 0)");

		cy.get("@prevButton").should("be.visible").and("be.disabled");
	});
});
