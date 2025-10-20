import {Extension, EditorState} from "@codemirror/state"
import {
    EditorView, keymap, drawSelection,
    highlightActiveLine, dropCursor, rectangularSelection,
    crosshairCursor, lineNumbers, highlightActiveLineGutter
} from "@codemirror/view"
import {
    bracketMatching, indentUnit
} from "@codemirror/language"
import {
    defaultKeymap, history, historyKeymap, indentWithTab
} from "@codemirror/commands"

export function createMinimalEditor(parentElement) {
    const view = new EditorView({
        parent: parentElement,
        extensions: [
            // Allow multiple cursors/selections
            EditorState.allowMultipleSelections.of(true),
            // Set tab to be 4 spaces, not 2
            indentUnit.of("    "),
            // A line number gutter
            lineNumbers(),
            // The undo history
            history(),
            // Replace native cursor/selection with our own
            drawSelection(),
            // Show a drop cursor when dragging over the editor
            dropCursor(),
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
            keymap.of([
                indentWithTab,
                // A large set of basic bindings
                ...defaultKeymap,
                // Redo/undo keys
                ...historyKeymap,
            ])
        ]
    })
    return view;
}
