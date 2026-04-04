#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { resolve } from "node:path";

/**
 * Returns the executable path which is located inside `node_modules`
 * The naming convention is app-${os}-${arch}
 * If the platform is `win32` or `cygwin`, executable will include a `.exe` extension.
 * @see https://nodejs.org/api/os.html#osarch
 * @see https://nodejs.org/api/os.html#osplatform
 * @example "x/xx/node_modules/app-darwin-arm64"
 */
function getExePath() {
	const appName = "cuid2gen";
	const arch = process.arch;
	let os = process.platform as string;
	let extension = "";
	if (["win32", "cygwin"].includes(process.platform)) {
		os = "windows";
		extension = ".exe";
	}

	try {
		// Since the binary will be located inside `node_modules`, we can simply call `require.resolve`
		return require.resolve(
			`${appName}-${os}-${arch}/bin/${appName}${extension}`,
		);
	} catch {
		const workspaceBinaryPath = resolve(
			__dirname,
			"..",
			"..",
			`${appName}-${os}-${arch}`,
			"bin",
			`${appName}${extension}`,
		);
		if (existsSync(workspaceBinaryPath)) {
			return workspaceBinaryPath;
		}

		throw new Error(
			`Couldn't find application binary inside node_modules for ${os}-${arch}. ` +
				`Supported platforms: linux-x64, linux-arm64, darwin-x64, darwin-arm64, windows-x64, windows-arm64`,
		);
	}
}

/**
 * Runs the application with args using nodejs spawn
 */
function run() {
	const args = process.argv.slice(2);
	const processResult = spawnSync(getExePath(), args, { stdio: "inherit" });

	if (processResult.error) {
		console.error(
			`Failed to execute cuid2gen binary: ${processResult.error.message}`,
		);
		process.exit(1);
	}

	if (processResult.signal) {
		console.error(
			`cuid2gen process terminated by signal ${processResult.signal}`,
		);
		process.exit(1);
	}

	process.exit(processResult.status ?? 1);
}

run();
