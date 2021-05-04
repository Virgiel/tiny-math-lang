import { treeIterator } from './utils';

/** Save selection in the given node */
function saveSelection(node) {
  const s = getSelection();
  let sel = { start: null, end: null };
  let offset = 0;

  for (const el of treeIterator(node)) {
    if (el === s.anchorNode) {
      sel.start = offset + s.anchorOffset;
      if (sel.end) {
        return sel;
      }
    }
    if (el === s.focusNode) {
      sel.end = offset + s.focusOffset;
      if (sel.start) {
        return sel;
      }
    }
    if (el.nodeType === Node.TEXT_NODE) {
      offset += el.nodeValue.length;
    }
  }

  return { start: 0, end: 0 };
}

/** Restore selection in the given node */
function restoreSelection(node, sel) {
  let s = getSelection();
  let startNode;
  let startOffset;
  let endNode;
  let endOffset;

  let offset = 0;

  for (const el of treeIterator(node)) {
    if (el.nodeType === Node.TEXT_NODE) {
      const len = el.nodeValue.length;
      if (!startNode && offset + len >= sel.start) {
        startNode = el;
        startOffset = sel.start - offset;
      }
      if (!endNode && offset + len >= sel.end) {
        endNode = el;
        endOffset = sel.end - offset;
      }

      if (startNode && endNode) {
        break;
      } else {
        offset += len;
      }
    }
  }

  s.setBaseAndExtent(startNode, startOffset, endNode, endOffset);
}

export { saveSelection, restoreSelection };
