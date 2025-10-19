import {Extension, EditorState} from "@codemirror/state"
import {
    EditorView, keymap, drawSelection,
    highlightActiveLine, dropCursor, rectangularSelection,
    crosshairCursor, lineNumbers, highlightActiveLineGutter
} from "@codemirror/view"
import {
    bracketMatching, foldKeymap
} from "@codemirror/language"
import {
    searchKeymap, highlightSelectionMatches
} from "@codemirror/search"
import {
    defaultKeymap, history, historyKeymap, indentWithTab
} from "@codemirror/commands"

// import {
//     closeBrackets,
//     closeBracketsKeymap
// } from "@codemirror/autocomplete"


export function createMinimalEditor(parentElement, initialText = "") {
    const view = new EditorView({
        doc: initialText,
        parent: parentElement,
        extensions: [
            EditorState.tabSize.of(16),
            // A line number gutter
            lineNumbers(),
            // The undo history
            history(),
            // Replace native cursor/selection with our own
            drawSelection(),
            // Show a drop cursor when dragging over the editor
            dropCursor(),
            // Allow multiple cursors/selections
            EditorState.allowMultipleSelections.of(true),
            // Highlight matching brackets near cursor
            bracketMatching(),
            // Allow alt-drag to select rectangular regions
            rectangularSelection(),
            // Change the cursor to a crosshair when holding alt
            crosshairCursor(),
            // Style the current line specially
            highlightActiveLine(),
            // Style the gutter for current line specially
            highlightActiveLineGutter(),
            // Highlight text that matches the selected text
            highlightSelectionMatches(),
            keymap.of([
                indentWithTab,
                // A large set of basic bindings
                ...defaultKeymap,
                // Redo/undo keys
                ...historyKeymap,
                // Code folding bindings
                ...foldKeymap,
            ])
        ]
    })

    return view;
}
