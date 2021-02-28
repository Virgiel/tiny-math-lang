import { writable } from 'svelte/store';
import { isArrowKey, isInputKey, isEditKey } from './key';
import { crate } from './wasm';

const defaultCode = `
# Welcome into the Tiny Mathematic Language online editor

# You can perform simple operations
1+1
1+2+3+4+5+6+7+8+9
3*3
(3/8) * (32+4) + 34
2+2
8/(2*(2+2))

# And even use mathematical function
cos(5)
sin(4/8)
log2(1000)

# ----- TODO ----- #
# Formatted print
# Variables
# Custom function ?
# Formatter ?
`;

const removeSpan = /<\/?span[^>]*>/g;

/** Code editor with multiple editable lines */
function newMultilineEditor() {
  let code = defaultCode.repeat(100); // Displayed code
  let linePos = 0; // Cursor line pos
  let charPos = 0; // Cursor char pos
  let offsets = []; // Lines offset in code
  let lengths = []; // Lines length
  let nbLines = 0; // Number of line displayed

  // Reactive state shared with the editor view
  const lines = writable([]);
  const pos = writable({ linePos, charPos });

  /** Refresh the state of the editor */
  function refreshState() {
    const split = code.split('\n');
    let offset = 0;
    offsets = split.map((line, idx) => {
      offset += line.length;
      return offset - line.length + idx;
    });
    lengths = split.map(line => line.length);
    nbLines = split.length;
    let highlight = crate().highlight_batch(code);
    let executed = crate().execute_batch(code);
    let wrap = lengths.map((length, i) => ({
      contentLen: length,
      content: highlight[i] ?? '',
      sideLen: (executed[i] ?? '').replace(removeSpan, '').length,
      side: executed[i] ?? '',
    }));
    lines.set(wrap);
    pos.set({ linePos, charPos });
  }

  refreshState();

  /** Find cursor index in code */
  function cursorIndexInCode() {
    return offsets[linePos] + Math.min(charPos, lengths[linePos]);
  }

  /** Insert char at index in code */
  function insertChar(index, char) {
    code = code.slice(0, index) + char + code.slice(index);
  }

  /** Remove char at index in code */
  function removeChar(index) {
    code = code.slice(0, index - 1) + code.slice(index);
  }

  /** Ensure current char pos is coherent */
  function clampCharPos() {
    // When moving vertically the char pos may be in a different pos than the cursor, this allow
    // to keep char pos during movement but must be fixed when moving the cursors or editing code */
    charPos = Math.min(charPos, lengths[linePos]);
  }

  return {
    lines,
    pos,
    onPos(newLinePos, newCharPos) {
      linePos = newLinePos;
      charPos = newCharPos;
      pos.set({ linePos, charPos });
    },
    onInput(e) {
      if (e.ctrlKey || e.metaKey || e.altKey) {
        return;
      }
      const key = e.key;
      if (isArrowKey(key)) {
        if (key == 'ArrowLeft') {
          clampCharPos();
          if (charPos > 0) {
            charPos--;
          } else if (linePos > 0) {
            linePos--;
            charPos = lengths[linePos];
          }
        } else if (key == 'ArrowRight') {
          clampCharPos();
          if (charPos + 1 < lengths[linePos]) {
            charPos++;
          } else if (linePos + 1 < nbLines) {
            linePos++;
            charPos = 0;
          }
        } else if (key == 'ArrowUp') {
          if (linePos > 0) {
            linePos--;
          }
        } else if (key == 'ArrowDown') {
          if (linePos + 1 < nbLines) {
            linePos++;
          }
        }
      } else if (isEditKey(key)) {
        clampCharPos();
        if (key == 'Tab') {
          insertChar(cursorIndexInCode(), '\t');
        } else if (key == 'Backspace') {
          removeChar(cursorIndexInCode());
          if (charPos > 0) {
            charPos--;
          } else if (linePos > 0) {
            linePos--;
            charPos = lengths[linePos];
          }
        } else if (key == 'Delete') {
          removeChar(cursorIndexInCode() + 1);
        } else if (key == 'Enter') {
          insertChar(cursorIndexInCode(), '\n');
          charPos = 0;
          linePos++;
        }
      } else if (isInputKey(key)) {
        clampCharPos();
        insertChar(cursorIndexInCode(), key);
        charPos++;
      } else {
        return;
      }
      e.preventDefault();
      refreshState();
    },
  };
}

export { newMultilineEditor };
