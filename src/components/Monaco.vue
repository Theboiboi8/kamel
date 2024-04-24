<script lang="js">
import loader from "@monaco-editor/loader";

export default {
	name: "Editor",
	async mounted() {

		loader.init().then((monaco) => {
			monaco.editor.defineTheme("kamel-theme", {
				base: 'vs-dark',
				inherit: true,
				rules: [
					{
						token: "comment",
						foreground: '434343',
						fontStyle: "italic"
					},
					{
						token: "string",
						foreground: 'fff48a'
					},
					{
						token: "keyword",
						foreground: 'e91e63'
					},
					{
						token: "identifier",
						foreground: '78dce8'
					},
					{
						token: "operator",
						foreground: '696969'
					},
					{
						token: "number",
						foreground: 'c994ff'
					},
					{
						token: "delimiter.parenthesis",
						foreground: 'e91e63'
					},
					{
						token: "delimiter.curly",
						foreground: 'e91e63'
					},
				],
				colors: {
					"editor.foreground": '#ffffff',
					"editor.background": '#121212',
					"editorCursor.foreground": '#e91e63',
					"editor.lineHighlightBackground": '#050505',
					"editorLineNumber.foreground": '#ffffff',
					"editorLineNumber.activeForeground": '#e91e63',
					"editor.selectionBackground": '#232323',
					"editor.inactiveSelectionBackground": '#151515'
				}
			})

			const editorOptions = {
				value: 'fn main() {\n\tprintln!("Hello, world!"); // Prints "Hello, world!" to the system out\n}',
				language: 'rust',
				minimap: { enabled: false },
				lineNumbers: "on",
				lineHeight: 1.2,
				fontFamily: "JetBrains Mono",
				fontSize: 13,
				roundedSelection: true,
				scrollBeyondLastLine: true,
				readonly: false
			}

			monaco.editor.setTheme('kamel-theme')

			monaco.editor.create(document.getElementById("monaco"), editorOptions);
		});
	},
};
</script>

<template>
	<div id="monaco" class="code_editor"></div>
</template>

<style scoped>
.code_editor {
	width: 100%;
	height: 100%;
	padding: 0;
	margin: 0;
	font-family: "JetBrains Mono", monospace;
	font-size: 13px;
	line-height: 1.2;
}
</style>