/** Iterate on each Dom tree children using pre-order traversal */
function treeIterator(root) {
  const queue = [];

  if (root.firstChild) queue.push(root.firstChild);

  return {
    next: function () {
      let el = queue.pop();

      if (el) {
        if (el.nextSibling) queue.push(el.nextSibling);
        if (el.firstChild) queue.push(el.firstChild);
        return { value: el, done: false };
      } else {
        return { value: null, done: true };
      }
    },
    [Symbol.iterator]: function () {
      return this;
    },
  };
}

/** Save selection in the given node */
function saveSelection(node) {
  const s = window.getSelection();
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
  let s = window.getSelection();
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
