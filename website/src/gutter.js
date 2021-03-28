/** Compute the number char needed to represent the given line number */
function nbCharLineNb(nb) {
  if (nb < 10) {
    return 1;
  } else if (nb < 100) {
    return 2;
  } else if (nb < 1000) {
    return 3;
  } else {
    return 4;
  }
}

/** Format a line number with right alignement and height padding */
function formatLineNb(idx, nbLine, height) {
  const pad = nbCharLineNb(nbLine) - nbCharLineNb(idx);
  return ' '.repeat(pad) + idx + '\n'.repeat(height);
}

/** Generate gutter content from lines heights */
function heightsToGutterContent(heights) {
  let content = '';
  for (let i = 0; i < heights.length; i++) {
    content += formatLineNb(i, heights.length, heights[i]) + '\n';
  }
  return content;
}

export { heightsToGutterContent };
