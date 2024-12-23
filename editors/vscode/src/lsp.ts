import * as vscode from "vscode";
import { LanguageClient, SymbolInformation } from "vscode-languageclient/node";
import { spawnSync } from "child_process";
import { resolve } from "path";

export let client: LanguageClient | undefined = undefined;

export function setClient(newClient: LanguageClient) {
  client = newClient;
  clientPromiseResolve(newClient);
}

let clientPromiseResolve = (_client: LanguageClient) => {};
let clientPromise: Promise<LanguageClient> = new Promise((resolve) => {
  clientPromiseResolve = resolve;
});
export async function getClient(): Promise<LanguageClient> {
  return clientPromise;
}

export interface PackageInfo {
  path: string;
  namespace: string;
  name: string;
  version: string;
}

export interface SymbolInfo {
  name: string;
  kind: string;
  children: SymbolInfo[];
}

interface ResourceRoutes {
  "/symbols": any;
  "/preview/index.html": string;
  "/dir/package": string;
  "/dir/package/local": string;
  "/package/by-namespace": PackageInfo[];
  "/package/symbol": SymbolInfo;
  "/package/docs": string;
}

export const tinymist = {
  getClient,
  probeEnvPath,
  probePaths,
  exportPdf: exportCommand("tinymist.exportPdf"),
  exportSvg: exportCommand("tinymist.exportSvg"),
  exportPng: exportCommand("tinymist.exportPng"),
  exportHtml: exportCommand("tinymist.exportHtml"),
  exportMarkdown: exportCommand("tinymist.exportMarkdown"),
  exportText: exportCommand("tinymist.exportText"),
  exportQuery: exportCommand("tinymist.exportQuery"),
  exportAnsiHighlight: exportCommand("tinymist.exportAnsiHighlight"),
  async executeCommand<R>(command: string, args: any[]) {
    return await (
      await getClient()
    ).sendRequest<R>("workspace/executeCommand", {
      command,
      arguments: args,
    });
  },
  getResource<T extends keyof ResourceRoutes>(path: T, ...args: any[]) {
    return tinymist.executeCommand<ResourceRoutes[T]>("tinymist.getResources", [path, ...args]);
  },
  getWorkspaceLabels() {
    return tinymist.executeCommand<SymbolInformation[]>("tinymist.getWorkspaceLabels", []);
  },
  showLog() {
    if (client) {
      client.outputChannel.show();
    }
  },

  /**
   * The code is borrowed from https://github.com/rust-lang/rust-analyzer/blob/fc98e0657abf3ce07eed513e38274c89bbb2f8ad/editors/code/src/config.ts#L98
   * Last checked time: 2024-11-14
   *
   * Sets up additional language configuration that's impossible to do via a
   * separate language-configuration.json file. See [1] for more information.
   *
   * [1]: https://github.com/Microsoft/vscode/issues/11514#issuecomment-244707076
   */
  configureLang: undefined as vscode.Disposable | undefined,
  configureLanguage(typingContinueCommentsOnNewline: boolean) {
    // Only need to dispose of the config if there's a change
    if (this.configureLang) {
      this.configureLang.dispose();
      this.configureLang = undefined;
    }

    let onEnterRules: vscode.OnEnterRule[] = [
      {
        // Carry indentation from the previous line
        // if it's only whitespace
        beforeText: /^\s+$/,
        action: { indentAction: vscode.IndentAction.None },
      },
      {
        // After the end of a function/field chain,
        // with the semicolon on the same line
        beforeText: /^\s+\..*;/,
        action: { indentAction: vscode.IndentAction.Outdent },
      },
      {
        // After the end of a function/field chain,
        // with semicolon detached from the rest
        beforeText: /^\s+;/,
        previousLineText: /^\s+\..*/,
        action: { indentAction: vscode.IndentAction.Outdent },
      },
    ];

    if (typingContinueCommentsOnNewline) {
      const indentAction = vscode.IndentAction.None;

      onEnterRules = [
        ...onEnterRules,
        {
          // Doc single-line comment
          // e.g. ///|
          beforeText: /^\s*\/{3}.*$/,
          action: { indentAction, appendText: "/// " },
        },
        {
          // Parent doc single-line comment
          // e.g. //!|
          beforeText: /^\s*\/{2}\!.*$/,
          action: { indentAction, appendText: "//! " },
        },
        {
          // Begins an auto-closed multi-line comment (standard or parent doc)
          // e.g. /** | */ or /*! | */
          beforeText: /^\s*\/\*(\*|\!)(?!\/)([^\*]|\*(?!\/))*$/,
          afterText: /^\s*\*\/$/,
          action: {
            indentAction: vscode.IndentAction.IndentOutdent,
            appendText: " * ",
          },
        },
        {
          // Begins a multi-line comment (standard or parent doc)
          // e.g. /** ...| or /*! ...|
          beforeText: /^\s*\/\*(\*|\!)(?!\/)([^\*]|\*(?!\/))*$/,
          action: { indentAction, appendText: " * " },
        },
        {
          // Continues a multi-line comment
          // e.g.  * ...|
          beforeText: /^(\ \ )*\ \*(\ ([^\*]|\*(?!\/))*)?$/,
          action: { indentAction, appendText: "* " },
        },
        {
          // Dedents after closing a multi-line comment
          // e.g.  */|
          beforeText: /^(\ \ )*\ \*\/\s*$/,
          action: { indentAction, removeText: 1 },
        },
      ];
    }

    console.log("Setting up language configuration", typingContinueCommentsOnNewline);
    this.configureLang = vscode.languages.setLanguageConfiguration("typst", {
      onEnterRules,
    });
  },
};

/// kill the probe task after 60s
const PROBE_TIMEOUT = 60_000;

function probeEnvPath(configName: string, configPath?: string): string {
  const isWindows = process.platform === "win32";
  const binarySuffix = isWindows ? ".exe" : "";
  const binaryName = "tinymist" + binarySuffix;

  const serverPaths: [string, string][] = configPath
    ? [[`\`${configName}\` (${configPath})`, configPath as string]]
    : [
        ["Bundled", resolve(__dirname, binaryName)],
        ["In PATH", binaryName],
      ];

  return tinymist.probePaths(serverPaths);
}

function probePaths(paths: [string, string][]): string {
  const messages = [];
  for (const [loc, path] of paths) {
    let messageSuffix;
    try {
      const result = spawnSync(path, ["probe"], { timeout: PROBE_TIMEOUT });
      if (result.status === 0) {
        return path;
      }

      const statusMessage = result.status !== null ? [`return status: ${result.status}`] : [];
      const errorMessage =
        result.error?.message !== undefined ? [`error: ${result.error.message}`] : [];
      const messages = [statusMessage, errorMessage];
      messageSuffix = messages.length !== 0 ? `:\n\t${messages.flat().join("\n\t")}` : "";
    } catch (e) {
      if (e instanceof Error) {
        messageSuffix = `: ${e.message}`;
      } else {
        messageSuffix = `: ${JSON.stringify(e)}`;
      }
    }

    messages.push([loc, path, `failed to probe${messageSuffix}`]);
  }

  const infos = messages.map(([loc, path, message]) => `${loc} ('${path}'): ${message}`).join("\n");
  throw new Error(`Could not find a valid tinymist binary.\n${infos}`);
}

function exportCommand(command: string) {
  return (uri: string, extraOpts?: any) => {
    return tinymist.executeCommand<string>(command, [uri, ...(extraOpts ? [extraOpts] : [])]);
  };
}
