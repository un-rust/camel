import { existsSync } from "node:fs";
import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import inquirer from "inquirer";
import { logger } from "rslog";
import { simpleGit } from "simple-git";
import { cargoTomlPath, getVersionFromCargo, rootDir } from "./shared.js";

type BumpType = "major" | "minor" | "patch";

function bumpVersion(version: string, type: BumpType): string {
	const match = version.match(/^(\d+)\.(\d+)\.(\d+)/);
	if (!match) throw new Error(`Invalid semver: ${version}`);

	const major = Number(match[1]);
	const minor = Number(match[2]);
	const patch = Number(match[3]);

	switch (type) {
		case "major":
			return `${major + 1}.0.0`;
		case "minor":
			return `${major}.${minor + 1}.0`;
		case "patch":
			return `${major}.${minor}.${patch + 1}`;
	}
}

function parseBumpArg(): BumpType | null {
	const arg = process.argv.find((a) =>
		["--major", "--minor", "--patch"].includes(a),
	);
	if (!arg) return null;
	return arg.slice(2) as BumpType;
}

async function main() {
	const version = await getVersionFromCargo();

	logger.greet("Welcome to the UnRust Bump Script!");
	logger.info("Current version: %s", version);

	const bumpType = parseBumpArg();
	let newVersion: string;

	if (bumpType) {
		newVersion = bumpVersion(version, bumpType);
		logger.info("Bumping %s → %s", bumpType, newVersion);
	} else {
		const { version: input } = await inquirer.prompt<{ version: string }>([
			{
				type: "input",
				name: "version",
				message: "Enter the new version",
				default: version,
			},
		]);
		newVersion = input;
	}

	const { confirm } = await inquirer.prompt<{ confirm: boolean }>([
		{
			type: "confirm",
			name: "confirm",
			message: `Bump version to ${newVersion} and push?`,
			default: true,
		},
	]);

	if (!confirm) {
		logger.info("Version bump cancelled");
		process.exit(0);
	}

	const content = await readFile(cargoTomlPath, "utf-8");
	// Match version = "x.y.z" or version = 'x.y.z' with optional spacing
	const versionRe = /^(version\s*=\s*)(["'])(?:\d+\.\d+\.\d+)\2/m;
	const newContent = content.replace(versionRe, `$1$2${newVersion}$2`);
	await writeFile(cargoTomlPath, newContent);

	// update .template-version
	const templateVersionPath = join(rootDir, ".template-version");
	if (existsSync(templateVersionPath)) {
		await writeFile(templateVersionPath, newVersion);
	}

	const git = simpleGit();
	await git.add(cargoTomlPath);
	await git.add(templateVersionPath);
	await git.commit(`chore: bump version to ${newVersion}`);
	await git.push();
	await git.addTag(`v${newVersion}`);
	await git.pushTags();

	logger.success("Version bumped and pushed successfully");
}

main().catch((err) => {
	logger.error(err);
	process.exit(1);
});
