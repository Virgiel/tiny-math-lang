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

const REG_NL = /\n/g;

/** Generate gutter content from editor lines */
function linesToGutterContent(lines) {
  console.time('gutter');
  let result = lines
    .map((line, idx) =>
      formatLineNb(idx, lines.length, (line.match(REG_NL) || []).length + 1)
    )
    .join('');
  console.timeEnd('gutter');
  return result;
}

export { linesToGutterContent };
