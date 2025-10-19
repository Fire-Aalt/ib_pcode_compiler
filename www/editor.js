// import { EditorState } from "@codemirror/state";
// import { EditorView, keymap } from "@codemirror/view";
// import { defaultKeymap, indentWithTab } from "@codemirror/commands";
// import { lineNumbers } from "@codemirror/gutter";
// import { bracketMatching, closeBrackets } from "@codemirror/language";
//
// export function createMinimalEditor(parentElement, initialText = "") {
//     const state = EditorState.create({
//         doc: initialText,
//         extensions: [
//             keymap.of([indentWithTab, ...defaultKeymap]),
//             lineNumbers(),
//             bracketMatching(),
//             closeBrackets()
//         ]
//     });
//
//     const view = new EditorView({
//         state,
//         parent: parentElement
//     });
//
//     return view;
// }
//
// const container = document.getElementById("editor");
// const view = createMinimalEditor(container, `fn main() {\n  println!("Hello");\n}`);