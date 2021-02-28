/** Key event utils */

/** Check if a given key is an arrow key */
function isArrowKey(key) {
  return ['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].indexOf(key) != -1;
}

/** Check if a given key is a printable key */
function isInputKey(key) {
  return key.length == 1;
}

/** Check if a given key is a edit action key */
function isEditKey(key) {
  return ['Tab', 'Backspace', 'Delete', 'Enter'].indexOf(key) != -1;
}

export { isArrowKey, isInputKey, isEditKey };
