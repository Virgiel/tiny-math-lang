/** Find the text offset of this position in a text node */
function offsetInTextAt(text, x, y) {
  const range = document.createRange();
  const nbChar = text.textContent.length;
  // Check if char
  for (let i = 0; i < nbChar; i++) {
    range.setStart(text, i);
    range.setEnd(text, i + 1);
    const r = range.getBoundingClientRect();
    if (r.left < x && r.right > x && r.top < y && r.bottom > y) {
      return i;
    }
  }
  return -1;
}

/** Find the text offset of this position in any node */
function offsetInNodeAt(node, x, y) {
  if (node.nodeName === '#text') {
    // In text search
    return offsetInTextAt(node, x, y);
  } else {
    // In child search
    let offset = 0;
    for (const child of node.childNodes) {
      const result = offsetInNodeAt(child, x, y);
      if (result >= 0) {
        return offset + result;
      } else {
        // Remember searched child
        offset += child.textContent.length;
      }
    }
  }
  return -1;
}

/** Find the bounding rect at offset in a text node */
function rectInTextAt(text, offset) {
  const range = document.createRange();
  const nbChar = text.textContent.length;
  // CHeck contains
  if (offset <= nbChar) {
    range.setStart(text, offset);
    range.setEnd(text, offset);
    return range.getBoundingClientRect();
  } else {
    return null;
  }
}

/** Find the bounding rect at offset in a any node */
function rectInNodeAt(node, offset) {
  if (node.nodeName === '#text') {
    // In text search
    return rectInTextAt(node, offset);
  } else {
    // In child search
    for (const child of node.childNodes) {
      const result = rectInNodeAt(child, offset);
      if (result) {
        return result;
      } else {
        // Remember searched child
        offset -= child.textContent.length;
      }
    }
  }
  return null;
}

export { offsetInNodeAt, rectInNodeAt };
