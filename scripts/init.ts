import { existsSync } from "node:fs";
import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import inquirer from "inquirer";
import { logger } from "rslog";
import { rootDir } from "./shared.js";

async function main() {
	logger.greet("Welcome to the UnRust Init Script!");

	const { name } = await inquirer.prompt<{ name: string }>([
		{
			type: "input",
			name: "name",
			message: "Enter the name of the project",
		},
	]);

	const crateName = name.replace(/-/g, "_");

	for (const file of ["Cargo.toml", "README.md", "package.json"]) {
		const path = join(rootDir, file);
		if (!existsSync(path)) continue;
		let content = await readFile(path, "utf-8");
		content = content.replaceAll("package-name", name);
		await writeFile(path, content);
	}

	for (const rel of ["src/main.rs", "tests/integration_test.rs"]) {
		const path = join(rootDir, rel);
		if (!existsSync(path)) continue;
		const content = await readFile(path, "utf-8");
		await writeFile(path, content.replaceAll("package_name", crateName));
	}

	const securityPath = join(rootDir, ".github", "SECURITY.md");
	if (existsSync(securityPath)) {
		const content = await readFile(securityPath, "utf-8");
		await writeFile(securityPath, content.replaceAll("package_name", name));
	}

	// clean the changelog
	await writeFile(join(rootDir, "CHANGELOG.md"), "");

	// reset version in Cargo.toml
	const cargoPath = join(rootDir, "Cargo.toml");
	if (existsSync(cargoPath)) {
		let cargo = await readFile(cargoPath, "utf-8");
		cargo = cargo.replace(/^version\s*=\s*"[^"]*"/m, 'version = "1.0.0"');
		await writeFile(cargoPath, cargo);
	}

	logger.success(
		`Replaced "package-name" with "${name}", crate name "${crateName}" in src and tests; cleared CHANGELOG.md; reset Cargo.toml version to 1.0.0.`,
	);
}

main().catch((err) => {
	logger.error(err);
	process.exit(1);
});
