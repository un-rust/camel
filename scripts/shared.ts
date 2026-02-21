import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { parse } from "@iarna/toml";

const scriptDir = import.meta.dirname;
export const rootDir = join(scriptDir, "..");
export const cargoTomlPath = join(rootDir, "Cargo.toml");

export async function getVersionFromCargo(): Promise<string> {
	const content = await readFile(cargoTomlPath, "utf-8");
	const cargo = parse(content) as { package?: { version?: string } };
	const version = cargo.package?.version;
	if (!version) {
		throw new Error("Could not find version in Cargo.toml");
	}
	return version;
}
