export const Selectors = {
	common: {
		buttonIcon: (icon) => {
			return icon
				? `button:has(svg.button-icon__icon use[href='/assets/svg/sprite.svg#${icon}'])`
				: ".button-icon";
		},
		linkIcon: (icon) => {
			return icon ? `a:has(svg.link-icon__icon use[href='/assets/svg/sprite.svg#${icon}'])` : ".link-icon";
		},
		navbar: ".navbar",
		themeSwitch: "label[for='theme-switch']",
	},
	home: {
		buttons: ".home-jobs__buttons-container",
		jobs: ".home-jobs",
		slide: ".home-jobs__slider-slide",
		slider: ".home-jobs__slider-container",
		sliderHeaderBottom: ".home-jobs__slider-header-bottom",
	},
};
