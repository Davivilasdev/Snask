"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const path = __importStar(require("path"));
const fs = __importStar(require("fs"));
const child_process_1 = require("child_process");
// Centralized definition for language features (keywords, functions)
const snaskLanguageFeatures = [
    { label: 'fun', kind: vscode.CompletionItemKind.Keyword, description: 'Declares a new function.', example: 'fun my_add(a, b) {\n\treturn a + b;\n}' },
    { label: 'let', kind: vscode.CompletionItemKind.Keyword, description: 'Declares an immutable variable.', example: 'let x = 10;' },
    { label: 'mut', kind: vscode.CompletionItemKind.Keyword, description: 'A modifier used with `let` to declare a mutable (changeable) variable.', example: 'let mut y = 20;\ny = 30; // Valid' },
    { label: 'const', kind: vscode.CompletionItemKind.Keyword, description: 'Declares a constant whose value is fixed.', example: 'const PI = 3.14159;' },
    { label: 'if', kind: vscode.CompletionItemKind.Keyword, description: 'Executes a code block if its condition is true.', example: 'if x > 5 {\n\tprint("x is large");\n}' },
    { label: 'else', kind: vscode.CompletionItemKind.Keyword, description: 'Executes a code block if the preceding `if` or `elif` condition is false.' },
    { label: 'elif', kind: vscode.CompletionItemKind.Keyword, description: 'Checks a new condition if a preceding `if` was false.', example: 'if x > 10 {\n\t//...\n} elif x > 5 {\n\t//...\n}' },
    { label: 'while', kind: vscode.CompletionItemKind.Keyword, description: 'Creates a loop that executes as long as a condition is true.', example: 'while i < 10 {\n\ti = i + 1;\n}' },
    { label: 'for', kind: vscode.CompletionItemKind.Keyword, description: 'Loops over an iterable (e.g., a list).', example: 'for item in my_list {\n\tprint(item);\n}' },
    { label: 'return', kind: vscode.CompletionItemKind.Keyword, description: 'Exits a function, optionally returning a value.', example: 'return 10;' },
    { label: 'print', kind: vscode.CompletionItemKind.Function, description: 'A built-in function that prints a value to the standard output.', example: 'print("Hello, World!");' },
    { label: 'input', kind: vscode.CompletionItemKind.Function, description: 'A built-in function that reads a line of text from standard input.', example: 'let name = input("Enter your name: ");' },
];
function activate(context) {
    console.log('Congratulations, your extension "snask-lang" is now active!');
    // 1. Command to run the current file
    context.subscriptions.push(vscode.commands.registerCommand('snask-lang.runFile', () => runSnaskFile()));
    // 2. Diagnostics Logic
    const diagnosticsCollection = vscode.languages.createDiagnosticCollection('snask');
    context.subscriptions.push(diagnosticsCollection);
    let debounceTimer;
    const triggerUpdateDiagnostics = (document) => {
        if (document.languageId !== 'snask')
            return;
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(() => updateDiagnostics(document, diagnosticsCollection), 300);
    };
    if (vscode.window.activeTextEditor) {
        triggerUpdateDiagnostics(vscode.window.activeTextEditor.document);
    }
    context.subscriptions.push(vscode.window.onDidChangeActiveTextEditor(editor => editor && triggerUpdateDiagnostics(editor.document)));
    context.subscriptions.push(vscode.workspace.onDidChangeTextDocument(event => triggerUpdateDiagnostics(event.document)));
    context.subscriptions.push(vscode.workspace.onDidCloseTextDocument(doc => diagnosticsCollection.delete(doc.uri)));
    // 3. Hover Provider
    context.subscriptions.push(vscode.languages.registerHoverProvider('snask', createHoverProvider()));
    // 4. Completion Item Provider
    context.subscriptions.push(vscode.languages.registerCompletionItemProvider('snask', createCompletionItemProvider()));
    // 5. Document Formatting Provider
    context.subscriptions.push(vscode.languages.registerDocumentFormattingEditProvider('snask', createFormattingProvider()));
    // 6. AI-Powered "Explain Code" Command
    context.subscriptions.push(vscode.commands.registerCommand('snask.explainCode', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.selection.isEmpty) {
            vscode.window.showInformationMessage('Please select a block of Snask code to explain.');
            return;
        }
        const text = editor.document.getText(editor.selection);
        // Simple analysis to simulate AI explanation
        let analysis = 'Este trecho de código Snask ';
        const funcs = (text.match(/fun/g) || []).length;
        if (funcs > 0)
            analysis += `define ${funcs} função(ões). `;
        if (text.includes('if') || text.includes('while') || text.includes('for'))
            analysis += 'Usa estruturas de controle de fluxo (loops ou condicionais). ';
        if (text.includes('let') || text.includes('const'))
            analysis += 'Realiza a declaração de variáveis ou constantes. ';
        if (text.includes('return'))
            analysis += 'Provavelmente retorna um valor de uma função. ';
        const lineCount = editor.selection.end.line - editor.selection.start.line + 1;
        analysis += `\n\n- O bloco selecionado tem ${lineCount} linha(s).`;
        vscode.window.showInformationMessage('Snask AI Explica', {
            modal: true,
            detail: analysis
        });
    }));
}
function runSnaskFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'snask') {
        vscode.window.showInformationMessage('No active Snask file to run!');
        return;
    }
    const snaskExecutable = getSnaskExecutablePath();
    if (!snaskExecutable) {
        vscode.window.showErrorMessage(`Snask executable not found. Please set 'snask.executablePath' in your settings.`);
        return;
    }
    const filePath = editor.document.fileName;
    const terminal = vscode.window.createTerminal(`Snask: ${path.basename(filePath)}`);
    terminal.show();
    terminal.sendText(`"${snaskExecutable}" interpret "${filePath}"`);
}
function getSnaskExecutablePath() {
    const config = vscode.workspace.getConfiguration('snask');
    const executablePath = config.get('executablePath');
    if (executablePath && fs.existsSync(executablePath))
        return executablePath;
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders === null || workspaceFolders === void 0 ? void 0 : workspaceFolders.length) {
        const defaultPath = path.join(workspaceFolders[0].uri.fsPath, 'executor', 'snask.exe');
        if (fs.existsSync(defaultPath))
            return defaultPath;
    }
    return null;
}
function updateDiagnostics(document, collection) {
    const snaskExecutable = getSnaskExecutablePath();
    if (!snaskExecutable) {
        collection.clear();
        return;
    }
    (0, child_process_1.execFile)(snaskExecutable, ['interpret', document.fileName], (error, stdout, stderr) => {
        const diagnostics = [];
        const errorRegex = /Error on line (\d+), col (\d+): (.*)/g;
        let match;
        while ((match = errorRegex.exec(stderr)) !== null) {
            const line = parseInt(match[1], 10) - 1;
            const column = parseInt(match[2], 10) - 1;
            const message = match[3];
            const range = document.getWordRangeAtPosition(new vscode.Position(line, column)) || new vscode.Range(line, column, line, document.lineAt(line).text.length);
            diagnostics.push(new vscode.Diagnostic(range, message, vscode.DiagnosticSeverity.Error));
        }
        collection.set(document.uri, diagnostics);
    });
}
function createHoverProvider() {
    const hoverMap = new Map();
    for (const feature of snaskLanguageFeatures) {
        const mdString = new vscode.MarkdownString().appendMarkdown(`**${feature.label}**\n\n---\n\n${feature.description}`);
        if (feature.example) {
            mdString.appendCodeblock(feature.example, 'snask');
        }
        hoverMap.set(feature.label, mdString);
    }
    return {
        provideHover(document, position) {
            const range = document.getWordRangeAtPosition(position);
            const word = range ? document.getText(range) : '';
            return hoverMap.has(word) ? new vscode.Hover(hoverMap.get(word)) : null;
        }
    };
}
function createCompletionItemProvider() {
    const completionItems = snaskLanguageFeatures.map(feature => {
        const item = new vscode.CompletionItem(feature.label, feature.kind);
        const mdString = new vscode.MarkdownString().appendMarkdown(feature.description);
        if (feature.example) {
            mdString.appendCodeblock(feature.example, 'snask');
        }
        item.documentation = mdString;
        return item;
    });
    return {
        provideCompletionItems: () => completionItems
    };
}
function createFormattingProvider() {
    return {
        provideDocumentFormattingEdits(document) {
            const edits = [];
            let indentLevel = 0;
            const editorConfig = vscode.workspace.getConfiguration('editor');
            const insertSpaces = editorConfig.get('insertSpaces', true);
            const tabSize = editorConfig.get('tabSize', 4);
            const indentChar = insertSpaces ? ' '.repeat(tabSize) : '\t';
            for (let i = 0; i < document.lineCount; i++) {
                const line = document.lineAt(i);
                if (line.isEmptyOrWhitespace)
                    continue;
                const trimmedLine = line.text.trim();
                if (trimmedLine.startsWith('}') || trimmedLine.startsWith(']')) {
                    indentLevel = Math.max(0, indentLevel - 1);
                }
                const currentIndentLength = line.firstNonWhitespaceCharacterIndex;
                const correctIndent = indentChar.repeat(indentLevel);
                if (currentIndentLength !== correctIndent.length || line.text.substring(0, currentIndentLength) !== correctIndent) {
                    edits.push(vscode.TextEdit.replace(new vscode.Range(i, 0, i, currentIndentLength), correctIndent));
                }
                if (trimmedLine.endsWith('{') || trimmedLine.endsWith('[')) {
                    indentLevel++;
                }
            }
            return edits;
        }
    };
}
function deactivate() { }
//# sourceMappingURL=extension.js.map