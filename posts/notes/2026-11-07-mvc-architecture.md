---
image: "assets/png/mvc.png"
title: "Architectural Patterns: MVC and the MV* Family"
date: 2026-11-07
description: "Separating Domain Logic, Presentation, and Interaction across Client-Server and Modern Frontend Architectures."
link: none
file_name: 2026-11-07-mvc-architecture
tags:
    - architecture
    - patterns
    - mvc
    - mvvm
---

# Architectural Patterns: MVC and the MV\* Family

A comprehensive breakdown of Model-View-\* architectural patterns, examining the core principles of separating domain logic, presentation, and interaction, deep-diving into classic client-server MVC, analyzing frontend implementation techniques, and exploring MVVM.

_Note: This technical note was built and reviewed with the assistance of a human-guided LLM._

### Table of contents <a name="toc"></a>

-   <a href="#The Core Dilemma: Separating Responsibilities in Interactive Applications">The Core Dilemma: Separating Responsibilities in Interactive Applications</a>
-   <a href="#The Calculator Problem: Why Coupling Breaks Down">The Calculator Problem: Why Coupling Breaks Down</a>
-   <a href="#Deconstructing the MVC Pattern">Deconstructing the MVC Pattern</a>
    -   <a href="#The Real-World Analogy: The Bank and ATM">The Real-World Analogy: The Bank and ATM</a>
    -   <a href="#Core Responsibilities of M, V, and C">Core Responsibilities of M, V, and C</a>
    -   <a href="#MVC Is a Pattern, Not a Rigid Framework">MVC Is a Pattern, Not a Rigid Framework</a>
-   <a href="#Classic Client-Server MVC (Node.js & Express)">Classic Client-Server MVC (Node.js & Express)</a>
    -   <a href="#The Model Layer (Domain Logic & In-Memory Store)">The Model Layer (Domain Logic & In-Memory Store)</a>
    -   <a href="#The View Layer (Server Templates & Safe Event Binding)">The View Layer (Server Templates & Safe Event Binding)</a>
    -   <a href="#The Controller Layer (HTTP Orchestration)">The Controller Layer (HTTP Orchestration)</a>
    -   <a href="#Handling Errors and Alternative Views">Handling Errors and Alternative Views</a>
    -   <a href="#Multi-Client Adaptability: HTML, REST, GraphQL, and SOAP">Multi-Client Adaptability: HTML, REST, GraphQL, and SOAP</a>
    -   <a href="#Production Considerations: Security, Transactions, and Layering">Production Considerations: Security, Transactions, and Layering</a>
-   <a href="#MVC on the Modern Frontend (TypeScript)">MVC on the Modern Frontend (TypeScript)</a>
    -   <a href="#Technique 1: Controller as Pure Mediator (Decoupled Flow)">Technique 1: Controller as Pure Mediator (Decoupled Flow)</a>
    -   <a href="#Technique 2: Model Notifies View (Observer Pattern)">Technique 2: Model Notifies View (Observer Pattern)</a>
    -   <a href="#Technique 3: Dependency Injection and Component Assembly">Technique 3: Dependency Injection and Component Assembly</a>
-   <a href="#Exploring the Broader MV* Family: MVVM">Exploring the Broader MV\* Family: MVVM</a>
    -   <a href="#Model-View-ViewModel (MVVM) and Its Origins">Model-View-ViewModel (MVVM) and Its Origins</a>
    -   <a href="#Declarative Data Binding and the ViewModel Abstraction">Declarative Data Binding and the ViewModel Abstraction</a>
    -   <a href="#Vue.js: An MVVM-Inspired Implementation">Vue.js: An MVVM-Inspired Implementation</a>
    -   <a href="#When to Choose MVC vs MVVM">When to Choose MVC vs MVVM</a>
-   <a href="#Summary & Key Takeaways">Summary & Key Takeaways</a>

---

#### The Core Dilemma: Separating Responsibilities in Interactive Applications <a name="The Core Dilemma: Separating Responsibilities in Interactive Applications"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

The **MV\*** family of architectural patterns (`MVC`, `MVP`, `MVVM`) focuses on separating application responsibilities, particularly domain logic, presentation, and user interaction. Although these patterns share this broad objective, they differ substantially in how their components communicate, coordinate state changes, and manage dependencies.

To establish clear architectural boundaries, it helps to distinguish the primary layers of an application:

-   **Domain Logic:** Rules and behaviors fundamental to the business domain itself (e.g., pricing calculations, order validation, eligibility checks, transaction policies, and shopping cart operations).
-   **Domain Model:** Entities, value objects, and business rules that model the core problem space independently of how data is stored or displayed.
-   **Application Logic:** Coordinates use cases, orchestrates transactions, invokes domain operations, and dispatches external events.
-   **Presentation Logic:** Formats data for visual rendering, manages UI display state (e.g., active tabs, accordion toggles, or loading spinners), and captures user input.
-   **Infrastructure:** Technical mechanisms that support the system without defining business rules (e.g., database drivers, password hashing algorithms, network transports, third-party payment gateways, and file storage).

**Model-View-Controller (MVC)** originated in 1978 through Trygve Reenskaug's work on Smalltalk at Xerox PARC. Its original purpose was to structure interactive desktop applications by separating the underlying information representation from user interaction and visual display. As software evolved from desktop GUIs to server-rendered web applications and modern single-page applications (SPAs), MVC branched into several distinct architectural styles.

When developers fail to separate these concerns, presentation and domain logic collapse into tightly coupled monoliths. Maintenance becomes time-consuming, automated testing requires complex UI harnesses, and introducing new interfaces turns into a high-risk refactoring endeavor.

---

#### The Calculator Problem: Why Coupling Breaks Down <a name="The Calculator Problem: Why Coupling Breaks Down"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

Consider a concrete example: we are developing a scientific calculator application. In a naive implementation, the mathematical calculations (arithmetic, trigonometry, logarithms, exponentiation, and divide-by-zero guards) are written directly inside DOM click listeners and UI rendering functions.

```
+-------------------------------------------------------------+
|                     Tightly Coupled Monolith                |
|  [DOM Events] <---> [Sin / Cos / Log / Add] <---> [Screen]  |
+-------------------------------------------------------------+
```

Now, product requirements shift:

1. A redesigned, modern interface must be delivered.
2. Existing users must retain the option to keep the legacy layout via an interface preference toggle.
3. New users must receive the modern layout by default.

Because the mathematical routines are intertwined with DOM queries and button listeners, we cannot simply swap out the CSS and markup. To support both interfaces, we are forced to either duplicate the complex calculation rules across both views (violating DRY and creating maintenance hazards) or heavily modify existing working code, risking regressions.

```
       [ Legacy UI View ]          [ Modern UI View ]
                \                         /
                 \                       /
                  v                     v
            +----------------------------------+
            |      Single Domain Model         |
            |   (Math Engine: sin, cos, etc.)  |
            +----------------------------------+
```

When presentation is cleanly decoupled from business rules:

-   The **Domain Model** remains an isolated mathematical engine that knows nothing about buttons, HTML, or CSS.
-   The **UI Layers** (both legacy and modern) capture user interactions and translate them into domain calls.
-   An intermediary (the **Controller** or application coordinator) routes interactions between the presentation layer and the domain model without tightly coupling them together.

---

## Deconstructing the MVC Pattern <a name="Deconstructing the MVC Pattern"></a>

#### The Real-World Analogy: The Bank and ATM <a name="The Real-World Analogy: The Bank and ATM"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

A classic analogy for understanding MVC is visiting a physical bank branch or operating an ATM:

1. **The View (ATM Screen / Receptionist):** When you step up to an ATM or approach the reception desk, you interact exclusively with the presentation layer. The ATM touch screen provides numbered buttons and prompt screens. The receptionist greets you and provides paperwork. Neither the screen nor the receptionist executes ledger transfers directly in the bank's core vaults.
2. **The Model (Core Banking Domain Engine):** The internal ledger, transaction engine, currency conversion services, and account rules represent the Model. Only the Model knows how to calculate fees, verify credit limits, enforce regulatory constraints, and debit accounts. It operates independently of whether the request originated from an ATM, a teller workstation, or a mobile banking app.
3. **The Controller (ATM Interaction Coordinator):** When you request a withdrawal, the Controller receives the interaction, extracts the requested amount, and delegates the operation to the banking system. The domain layer verifies account permissions, available funds, transaction limits, and fees. The Controller then coordinates the resulting response and updates the presentation.

---

#### Core Responsibilities of M, V, and C <a name="Core Responsibilities of M, V, and C"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

-   **Model (M):** Represents application data, domain entities, and business operations. Depending on the architecture, it may coordinate persistence, enforce domain invariants, and expose state to other application components. It should remain independent of presentation concerns.
-   **View (V):** Renders data provided to it into a visual or structural format (HTML, server templates, canvas, native widgets) and captures user input. Contains presentation logic only, never core business rules.
-   **Controller (C):** Primarily handles request orchestration and delegates business operations to appropriate application or domain components. It parses inputs, performs basic request validation, invokes domain actions, and selects the appropriate View or response format. Business rules should not be duplicated across controllers.

---

#### MVC Is a Pattern, Not a Rigid Framework <a name="MVC Is a Pattern, Not a Rigid Framework"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

It is important to recognize that MVC is an architectural concept, not a monolithic framework rulebook. Different ecosystems interpret and implement the pattern according to their platform constraints:

-   **Smalltalk MVC (Desktop):** In classic Smalltalk MVC implementations, Views commonly observe Model changes through an Observer-style mechanism, while Controllers interpret user gestures and translate them into application operations.
-   **Web Application MVC (Server-Side):** Exemplified by Express, Ruby on Rails, Django, and ASP.NET. Because HTTP is stateless and request-driven, the Controller receives the HTTP request, queries the Model, feeds the data to a View template, and returns HTML. The View never observes the Model over a persistent connection.
-   **Frontend Component MVC (Client-Side):** In SPAs, controllers often act as mediators between DOM events and reactive client models, or components leverage dedicated state stores.

The historic slogan **"Fat Model, Skinny Controller"** became popular during the early days of Ruby on Rails to discourage putting database queries and business calculations inside controllers. However, in larger enterprise architectures, cramming all business rules, persistence logic, and query logic into Active Record models leads to unwieldy God Objects. Modern practice favors layered designs where controllers remain thin, delegating to focused domain entities, application use-case services, and repository abstractions.

---

## Classic Client-Server MVC (Node.js & Express) <a name="Classic Client-Server MVC (Node.js & Express)"></a>

In traditional server-side web development, the MVC cycle coordinates browser HTTP requests and server-side responses:

```
[ Browser ]
     |
     | 1. Sends HTTP Request
     v
[ Controller ]
     |
     | 2. Invokes domain operations
     v
 [ Model ]
     |
     | 3. Returns domain data / result
     v
[ Controller ]
     |
     | 4. Supplies presentation data
     v
  [ View ]
     |
     | 5. Returns rendered HTML
     v
[ Browser ]
```

Let's examine a clean server-rendered user management application using Node.js, Express, and Handlebars.

#### The Model Layer (Domain Logic & In-Memory Store) <a name="The Model Layer (Domain Logic & In-Memory Store)"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

The Model encapsulates the domain entities, validation rules, and data operations.

Strictly speaking, this example uses a simplified Active Record/service-like Model abstraction rather than a fully separated Domain Model and Repository architecture. This keeps the example concise while illustrating how the MVC Controller delegates application operations to the Model layer.

```typescript
import { randomUUID } from "node:crypto";

export class DuplicateUsernameError extends Error {
	constructor(username: string) {
		super(`User with name "${username}" already exists.`);
		this.name = "DuplicateUsernameError";
	}
}

export interface UserEntity {
	id: string;
	username: string;
	age: number;
}

export class UserModel {
	private static users: UserEntity[] = [];

	public static getAll(): UserEntity[] {
		return [...this.users];
	}

	public static getById(id: string): UserEntity | undefined {
		return this.users.find((user) => user.id === id);
	}

	public static create(username: string, age: number): UserEntity {
		const trimmed = username.trim();

		// Domain invariant validation
		if (!trimmed) {
			throw new Error("Username cannot be empty.");
		}

		if (!Number.isInteger(age) || age <= 0) {
			throw new Error("Age must be a positive integer.");
		}

		const existing = this.users.find((u) => u.username.toLowerCase() === trimmed.toLowerCase());

		if (existing) {
			throw new DuplicateUsernameError(trimmed);
		}

		const newUser: UserEntity = {
			id: randomUUID(),
			username: trimmed,
			age,
		};

		this.users.push(newUser);
		return newUser;
	}

	public static removeById(id: string): void {
		const index = this.users.findIndex((user) => user.id === id);
		if (index === -1) {
			throw new Error(`User with ID "${id}" was not found.`);
		}

		this.users.splice(index, 1);
	}
}
```

---

#### The View Layer (Server Templates & Safe Event Binding) <a name="The View Layer (Server Templates & Safe Event Binding)"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

The View receives data injected by the controller and renders markup. To avoid inline script vulnerabilities (where string interpolation inside HTML event handlers could permit injection), we bind events unobtrusively using `data-*` attributes and handle potential request failures gracefully:

```html
<!-- views/users.hbs -->
<!doctype html>
<html lang="en">
	<head>
		<meta charset="UTF-8" />
		<title>User Directory</title>
	</head>
	<body>
		<h1>User Directory</h1>

		<!-- Creation Form -->
		<form action="/users/create" method="POST">
			<input type="text" name="username" placeholder="Username" required />
			<input type="number" name="age" placeholder="Age" min="1" step="1" required />
			<button type="submit">Add User</button>
		</form>

		<hr />

		<!-- Presentation Logic: Displaying the user list -->
		<ul>
			{{#each users}}
			<li>
				<strong>{{this.username}}</strong> (Age: {{this.age}})
				<button type="button" data-user-id="{{this.id}}">Delete</button>
			</li>
			{{/each}}
		</ul>

		<script>
			document.querySelectorAll("[data-user-id]").forEach((button) => {
				button.addEventListener("click", async () => {
					const id = button.dataset.userId;
					try {
						const response = await fetch(`/users/remove?id=${encodeURIComponent(id)}`, {
							method: "DELETE",
						});

						if (!response.ok) {
							throw new Error("Unable to delete user.");
						}

						window.location.reload();
					} catch (error) {
						alert(error instanceof Error ? error.message : "Unexpected error occurred.");
					}
				});
			});
		</script>
	</body>
</html>
```

---

#### The Controller Layer (HTTP Orchestration) <a name="The Controller Layer (HTTP Orchestration)"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

The Controller intercepts incoming HTTP requests, performs parameter extraction and structural validation, queries the Model, and selects the View:

```typescript
import { Request, Response } from "express";
import { DuplicateUsernameError, UserModel } from "./user.model";

export class UserController {
	// GET /users
	public static getAllUsers(req: Request, res: Response): void {
		const users = UserModel.getAll();
		res.render("users", { users });
	}

	// POST /users/create
	public static createUser(req: Request, res: Response): void {
		const username = req.body.username;
		const age = Number(req.body.age);

		// Controller validation: structural verification of request payload
		if (typeof username !== "string" || !username.trim() || !Number.isInteger(age) || age <= 0) {
			res.status(400).render("users-error", {
				message: "Username and a valid positive integer age are required.",
			});
			return;
		}

		try {
			UserModel.create(username, age);
			// Post-Redirect-Get pattern
			res.redirect("/users");
		} catch (err: unknown) {
			if (err instanceof DuplicateUsernameError) {
				res.status(409).render("users-error", { message: err.message });
				return;
			}

			const message = err instanceof Error ? err.message : "An unexpected error occurred.";
			res.status(500).render("users-error", { message });
		}
	}

	// DELETE /users/remove?id=...
	public static removeUser(req: Request, res: Response): void {
		const id = req.query.id;

		if (typeof id !== "string" || !id.trim()) {
			res.status(400).json({ error: "Missing or invalid user ID in query parameters." });
			return;
		}

		try {
			UserModel.removeById(id);
			res.status(200).json({ success: true });
		} catch (err: unknown) {
			const message = err instanceof Error ? err.message : "An unexpected error occurred.";
			res.status(404).json({ error: message });
		}
	}
}
```

```typescript
// server.ts - Wiring the routing layer
import express from "express";
import { UserController } from "./user.controller";

const app = express();

// Standard Express body parsing middleware
app.use(express.urlencoded({ extended: true }));
app.use(express.json());
app.set("view engine", "hbs");

app.get("/users", UserController.getAllUsers);
app.post("/users/create", UserController.createUser);
app.delete("/users/remove", UserController.removeUser);

app.listen(5000, () => console.log("Server running on http://localhost:5000"));
```

---

#### Handling Errors and Alternative Views <a name="Handling Errors and Alternative Views"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

When an exceptional condition occurs in the domain layer, the Controller traps the domain exception, chooses an appropriate HTTP status code (`409 Conflict` for duplicates, `404 Not Found` for missing resources, `500` for unexpected errors), and routes the failure to an alternative error view (`users-error.hbs`):

```html
<!-- views/users-error.hbs -->
<!doctype html>
<html lang="en">
	<head>
		<meta charset="UTF-8" />
		<title>Operation Error</title>
	</head>
	<body>
		<h2>Something went wrong</h2>
		<p style="color: red;">{{message}}</p>
		<a href="/users">Return to User Directory</a>
	</body>
</html>
```

---

#### Multi-Client Adaptability: HTML, REST, GraphQL, and SOAP <a name="Multi-Client Adaptability: HTML, REST, GraphQL, and SOAP"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

Because `UserModel` contains pure domain logic and is completely independent of Express or HTML templates, different client consumers can interact with the same core business engine:

```
                          +-----------------+
                          |    UserModel    |
                          | (Domain Engine) |
                          +-----------------+
                           /       |       \
                          /        |        \
                         v         v         v
               [HTML Controller] [REST API] [GraphQL Resolver]
                     |                 |            |
                     v                 v            v
                HTML Pages         JSON Data    Graph Schema
             (Desktop Web)       (Mobile App)  (Integrations)
```

1. **HTML Controller:** Serves server-rendered Handlebars templates for web browsers.
2. **REST Controller:** Returns structured JSON responses for mobile applications and third-party partners.
3. **GraphQL Resolver:** Maps GraphQL queries and mutations to underlying application operations.
4. **SOAP Endpoint:** Serializes domain data into XML envelopes for legacy enterprise clients.

These interfaces can reuse the same domain and application logic, although their request-handling components do not necessarily follow traditional MVC Controller semantics (for example, GraphQL resolvers map fields directly to application services rather than rendering view templates).

---

#### Production Considerations: Security, Transactions, and Layering <a name="Production Considerations: Security, Transactions, and Layering"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

While the example above demonstrates classic client-server MVC, a production-grade system introduces several crucial layers:

-   **Persistence & Transactions:** In-memory arrays disappear upon server restart. Production models delegate persistence to database repositories using transactions to ensure atomicity during concurrent operations.
-   **CSRF Protection:** State-changing requests authenticated through cookies require appropriate anti-forgery protections. Depending on the application architecture, this may involve CSRF tokens, appropriately configured SameSite cookies, origin validation, or a combination of these mechanisms.
-   **Authentication & Authorization:** Controllers typically run after authentication middlewares, verifying that the requesting user possesses the permissions required to execute the action.
-   **Centralized Error Handling:** Rather than catching errors individually in every controller method, unhandled exceptions are forwarded to centralized Express error middleware (`(err, req, res, next) => ...`) which logs internal diagnostics and prevents leaking sensitive stack traces to clients.

---

## MVC on the Modern Frontend (TypeScript) <a name="MVC on the Modern Frontend (TypeScript)"></a>

Modern frontend applications have evolved to possess their own substantial business logic, complex client-side state, caching, and dynamic rendering.

Let's explore three implementation techniques commonly used when building MVC-inspired frontend applications in framework-independent TypeScript.

---

#### Technique 1: Controller as Pure Mediator (Decoupled Flow) <a name="Technique 1: Controller as Pure Mediator (Decoupled Flow)"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

In this classic pattern, the **View and Model have zero knowledge of each other**. The Controller acts as a pure mediator: it handles UI events, invokes the Model, queries state, and pushes updates to the View.

```
 [ View ]  --- 1. User Click (Event) --->  [ Controller ]
    ^                                            |
    |                                            | 2. Mutates State
    |                                            v
    |                                        [ Model ]
    |                                            |
    +---- 4. Updates Title (New Value) <--------+ 3. Returns New Value
```

```typescript
// 1. Model: Pure state & calculations
class CounterModel {
	private value: number;

	constructor(initialValue: number = 0) {
		this.value = initialValue;
	}

	public increment(): number {
		return ++this.value;
	}

	public decrement(): number {
		return --this.value;
	}

	public multiply(factor: number): number {
		this.value *= factor;
		return this.value;
	}

	public getValue(): number {
		return this.value;
	}
}

// 2. Controller: Mediator between View & Model
class CounterController {
	private model: CounterModel;
	private view!: CounterView;

	constructor(model: CounterModel) {
		this.model = model;
	}

	public setView(view: CounterView): void {
		this.view = view;
	}

	public getCurrentValue(): number {
		return this.model.getValue();
	}

	public handleIncrement(): void {
		const newValue = this.model.increment();
		this.view.updateTitle(newValue);
	}

	public handleDecrement(): void {
		const newValue = this.model.decrement();
		this.view.updateTitle(newValue);
	}

	public handleMultiply(factor: number): void {
		const newValue = this.model.multiply(factor);
		this.view.updateTitle(newValue);
	}
}

// 3. View: Presentation & DOM manipulation
class CounterView {
	private root: HTMLElement;
	private controller: CounterController;
	private titleElement!: HTMLHeadingElement;

	constructor(root: HTMLElement, controller: CounterController) {
		this.root = root;
		this.controller = controller;
		this.controller.setView(this);
	}

	public mount(): void {
		this.titleElement = document.createElement("h2");
		// Initializes display with actual Model state via Controller
		this.titleElement.textContent = `Count: ${this.controller.getCurrentValue()}`;

		const incBtn = document.createElement("button");
		incBtn.textContent = "+1";
		incBtn.onclick = () => this.controller.handleIncrement();

		const decBtn = document.createElement("button");
		decBtn.textContent = "-1";
		decBtn.onclick = () => this.controller.handleDecrement();

		const mulBtn = document.createElement("button");
		mulBtn.textContent = "x2";
		mulBtn.onclick = () => this.controller.handleMultiply(2);

		this.root.append(this.titleElement, incBtn, decBtn, mulBtn);
	}

	public updateTitle(value: number): void {
		this.titleElement.textContent = `Count: ${value}`;
	}
}
```

---

#### Technique 2: Model Notifies View (Observer Pattern) <a name="Technique 2: Model Notifies View (Observer Pattern)"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

In the second technique, user actions travel from the View to the Controller, and the Controller invokes operations on the Model. However, instead of passing values back through the Controller, the Model directly notifies registered Views via an observer subscription that its state has changed.

```
 [ View ]  --- 1. User Action --->  [ Controller ]
    ^                                    |
    |                                    | 2. Updates Data
    |                                    v
    +----- 3. Direct State Change <--- [ Model ]
           (Observer Event / Callback)
```

```typescript
type Listener = () => void;

class ObservableCounterModel {
	private value: number;
	private listeners: Listener[] = [];

	constructor(initialValue: number = 0) {
		this.value = initialValue;
	}

	public subscribe(listener: Listener): () => void {
		this.listeners.push(listener);
		return () => {
			this.listeners = this.listeners.filter((fn) => fn !== listener);
		};
	}

	private notify(): void {
		this.listeners.forEach((fn) => {
			try {
				fn();
			} catch (err) {
				console.error("Error in listener:", err);
			}
		});
	}

	public increment(): void {
		this.value++;
		this.notify();
	}

	public getValue(): number {
		return this.value;
	}
}

class ObservableCounterController {
	constructor(private model: ObservableCounterModel) {}

	public handleIncrement(): void {
		this.model.increment();
	}
}

class ObserverCounterView {
	private readonly unsubscribe: () => void;

	constructor(
		private readonly root: HTMLElement,
		private readonly model: ObservableCounterModel,
		private readonly controller: ObservableCounterController,
	) {
		this.unsubscribe = this.model.subscribe(() => this.render());
		this.render();
	}

	public render(): void {
		this.root.innerHTML = `
      <h2>Count: ${this.model.getValue()}</h2>
      <button id="inc-btn">+1</button>
    `;

		this.root.querySelector("#inc-btn")!.onclick = () => this.controller.handleIncrement();
	}

	public destroy(): void {
		this.unsubscribe();
		this.root.replaceChildren();
	}
}
```

---

#### Technique 3: Dependency Injection and Component Assembly <a name="Technique 3: Dependency Injection and Component Assembly"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

For larger frontend modules, we assemble components using **Dependency Injection (DI)** at the composition root. Crucially, the Controller must not contain browser presentation APIs (such as `alert()` or `prompt()`), returning structured results instead so the View retains full control over display:

```typescript
export type CreateUserResult = { success: true } | { success: false; error: string };

class UserDirectoryController {
	constructor(private model: UserDirectoryModel) {}

	public handleCreateUser(username: string, ageStr: string): CreateUserResult {
		const age = Number(ageStr);

		if (!username.trim() || !Number.isInteger(age) || age <= 0) {
			return {
				success: false,
				error: "Please provide a non-empty username and a positive integer age.",
			};
		}

		this.model.addUser(username.trim(), age);
		return { success: true };
	}

	public handleSortChange(field: "username" | "age", order: "asc" | "desc"): void {
		this.model.sortUsers(field, order);
	}
}
```

```typescript
// Assembling the module at composition root:
const rootElement = document.getElementById("user-app")!;
const model = new UserDirectoryModel();
const controller = new UserDirectoryController(model);
const view = new UserDirectoryView(rootElement, controller);

view.mount();
```

With clearly defined interfaces and no dependencies on rendering APIs, the domain Model can remain unchanged when replacing the presentation technology. The Controller may also remain unchanged if the new View can adapt its interactions to the Controller's existing interface; otherwise, only the presentation adapter or integration layer needs modification.

---

## Exploring the Broader MV* Family: MVVM <a name="Exploring the Broader MV* Family: MVVM"></a>

While MVC originated in desktop computing and became the standard for server-side web development, other variations evolved specifically to tackle complex graphical user interfaces.

#### Model-View-ViewModel (MVVM) and Its Origins <a name="Model-View-ViewModel (MVVM) and Its Origins"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

**MVVM** was introduced and popularized by John Gossman in 2005, in the context of Microsoft's Windows Presentation Foundation (WPF). It evolved from earlier presentation patterns (such as Martin Fowler's Presentation Model) and was designed to facilitate clean separation between visual interfaces and presentation state.

In MVVM:

-   **Model:** Represents application data, domain entities, and business operations.
-   **View:** The visual structure and user interface (defined in XAML, HTML templates, or JSX).
-   **ViewModel:** An abstraction of the View that exposes presentation state, computed properties, and executable commands. It transforms domain data into display-ready information and coordinates presentation-related operations without depending on concrete UI controls or rendering technologies.

```
+----------+      Declarative Data Binding      +---------------+                +-----------+
|   View   |  <==============================>  |   ViewModel   |  <-----------> |   Model   |
|  (HTML)  |      (Inputs & State Sync)         | (State/Props) |  (Domain Ops)  | (Entities)|
+----------+                                    +---------------+                +-----------+
```

---

#### Declarative Data Binding and the ViewModel Abstraction <a name="Declarative Data Binding and the ViewModel Abstraction"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

A common characteristic of MVVM frameworks is **declarative data binding**, which can be implemented using one-way or two-way synchronization:

-   The **ViewModel** exposes presentation state, computed values, and actions.
-   The **framework's binding engine** coordinates synchronization between the View and the ViewModel.
-   When the user modifies an input field, the bound ViewModel property updates. Conversely, when asynchronous background operations update ViewModel state, the corresponding UI controls reactively update without manual DOM querying or text assignment.

Historically, libraries such as **Knockout.js** popularized the MVVM pattern in web development by providing observable properties, declarative bindings, and computed values. In contemporary web ecosystems, frameworks like **Vue.js** embody MVVM-inspired ergonomics.

---

#### Vue.js: An MVVM-Inspired Implementation <a name="Vue.js: An MVVM-Inspired Implementation"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

While Vue.js is a component-oriented framework rather than a dogmatic MVVM implementation, its component structure strongly reflects MVVM principles:

```html
<!-- UserCard.vue -->
<template>
	<div class="user-card">
		<!-- Declarative binding to ViewModel state -->
		<input v-model="username" placeholder="Enter username" />

		<!-- Synchronized computed presentation state -->
		<p>Current User: {{ formattedUsername }}</p>

		<!-- Command execution bound to ViewModel method -->
		<button @click="saveUser">Save to Database</button>
	</div>
</template>

<script>
	import { UserService } from "./user.service";

	export default {
		data() {
			// The ViewModel State
			return {
				username: "",
			};
		},
		computed: {
			// View-specific data transformation
			formattedUsername() {
				return this.username.trim().toUpperCase() || "Anonymous";
			},
		},
		methods: {
			// Action delegating to domain service
			saveUser() {
				UserService.save({ username: this.username });
			},
		},
	};
</script>
```

In this setup, developers declare reactive state and computed properties on the component instance (the ViewModel), and Vue's reactivity system automatically coordinates synchronization with the template (the View).

---

#### When to Choose MVC vs MVVM <a name="When to Choose MVC vs MVVM"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

To choose the right pattern, compare how each handles key architectural responsibilities:

1. **Main Intermediary:**
    - **MVC:** The Controller acts as an operational coordinator that receives user requests, invokes domain logic, and selects views.
    - **MVVM:** The ViewModel acts as a stateful abstraction of the View, exposing display-ready properties and executable commands.
2. **User Interaction Flow:**
    - **MVC:** Routed through the Controller via HTTP endpoints or DOM event listeners.
    - **MVVM:** Handled through declarative bindings and commands declared in the markup.
3. **Presentation State Management:**
    - **MVC:** Coordinated explicitly by the Controller or held within the Model.
    - **MVVM:** Directly encapsulated by the ViewModel, separating visual state from domain models.
4. **Data Synchronization:**
    - **MVC:** Typically explicit and procedural in server-side web apps, or event-driven via observer subscriptions in client implementations.
    - **MVVM:** Automated through the framework's reactive data-binding system.
5. **View & Model Coupling:**
    - **MVC:** In controller-mediated MVC, View and Model remain completely independent. In observer-based MVC, the View observes the Model directly.
    - **MVVM:** The View is declaratively bound to the ViewModel's public API, while the domain Model remains unaware of UI state.
6. **Automated Testing:**
    - **MVC:** Models and Controllers can be tested independently with standard headless unit test runners.
    - **MVVM:** ViewModels can be tested as pure state objects without mounting a browser DOM or rendering engine.

---

## Summary & Key Takeaways <a name="Summary & Key Takeaways"></a>

<p align="right">(<a href="#toc">back to contents</a>)</p>

1. **Separation of Concerns is Foundational:** The presentation layer changes frequently due to marketing redesigns, styling updates, or multi-platform targets. Business rules change based on domain requirements. Blending them makes code brittle and expensive to maintain.
2. **Domain Models Should Remain Independent:** Domain models should remain independent of presentation technologies such as HTTP controllers, HTML templates, DOM trees, and UI widgets. Persistence and infrastructure dependencies should be managed according to the application's architectural boundaries.
3. **Keep Controllers Focused:** Controllers should primarily handle request orchestration—validating input parameters, invoking domain operations, and selecting the response representation. Business rules should not be duplicated across controllers.
4. **Architectural Flexibility:** A cleanly isolated domain model can power an HTML template controller, a JSON REST API, and a GraphQL endpoint concurrently without duplicating business logic.
5. **MV\* Evolution:** Whether using classic MVC in server systems or leveraging reactive bindings with ViewModels on the frontend, the core objective remains constant: **clear boundaries, predictable data flows, and decoupled, testable code.**
