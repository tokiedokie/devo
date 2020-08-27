import { DEVO_VERSION } from "./version.ts";

main();

function main() {
  const args = Deno.args;
  if (args[0] === "help" || args[0] === "--help" || args[0] === "-h") {
    help();
  } else if (args[0] === "--version") {
    version();
  } else {
    denoExec(args);
  }
}

function help() {
  console.log("help is not implemented.");
}

async function version() {
  console.log(`devo ${DEVO_VERSION}`);
  await denoRun(["deno", "--version"]);
}

async function denoExec(args: string[]) {
  await denoRun(args);
  console.log("deno exec");
}

async function denoRun(args: string[]) {
  console.log(args)
  await Deno.run({
    cmd: args,
    cwd: Deno.cwd(),
  }).status();
  Deno.
}
