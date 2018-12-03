function define_element_prop(property, getter) {
  if (!(property in Element.prototype)) {
    Object.defineProperty(Element.prototype, property, {
      get: function() {
        return getter(this);
      }
    });
  }
}

define_element_prop("__stretch_description__", (e) => {
  return JSON.stringify(describeElement(e));
});

function parseDimension(input) {
  if (input.endsWith("px")) {
    return {
      unit: 'points', 
      value: Number(input.replace('px',''))
    };
  } else if (input.endsWith("%")) {
    return {
      unit: 'percent', 
      value: Number(input.replace('%','')) / 100
    };
  } else {
    return input == "auto" ? {unit: "auto"} : undefined;
  }
}

function parseNumber(input) {
  if (input === "" || isNaN(input)) {
    return undefined;
  } else {
    return Number(input);
  }
}

function parseEnum(input) {
  if (input) {
    return input;
  } else {
    return undefined;
  }
}

function parseEdges(edges) {
  var start = parseDimension(edges.start);
  var end = parseDimension(edges.end);
  var top = parseDimension(edges.top);
  var bottom = parseDimension(edges.bottom);
  
  if (start === undefined && end === undefined && top === undefined && bottom === undefined) {
    return undefined;
  }

  return {
    start: start,
    end: end,
    top: top,
    bottom: bottom
  };
}

function describeElement(e) {
  return {
    style: {
      position: parseEnum(e.style.position),
      direction: parseEnum(e.style.direction),
      flexDirection: parseEnum(e.style.flexDirection),

      flexWrap: parseEnum(e.style.flexWrap),
      overflow: parseEnum(e.style.overflow),

      alignItems: parseEnum(e.style.alignItems),
      alignSelf: parseEnum(e.style.alignSelf),
      alignContent: parseEnum(e.style.alignContent),
      
      justifyContent: parseEnum(e.style.justifyContent),

      flexGrow: parseNumber(e.style.flexGrow),
      flexShrink: parseNumber(e.style.flexShrink),
      flexBasis: parseDimension(e.style.flexBasis),

      width: parseDimension(e.style.width),
      minWidth: parseDimension(e.style.minWidth),
      maxWidth: parseDimension(e.style.maxWidth),

      height: parseDimension(e.style.height),
      minHeight: parseDimension(e.style.minHeight),
      maxHeight: parseDimension(e.style.maxHeight),

      margin: parseEdges({
        start: e.style.marginLeft,
        end: e.style.marginRight,
        top: e.style.marginTop,
        bottom: e.style.marginBottom,
      }),

      padding: parseEdges({
        start: e.style.paddingLeft,
        end: e.style.paddingRight,
        top: e.style.paddingTop,
        bottom: e.style.paddingBottom,
      }),

      border: parseEdges({
        start: e.style.borderLeftWidth,
        end: e.style.borderRightWidth,
        top: e.style.borderTopWidth,
        bottom: e.style.borderBottomWidth,
      }),

      start: parseDimension(e.style.left),
      end: parseDimension(e.style.right),
      top: parseDimension(e.style.top),
      bottom: parseDimension(e.style.bottom),
    },

    layout: {
      width: e.offsetWidth,
      height: e.offsetHeight,
      x: e.offsetLeft + e.parentNode.clientLeft,
      y: e.offsetTop + e.parentNode.clientTop,
    },

    children: Array.from(e.children).map(c => describeElement(c)),
  }
}
