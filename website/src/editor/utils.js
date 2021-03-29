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

function debounce(cb, wait) {
  let timeout = 0;
  return (...args) => {
    clearTimeout(timeout);
    timeout = window.setTimeout(() => cb(...args), wait);
  };
}

export { treeIterator, debounce };
