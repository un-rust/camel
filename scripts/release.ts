import { execSync } from "node:child_process";
import { existsSync, promises as fsp } from "node:fs";
import {
	generateMarkDown,
	getGitDiff,
	loadChangelogConfig,
	parseCommits,
	resolveGithubToken,
	syncGithubRelease,
} from "changelogen";
import open from "open";
import { logger } from "rslog";
import { simpleGit } from "simple-git";
import { getVersionFromCargo, rootDir } from "./shared.js";

/** Parse "v1.2.3" or "1.2.3" to [1,2,3] for comparison. */
function parseSemver(s: string): number[] {
	const v = s.replace(/^v/, "").match(/^\d+\.\d+\.\d+/);
	return v ? v[0].split(".").map(Number) : [];
}

/** Compare two semver arrays; returns positive if a > b. */
function compareSemver(a: number[], b: number[]): number {
	for (let i = 0; i < 3; i++) {
		const d = (a[i] ?? 0) - (b[i] ?? 0);
		if (d !== 0) return d;
	}
	return 0;
}

/** Get the latest git tag that is strictly older than currentVersion (e.g. v2.0.0 for current 2.0.1). */
function getPreviousTag(
	cwd: string,
	currentVersion: string,
): string | undefined {
	const tags = execSync("git tag -l 'v*'", { cwd, encoding: "utf-8" })
		.trim()
		.split(/\s+/)
		.filter(Boolean);
	const current = parseSemver(currentVersion);
	const older = tags
		.filter((t) => compareSemver(parseSemver(t), current) < 0)
		.sort((a, b) => -compareSemver(parseSemver(a), parseSemver(b)));
	return older[0];
}

async function main() {
	process.chdir(rootDir);
	logger.greet("Welcome to the UnRust Release Script!");

	const version = await getVersionFromCargo();
	logger.info("Current version: %s", version);

	const previousTag = getPreviousTag(rootDir, version);
	const config = await loadChangelogConfig(rootDir, {
		newVersion: version,
		output: "CHANGELOG.md",
		from: previousTag ?? "",
		to: "HEAD",
	});

	logger.info("Generating changelog for %s...%s", config.from || "", config.to);

	const rawCommits = await getGitDiff(config.from, config.to, config.cwd);
	const commits = parseCommits(rawCommits, config)
		.map((c) => ({ ...c, type: c.type.toLowerCase() }))
		.filter(
			(c) =>
				config.types[c.type] &&
				!(
					c.type === "chore" &&
					["deps", "release"].includes(c.scope) &&
					!c.isBreaking
				),
		);

	const markdown = await generateMarkDown(commits, config);
	const body = markdown
		.split("\n")
		.slice(2)
		.join("\n")
		.replaceAll(/\(\[(@.+)\]\(.+\)\)/g, "($1)");

	const release = { version, body };

	if (config.repo?.provider !== "github") {
		logger.error("Release script only supports GitHub repositories.");
		process.exit(1);
	}

	config.tokens.github =
		config.tokens.github ||
		(await resolveGithubToken(config).catch(() => undefined));

	if (typeof config.output === "string") {
		let changelogMD: string;
		if (existsSync(config.output)) {
			changelogMD = await fsp.readFile(config.output, "utf8");
		} else {
			changelogMD = "# Changelog\n\n";
		}
		const lastEntry = changelogMD.match(/^###?\s+.*$/m);
		if (lastEntry) {
			changelogMD =
				changelogMD.slice(0, lastEntry.index) +
				markdown +
				"\n\n" +
				changelogMD.slice(lastEntry.index);
		} else {
			changelogMD += `\n${markdown}\n\n`;
		}
		await fsp.writeFile(config.output, changelogMD);
		logger.success("Updated %s", config.output);
	}

	const git = simpleGit();
	await git.add("CHANGELOG.md");
	await git.add("Cargo.lock");
	await git.commit(`chore: update changelog for version ${version}`);
	await git.push();

	const result = await syncGithubRelease(config, release);

	if (result.status === "manual") {
		const url = result.url;
		if (!url) {
			logger.error("No release URL found");
			process.exit(1);
		}
		logger.info("Opening GitHub release page with pre-filled data...");
		await open(url).catch(() => {
			logger.info("Open this link to create the release manually:\n%o", url);
		});
	} else {
		logger.success("Release v%s synced to GitHub!", version);
	}
}

main().catch((err) => {
	logger.error(err);
	process.exit(1);
});
