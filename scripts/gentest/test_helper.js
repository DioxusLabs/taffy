
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
      if (char === ' ') { this.index++; continue;}

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

      throw new Error (`Invalid start of token ${char}`);
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
    const res = parseDimension(item, { allowFrUnits: this.options.allowFrUnits });
    if (!res) throw new Error(`Invalid scalar grid track sizing function ${item}`);
    return res;
  }

}

function parseDimension(input, options = { allowFrUnits: false }) {
  if (options.allowFrUnits && input.endsWith('fr')) return { unit: 'fraction', value: parseFloat(input.replace('fr','')) };
  if (input.endsWith('px')) return { unit: 'points',   value: parseFloat(input.replace('px','')) };
  if (input.endsWith('%'))  return { unit: 'percent',  value: parseFloat(input.replace('%','')) / 100 };
  if (input === 'auto')     return { unit: 'auto' };
  if (input === 'min-content')     return { unit: 'min-content' };
  if (input === 'max-content')     return { unit: 'max-content' };
  return undefined;
}

function parseNumber(input) {
  if (input === '' || isNaN(input)) return undefined;
  return Number(input);
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
    return { row: parseDimension(style.rowGap), column: parseDimension(style.columnGap) }
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
  if (input === 'auto') return { kind: 'auto' }
  if (/^span +\d+$/.test(input)) return { kind: 'span', value: parseInt(input.replace(/[^\d]/g, ''), 10)}
  if (/^-?\d+$/.test(input)) return { kind: 'track', value: parseInt(input, 10)}
  return undefined;
}

function describeElement(e) {

  return {
    style: {
      display: parseEnum(e.style.display),

      positionType: parseEnum(e.style.position),
      direction: parseEnum(e.style.direction),
      flexDirection: parseEnum(e.style.flexDirection),

      flexWrap: parseEnum(e.style.flexWrap),
      overflow: parseEnum(e.style.overflow),

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

      size: parseSize({width: e.style.width, height: e.style.height}),
      minSize: parseSize({width: e.style.minWidth, height: e.style.minHeight}),
      maxSize: parseSize({width: e.style.maxWidth, height: e.style.maxHeight}),

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

      position: parseEdges({
        left: e.style.left,
        right: e.style.right,
        top: e.style.top,
        bottom: e.style.bottom,
      }),
    },

    // The textContent is used for generating intrinsic sizing measure funcs
    // So we're only interested in the text content of leaf nodes
    textContent: e.childElementCount === 0 && e.textContent.length && e.textContent !== "\n" ? e.textContent : undefined,

    layout: {
      width: e.offsetWidth,
      height: e.offsetHeight,
      x: e.offsetLeft + e.parentNode.clientLeft,
      y: e.offsetTop + e.parentNode.clientTop,
    },

    children: Array.from(e.children).map(c => describeElement(c)),
  }
}

// Useful when developing this script. Logs the parsed style to the console when any test fixture is opened in a browser.
window.onload = function () {
  try {
    console.log(describeElement(document.getElementById('test-root')));
  } catch (e) {
    console.error(e);
  }
}
