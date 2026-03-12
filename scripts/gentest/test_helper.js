
function getScrollBarWidth() {
  let el = document.createElement("div");
  el.style.cssText = "overflow:scroll; visibility:hidden; position:absolute;";
  document.body.appendChild(el);
  let width = el.offsetWidth - el.clientWidth;
  el.remove();
  return width;
}

class TrackSizingParser {
  static INITIAL_CHAR_REGEX = /[a-z-A-Z0-9]/;
  static TOKEN_CHAR_REGEX = /[-\.a-z-A-Z0-9%]/;

  constructor(input, options = { allowFrUnits: true }) {
    this.input = input;
    this.index = 0;
    this.options = options;
  }

  parseList() {
    return this._parseItemList(' ', null);
  }

  parseSingleItem() {
    return this.parseItem();
  }

  _parseItemList(separator, terminator = null) {
    if (!separator) throw new Error('No terminator passed');
    let tokenList = [];
    // console.debug('Parse List', this.index, this.input.slice(this.index));

    while (this.index < this.input.length) {
      const char = this.input[this.index];
      // console.debug(this.index, char);

      // Skip whitespace
      if (char === ' ') { this.index++; continue; }

      if (TrackSizingParser.INITIAL_CHAR_REGEX.test(char)) {
        const token = this._parseItem();
        tokenList.push(token);

        const nextChar = this.input[this.index];
        if ((terminator && nextChar === terminator) || !terminator && !nextChar) {
          return tokenList;
        } else {
          this.index++;
          continue;
        }
      }

      throw new Error(`Invalid start of token ${char}`);
    }
  }

  _parseItem() {
    let token = '';
    // console.debug('Parse Item', this.index, this.input.slice(this.index));

    while (this.index < this.input.length) {
      const char = this.input[this.index];
      // console.debug(this.index, char);

      if (TrackSizingParser.TOKEN_CHAR_REGEX.test(char)) {
        token += char;
        this.index++;
        continue;
      }

      if (char === '(') {
        this.index++;
        const args = this._parseItemList(',', ')');
        this.index++;
        return { kind: 'function', name: token, arguments: args };
      }

      return { kind: 'scalar', ...this._parseScalarItem(token) };
    }
    return { kind: 'scalar', ...this._parseScalarItem(token) };

  }

  _parseScalarItem(item) {
    const res = parseRepetition(item) || parseDimension(item, { allowFrUnits: this.options.allowFrUnits });
    if (!res) throw new Error(`Invalid scalar grid track sizing function ${item}`);
    return res;
  }

}

function parseViewportConstraint(e) {
  if (e.parentNode.classList.contains('viewport')) {
    return {
      width: parseDimension(e.parentNode.style.width || 'max-content'),
      height: parseDimension(e.parentNode.style.height || 'max-content'),
    }
  } else {
    return {
      width: { unit: 'max-content' },
      height: { unit: 'max-content' },
    }
  }
}

function parseRepetition(input) {
  if (input === "auto-fill") return { unit: 'auto-fill' };
  if (input === "auto-fit") return { unit: 'auto-fit' };
  if (/^[0-9]*$/.test(input)) return { 'unit': 'integer', value: parseInt(input, 10) };
  return undefined;
}

function parseDimension(input, options = { allowFrUnits: false }) {
  if (options.allowFrUnits && input.endsWith('fr')) return { unit: 'fraction', value: parseFloat(input.replace('fr', '')) };
  if (input.endsWith('px')) return { unit: 'px', value: parseFloat(input.replace('px', '')) };
  if (input.endsWith('%')) return { unit: 'percent', value: parseFloat(input.replace('%', '')) / 100 };
  if (input === 'auto') return { unit: 'auto' };
  if (input === 'min-content') return { unit: 'min-content' };
  if (input === 'max-content') return { unit: 'max-content' };
  return undefined;
}

function parseNumber(input) {
  if (input === '' || isNaN(input)) return undefined;
  return Number(input);
}

function parseRatio(input) {
  if (!input) return undefined;

  if (input.includes('/')) {
    let [width, height] = input.split("/").map(part => parseFloat(part.trim()));
    if (!width || width < 0 || !height || height <= 0) return undefined;
    return width / height;
  }

  let ratio = parseFloat(input);
  if (!ratio || ratio < 0) return undefined;
  return ratio;
}

function parseEnum(input) {
  if (input) return input;
  return undefined;
}

function parseEdges(edges) {
  const left = parseDimension(edges.left);
  const right = parseDimension(edges.right);
  const top = parseDimension(edges.top);
  const bottom = parseDimension(edges.bottom);

  if (!left && !right && !top && !bottom) return undefined;
  return { left, right, top, bottom };
}

function parseSize(size) {
  const width = parseDimension(size.width);
  const height = parseDimension(size.height);

  if (!width && !height) return undefined;
  return { width, height };
}

function parseGaps(style) {
  if (style.gap) {
    const gaps = style.gap.trim().split(/\s+/).map(part => parseDimension(part));
    return { row: gaps[0], column: gaps[1] ?? gaps[0] };
  }
  if (style.rowGap || style.columnGap) {
    return { row: parseDimension(style.rowGap), column: parseDimension(style.columnGap) };
  }
  return undefined;
}


function parseGridTrackDefinitions(input) {
  if (input === '') return undefined;
  return new TrackSizingParser(input).parseList();
}

function parseGridAutoFlow(input) {
  if (!/column/.test(input) && !/row/.test(input) && !/dense/.test(input)) return undefined;
  const direction = /column/.test(input) ? 'column' : 'row';
  const algorithm = /dense/.test(input) ? 'dense' : 'sparse';
  return { direction, algorithm };
}

function parseGridPosition(input) {
  if (input === 'auto') return { kind: 'auto' };
  if (/^span +\d+$/.test(input)) return { kind: 'span', value: parseInt(input.replace(/[^\d]/g, ''), 10) };
  if (/^-?\d+$/.test(input)) return { kind: 'line', value: parseInt(input, 10) };
  return undefined;
}

function describeElement(e) {

  // Get precise, unrounded dimensions for the current element and it's parent
  let boundingRect = e.getBoundingClientRect();
  let parentBoundingRect = e.parentNode.getBoundingClientRect();

  const computedStyle = getComputedStyle(e);

  return {
    style: {
      display: parseEnum(e.style.display),
      boxSizing: parseEnum(computedStyle.boxSizing),

      position: parseEnum(e.style.position),
      direction: parseEnum(computedStyle.direction),

      writingMode: parseEnum(e.style.writingMode),

      cssFloat: parseEnum(e.style.cssFloat),
      clear: parseEnum(e.style.clear),

      textAlign: parseEnum(e.style.textAlign),

      flexDirection: parseEnum(e.style.flexDirection),
      flexWrap: parseEnum(e.style.flexWrap),
      overflowX: parseEnum(e.style.overflowX),
      overflowY: parseEnum(e.style.overflowY),
      scrollbarWidth: getScrollBarWidth(),

      alignItems: parseEnum(e.style.alignItems),
      alignSelf: parseEnum(e.style.alignSelf),
      justifyItems: parseEnum(e.style.justifyItems),
      justifySelf: parseEnum(e.style.justifySelf),

      alignContent: parseEnum(e.style.alignContent),
      justifyContent: parseEnum(e.style.justifyContent),

      flexGrow: parseNumber(e.style.flexGrow),
      flexShrink: parseNumber(e.style.flexShrink),
      flexBasis: parseDimension(e.style.flexBasis),

      gridTemplateRows: parseGridTrackDefinitions(e.style.gridTemplateRows),
      gridTemplateColumns: parseGridTrackDefinitions(e.style.gridTemplateColumns),
      gridAutoRows: parseGridTrackDefinitions(e.style.gridAutoRows),
      gridAutoColumns: parseGridTrackDefinitions(e.style.gridAutoColumns),
      gridAutoFlow: parseGridAutoFlow(e.style.gridAutoFlow),

      gridRowStart: parseGridPosition(e.style.gridRowStart),
      gridRowEnd: parseGridPosition(e.style.gridRowEnd),
      gridColumnStart: parseGridPosition(e.style.gridColumnStart),
      gridColumnEnd: parseGridPosition(e.style.gridColumnEnd),

      gap: parseGaps(e.style),

      size: parseSize({ width: e.style.width, height: e.style.height }),
      minSize: parseSize({ width: e.style.minWidth, height: e.style.minHeight }),
      maxSize: parseSize({ width: e.style.maxWidth, height: e.style.maxHeight }),
      aspectRatio: parseRatio(e.style.aspectRatio),

      margin: parseEdges({
        left: e.style.marginLeft,
        right: e.style.marginRight,
        top: e.style.marginTop,
        bottom: e.style.marginBottom,
      }),

      padding: parseEdges({
        left: e.style.paddingLeft,
        right: e.style.paddingRight,
        top: e.style.paddingTop,
        bottom: e.style.paddingBottom,
      }),

      border: parseEdges({
        left: e.style.borderLeftWidth,
        right: e.style.borderRightWidth,
        top: e.style.borderTopWidth,
        bottom: e.style.borderBottomWidth,
      }),

      inset: parseEdges({
        left: e.style.left,
        right: e.style.right,
        top: e.style.top,
        bottom: e.style.bottom,
      }),
    },

    // The textContent is used for generating intrinsic sizing measure funcs
    // So we're only interested in the text content of leaf nodes
    textContent: e.childElementCount === 0 && e.textContent.length && e.textContent !== "\n" ? e.textContent : undefined,

    // The layout of the node in full precision (floating-point)
    unroundedLayout: {
      width: boundingRect.width,
      height: boundingRect.height,
      x: boundingRect.x - parentBoundingRect.x,
      y: boundingRect.y - parentBoundingRect.y,
      scrollWidth: e.scrollWidth,
      scrollHeight: e.scrollHeight,
      clientWidth: e.clientWidth,
      clientHeight: e.clientHeight,
    },

    // The naively rounded layout of the node. This is equivalent to calling Math.round() on
    // each value in the unrounded layout individually
    naivelyRoundedLayout: {
      width: e.offsetWidth,
      height: e.offsetHeight,
      x: e.offsetLeft + e.parentNode.clientLeft,
      y: e.offsetTop + e.parentNode.clientTop,
      scrollWidth: e.scrollWidth,
      scrollHeight: e.scrollHeight,
      clientWidth: e.clientWidth,
      clientHeight: e.clientHeight,
    },

    // The naive rounding can result in 1px gaps in the layout, so Taffy uses a smarter algorithm to avoid this.
    // Chrome also uses a smarter algorithm, but it doesn't expose the output of that rounding.
    // So we just emulate Taffy's computation here.
    smartRoundedLayout: {
      width: Math.round(boundingRect.right) - Math.round(boundingRect.left),
      height: Math.round(boundingRect.bottom) - Math.round(boundingRect.top),
      x: Math.round(boundingRect.x - parentBoundingRect.x),
      y: Math.round(boundingRect.y - parentBoundingRect.y),
      scrollWidth: e.scrollWidth,
      scrollHeight: e.scrollHeight,
      clientWidth: e.clientWidth,
      clientHeight: e.clientHeight,
    },

    // Whether the test should enable rounding
    useRounding: e.getAttribute("data-test-rounding") !== "false",

    viewport: parseViewportConstraint(e),

    children: Array.from(e.children).map(c => describeElement(c)),
  };
}

function getTestData() {
  document.body.className = "border-box ltr";
  const borderBoxLtrData = describeElement(document.getElementById('test-root'));
  document.body.className = "content-box ltr";
  const contentBoxLtrData = describeElement(document.getElementById('test-root'));
  document.body.className = "border-box rtl";
  const borderBoxRtlData = describeElement(document.getElementById('test-root'));
  document.body.className = "content-box rtl";
  const contentBoxRtlData = describeElement(document.getElementById('test-root'));

  return JSON.stringify({ borderBoxLtrData, contentBoxLtrData, borderBoxRtlData, contentBoxRtlData });
}

// Useful when developing this script. Logs the parsed style to the console when any test fixture is opened in a browser.
window.onload = function () {
  try {
    console.log(describeElement(document.getElementById('test-root')));
  } catch (e) {
    console.error(e);
  }
};
